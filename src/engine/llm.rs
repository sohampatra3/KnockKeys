use crate::models::{AiConfig, AiProvider, ProgressData, Settings};
use anyhow::{anyhow, Context, Result};
use serde_json::json;
use std::env;
use std::time::Duration;

const DEFAULT_GROQ_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const DEFAULT_MODEL: &str = "gpt-oss-120b";

pub fn request_typing_guru_report(
    progress: &ProgressData,
    settings: &Settings,
    ai_config: &AiConfig,
) -> Result<String> {
    let api_key = resolve_api_key(ai_config).ok_or_else(|| {
        anyhow!(
            "Missing API key. Paste it in Typing Guru AI screen or set provider-specific env vars."
        )
    })?;

    let url = resolve_url(ai_config);
    let model = resolve_model(ai_config);

    let system_prompt = "You are Typing Guru, an elite typing coach for software engineers. Analyze metrics deeply and give practical drills. Focus on measurable improvements and coding-specific typing efficiency.";
    let user_prompt = build_prompt(progress, settings);

    let payload = json!({
        "model": model,
        "temperature": 0.3,
        "max_tokens": 900,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}
        ]
    });

    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(8))
        .timeout_read(Duration::from_secs(30))
        .build();

    let response = agent
        .post(&url)
        .set("Authorization", &format!("Bearer {api_key}"))
        .set("Content-Type", "application/json")
        .send_string(&payload.to_string())
        .map_err(|e| anyhow!("API request failed: {e}"))?;

    let value: serde_json::Value =
        serde_json::from_reader(response.into_reader()).context("Failed to parse AI response")?;

    let report = value
        .pointer("/choices/0/message/content")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| anyhow!("AI response was empty or malformed"))?;

    Ok(report.trim().to_string())
}

pub fn local_analytics_report(progress: &ProgressData, settings: &Settings) -> String {
    let total_sessions = progress.session_history.len();
    let total_minutes = progress.total_practice_secs / 60;

    let last10: Vec<_> = progress.session_history.iter().rev().take(10).collect();
    let avg_last10_wpm = if last10.is_empty() {
        0.0
    } else {
        last10.iter().map(|s| s.wpm).sum::<f64>() / last10.len() as f64
    };

    let avg_last10_acc = if last10.is_empty() {
        0.0
    } else {
        last10.iter().map(|s| s.accuracy).sum::<f64>() / last10.len() as f64
    };

    let mut error_keys: Vec<(char, f64, u64)> = progress
        .key_stats
        .iter()
        .filter(|(_, s)| s.attempts >= 5)
        .map(|(k, s)| (*k, s.error_rate() * 100.0, s.attempts))
        .collect();
    error_keys.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut slow_keys: Vec<(char, f64, u64)> = progress
        .key_stats
        .iter()
        .filter(|(_, s)| s.attempts >= 5)
        .map(|(k, s)| (*k, s.avg_time_ms(), s.attempts))
        .collect();
    slow_keys.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let top_errors = format_triplet_keys(&error_keys, true);
    let top_slow = format_triplet_keys(&slow_keys, false);

    let mut lang_rows: Vec<(String, f64, f64, u64)> = progress
        .language_stats
        .iter()
        .map(|(k, v)| (pretty_lang(k), v.avg_wpm(), v.best_wpm, v.sessions))
        .collect();
    lang_rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let lang_summary = if lang_rows.is_empty() {
        "No code-language drill stats yet".to_string()
    } else {
        lang_rows
            .iter()
            .take(5)
            .map(|(k, avg, best, sess)| {
                format!("- {k}: avg {avg:.1} WPM, best {best:.1}, sessions {sess}")
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let drills = improvement_drills(&error_keys, &slow_keys);

    format!(
        "Typing Guru Offline Analytics\n\nOverview\n- Sessions: {total_sessions}\n- Total practice: {total_minutes} min\n- Best WPM: {:.1}\n- Last 10 avg WPM: {avg_last10_wpm:.1}\n- Last 10 avg Accuracy: {avg_last10_acc:.1}%\n- Unlock threshold: {:.0}% acc / {:.0} WPM\n\nWeakest keys (highest error %)\n{top_errors}\n\nSlowest keys (highest avg ms)\n{top_slow}\n\nCode language speed\n{lang_summary}\n\nImprovement tricks\n{drills}\n\nTo enable full AI report: open Typing Guru AI from Home, choose provider, paste API key, then generate.",
        progress.best_wpm,
        settings.unlock_accuracy_threshold,
        settings.unlock_wpm_threshold,
    )
}

fn resolve_api_key(ai_config: &AiConfig) -> Option<String> {
    let key = ai_config.api_key.trim();
    if !key.is_empty() {
        return Some(key.to_string());
    }

    env::var("TYPETRAINER_API_KEY")
        .ok()
        .or_else(|| match ai_config.provider {
            AiProvider::Groq => env::var("GROQ_API_KEY")
                .ok()
                .or_else(|| env::var("TYPETRAINER_GROQ_API_KEY").ok()),
            AiProvider::OpenAICompatible => env::var("OPENAI_API_KEY")
                .ok()
                .or_else(|| env::var("TYPETRAINER_OPENAI_API_KEY").ok()),
        })
}

fn resolve_url(ai_config: &AiConfig) -> String {
    let configured = ai_config.base_url.trim();
    if !configured.is_empty() {
        return configured.to_string();
    }
    match ai_config.provider {
        AiProvider::Groq => {
            env::var("TYPETRAINER_GROQ_URL").unwrap_or_else(|_| DEFAULT_GROQ_URL.to_string())
        }
        AiProvider::OpenAICompatible => env::var("TYPETRAINER_OPENAI_URL")
            .unwrap_or_else(|_| AiProvider::OpenAICompatible.default_base_url().to_string()),
    }
}

fn resolve_model(ai_config: &AiConfig) -> String {
    let configured = ai_config.model.trim();
    if !configured.is_empty() {
        return configured.to_string();
    }
    match ai_config.provider {
        AiProvider::Groq => {
            env::var("TYPETRAINER_GROQ_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string())
        }
        AiProvider::OpenAICompatible => env::var("TYPETRAINER_OPENAI_MODEL")
            .unwrap_or_else(|_| AiProvider::OpenAICompatible.default_model().to_string()),
    }
}

fn build_prompt(progress: &ProgressData, settings: &Settings) -> String {
    let recent: Vec<_> = progress
        .session_history
        .iter()
        .rev()
        .take(12)
        .map(|s| {
            json!({
                "mode": s.mode.title(),
                "wpm": s.wpm,
                "accuracy": s.accuracy,
                "duration_secs": s.duration_secs,
                "mistakes": s.mistakes,
                "problematic_keys": s.problematic_keys,
                "code_language": s.code_language.map(|l| l.title().to_string()),
                "code_topic": s.code_topic.map(|t| t.title().to_string()),
            })
        })
        .collect();

    let key_stats: Vec<_> = progress
        .key_stats
        .iter()
        .filter(|(_, s)| s.attempts >= 5)
        .map(|(k, s)| {
            json!({
                "key": k,
                "attempts": s.attempts,
                "accuracy": s.accuracy() * 100.0,
                "error_rate": s.error_rate() * 100.0,
                "avg_time_ms": s.avg_time_ms(),
            })
        })
        .collect();

    let language_stats: Vec<_> = progress
        .language_stats
        .iter()
        .map(|(k, v)| {
            json!({
                "language": pretty_lang(k),
                "sessions": v.sessions,
                "avg_wpm": v.avg_wpm(),
                "best_wpm": v.best_wpm,
                "total_chars": v.total_chars,
                "total_secs": v.total_secs,
            })
        })
        .collect();

    let metrics_json = json!({
        "total_sessions": progress.session_history.len(),
        "total_practice_secs": progress.total_practice_secs,
        "best_wpm": progress.best_wpm,
        "current_streak": progress.current_streak(),
        "unlocked_chars": progress.unlocked_chars.iter().collect::<String>(),
        "settings": {
            "difficulty": settings.difficulty.title(),
            "unlock_accuracy_threshold": settings.unlock_accuracy_threshold,
            "unlock_wpm_threshold": settings.unlock_wpm_threshold,
            "focus_mode": settings.focus_mode,
        },
        "recent_sessions": recent,
        "key_stats": key_stats,
        "language_stats": language_stats,
    });

    format!(
        "Analyze these typing metrics and return a full coaching report with the exact sections below.\n\nSections required:\n1) Performance Snapshot\n2) Biggest Bottlenecks\n3) 7-Day Improvement Plan\n4) Code-Typing Technique Tips\n5) Next Session Drill Prescription\n6) Mistake Prevention Checklist\n\nTone: direct, practical, typing-guru style. Use concrete targets (WPM/accuracy/time).\n\nMetrics:\n{}",
        metrics_json
    )
}

fn format_triplet_keys(rows: &[(char, f64, u64)], is_percent: bool) -> String {
    if rows.is_empty() {
        return "- Not enough data yet".to_string();
    }
    rows.iter()
        .take(6)
        .map(|(k, v, a)| {
            if is_percent {
                format!("- {}: {:.1}% error over {} attempts", k, v, a)
            } else {
                format!("- {}: {:.0}ms avg over {} attempts", k, v, a)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn improvement_drills(error_keys: &[(char, f64, u64)], slow_keys: &[(char, f64, u64)]) -> String {
    let err_focus: String = error_keys
        .iter()
        .take(3)
        .map(|(k, _, _)| k.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let slow_focus: String = slow_keys
        .iter()
        .take(3)
        .map(|(k, _, _)| k.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let err_part = if err_focus.is_empty() {
        "home-row keys".to_string()
    } else {
        err_focus
    };
    let slow_part = if slow_focus.is_empty() {
        "index-finger travel keys".to_string()
    } else {
        slow_focus
    };

    format!(
        "- Drill A (accuracy): 4 x 90s strict mode on [{}], target >= 97% accuracy.\n- Drill B (speed stability): 5 x 60s free-flow on [{}], keep variance under +/- 4 WPM.\n- Drill C (code symbols): 3 x 75s code-drill mixed with brackets/semicolons, focus on opposite-hand shift.\n- Technique: relax shoulders, keep wrists neutral, return to home row after each reach.\n- Cadence: daily 15 minutes minimum, split into 3 focused blocks.",
        err_part, slow_part
    )
}

fn pretty_lang(key: &str) -> String {
    match key {
        "cpp" => "C++".to_string(),
        "c" => "C".to_string(),
        "rust" => "Rust".to_string(),
        "python" => "Python".to_string(),
        "go" => "Go".to_string(),
        _ => key.to_string(),
    }
}
