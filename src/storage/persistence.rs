use crate::models::{AiConfig, CodeConfig, ProgressData, Settings, TestConfig};
use anyhow::{Context, Result};
use chrono::Local;
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct StoragePaths {
    pub base_dir: PathBuf,
    pub progress_file: PathBuf,
    pub exports_dir: PathBuf,
    pub backups_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedState {
    pub settings: Settings,
    pub progress: ProgressData,
    pub test_config: TestConfig,
    #[serde(default)]
    pub code_config: CodeConfig,
    #[serde(default)]
    pub ai_config: AiConfig,
}

impl Default for PersistedState {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            progress: ProgressData::default(),
            test_config: TestConfig::default(),
            code_config: CodeConfig::default(),
            ai_config: AiConfig::default(),
        }
    }
}

pub fn default_paths() -> Result<StoragePaths> {
    let home = dirs::home_dir().context("Could not resolve home directory")?;
    let base_dir = home.join(".typetrainer");
    let progress_file = base_dir.join("progress.json");
    let exports_dir = base_dir.join("exports");
    let backups_dir = base_dir.join("backups");
    Ok(StoragePaths {
        base_dir,
        progress_file,
        exports_dir,
        backups_dir,
    })
}

pub fn ensure_dirs(paths: &StoragePaths) -> Result<()> {
    fs::create_dir_all(&paths.base_dir).with_context(|| {
        format!(
            "Failed to create storage directory: {}",
            paths.base_dir.display()
        )
    })?;
    fs::create_dir_all(&paths.exports_dir).with_context(|| {
        format!(
            "Failed to create exports directory: {}",
            paths.exports_dir.display()
        )
    })?;
    fs::create_dir_all(&paths.backups_dir).with_context(|| {
        format!(
            "Failed to create backups directory: {}",
            paths.backups_dir.display()
        )
    })?;
    Ok(())
}

pub fn load_state(paths: &StoragePaths) -> Result<PersistedState> {
    ensure_dirs(paths)?;
    if !paths.progress_file.exists() {
        return Ok(PersistedState::default());
    }

    let text = fs::read_to_string(&paths.progress_file)
        .with_context(|| format!("Failed reading {}", paths.progress_file.display()))?;

    let mut state: PersistedState =
        serde_json::from_str(&text).context("Failed to parse progress JSON")?;

    if state.progress.unlocked_chars.is_empty() {
        state.progress.unlocked_chars = vec!['f', 'j', 'd', 'k', 's', 'l', 'a', ';'];
    }

    Ok(state)
}

pub fn save_state(paths: &StoragePaths, state: &PersistedState) -> Result<()> {
    ensure_dirs(paths)?;
    let mut state = state.clone();
    state.progress.updated_at = Local::now();

    let serialized = serde_json::to_string_pretty(&state).context("Failed to serialize state")?;
    let tmp_path = paths.progress_file.with_extension("json.tmp");
    let mut file = fs::File::create(&tmp_path)
        .with_context(|| format!("Failed creating {}", tmp_path.display()))?;
    file.write_all(serialized.as_bytes())
        .with_context(|| format!("Failed writing {}", tmp_path.display()))?;
    file.flush()
        .with_context(|| format!("Failed flushing {}", tmp_path.display()))?;
    fs::rename(&tmp_path, &paths.progress_file).with_context(|| {
        format!(
            "Failed moving {} -> {}",
            tmp_path.display(),
            paths.progress_file.display()
        )
    })?;

    Ok(())
}

pub fn export_sessions_csv(
    paths: &StoragePaths,
    state: &PersistedState,
    out: Option<&Path>,
) -> Result<PathBuf> {
    ensure_dirs(paths)?;
    let path = out.map(Path::to_path_buf).unwrap_or_else(|| {
        paths.exports_dir.join(format!(
            "sessions-{}.csv",
            Local::now().format("%Y%m%d-%H%M%S")
        ))
    });

    let mut writer = Writer::from_path(&path)
        .with_context(|| format!("Failed creating CSV at {}", path.display()))?;

    writer.write_record([
        "started_at",
        "mode",
        "code_language",
        "code_topic",
        "duration_secs",
        "wpm",
        "accuracy",
        "mistakes",
        "chars_typed",
        "problematic_keys",
    ])?;

    for s in &state.progress.session_history {
        writer.write_record([
            s.started_at.to_rfc3339(),
            s.mode.title().to_string(),
            s.code_language
                .map(|l| l.title().to_string())
                .unwrap_or_default(),
            s.code_topic
                .map(|t| t.title().to_string())
                .unwrap_or_default(),
            s.duration_secs.to_string(),
            format!("{:.2}", s.wpm),
            format!("{:.2}", s.accuracy),
            s.mistakes.to_string(),
            s.chars_typed.to_string(),
            s.problematic_keys.iter().collect::<String>(),
        ])?;
    }

    writer.flush()?;
    Ok(path)
}

pub fn backup_progress(
    paths: &StoragePaths,
    state: &PersistedState,
    out: Option<&Path>,
) -> Result<PathBuf> {
    ensure_dirs(paths)?;
    let path = out.map(Path::to_path_buf).unwrap_or_else(|| {
        paths.backups_dir.join(format!(
            "progress-{}.json",
            Local::now().format("%Y%m%d-%H%M%S")
        ))
    });

    let serialized = serde_json::to_string_pretty(state).context("Failed to serialize backup")?;
    fs::write(&path, serialized)
        .with_context(|| format!("Failed writing backup file {}", path.display()))?;
    Ok(path)
}

pub fn import_progress(paths: &StoragePaths, path: &Path) -> Result<PersistedState> {
    ensure_dirs(paths)?;
    let text = fs::read_to_string(path)
        .with_context(|| format!("Failed reading import file {}", path.display()))?;
    let state: PersistedState =
        serde_json::from_str(&text).context("Import JSON is invalid or incompatible")?;

    save_state(paths, &state)?;
    Ok(state)
}

pub fn latest_backup(paths: &StoragePaths) -> Result<Option<PathBuf>> {
    ensure_dirs(paths)?;

    let mut files: Vec<(PathBuf, std::time::SystemTime)> = vec![];
    for entry in fs::read_dir(&paths.backups_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let modified = entry
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        files.push((path, modified));
    }

    files.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(files.first().map(|(p, _)| p.clone()))
}
