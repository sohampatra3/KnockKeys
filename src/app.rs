use crate::engine::{code, lesson, metrics};
use crate::input::keys::{translate, Action};
use crate::models::{
    CodeConfig, HomeItem, LessonItem, PauseItem, ProgressData, Screen, SessionMode, SessionResult,
    SessionSummary, Settings, SettingsItem, TestConfig, TestDuration, TestTextType,
    TrainingDifficulty, TypedEntry,
};
use crate::storage::persistence::{
    backup_progress, export_sessions_csv, import_progress, latest_backup, save_state,
    PersistedState, StoragePaths,
};
use chrono::Local;
use crossterm::event::KeyEvent;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum SessionPlan {
    AdaptiveTraining,
    Lesson {
        keys: Vec<char>,
        label: String,
    },
    WeakDrill,
    TypingTest(TestConfig),
    CodeDrill {
        language: crate::models::LanguageMode,
        topic: crate::models::CodeTopic,
    },
}

#[derive(Debug, Clone)]
pub struct ActiveSession {
    pub plan: SessionPlan,
    pub mode: SessionMode,
    pub label: String,
    pub target: Vec<char>,
    pub typed: Vec<TypedEntry>,
    pub cursor: usize,
    pub started_at: Instant,
    pub started_wallclock: chrono::DateTime<Local>,
    pub paused_total: Duration,
    pub paused_since: Option<Instant>,
    pub last_key_at: Instant,
    pub time_limit: Option<Duration>,
    pub blocked_errors: u64,
    pub misses_by_key: HashMap<char, u64>,
    pub wpm_samples: Vec<f64>,
    pub last_sample_sec: u64,
}

impl ActiveSession {
    pub fn next_target_char(&self) -> Option<char> {
        self.target.get(self.cursor).copied()
    }

    pub fn elapsed(&self, now: Instant) -> Duration {
        let mut elapsed = now.saturating_duration_since(self.started_at);
        elapsed = elapsed.saturating_sub(self.paused_total);
        if let Some(paused_since) = self.paused_since {
            elapsed = elapsed.saturating_sub(now.saturating_duration_since(paused_since));
        }
        elapsed
    }

    pub fn remaining_secs(&self, now: Instant) -> Option<u64> {
        self.time_limit.map(|limit| {
            let elapsed = self.elapsed(now);
            limit.saturating_sub(elapsed).as_secs()
        })
    }

    pub fn is_paused(&self) -> bool {
        self.paused_since.is_some()
    }
}

pub struct App {
    pub running: bool,
    pub screen: Screen,
    pub home_index: usize,
    pub lesson_index: usize,
    pub settings_index: usize,
    pub pause_index: usize,
    pub test_config_index: usize,
    pub code_config_index: usize,
    pub settings: Settings,
    pub progress: ProgressData,
    pub test_config: TestConfig,
    pub code_config: CodeConfig,
    pub active_session: Option<ActiveSession>,
    pub summary: Option<SessionSummary>,
    pub notification: Option<(String, Instant)>,
    pub storage_paths: StoragePaths,
    pub paused_from: Screen,
    seed_counter: u64,
}

impl App {
    pub fn new(state: PersistedState, storage_paths: StoragePaths) -> Self {
        let seed_counter =
            state.progress.total_chars_typed + state.progress.session_history.len() as u64;
        Self {
            running: true,
            screen: Screen::Home,
            home_index: 0,
            lesson_index: 0,
            settings_index: 0,
            pause_index: 0,
            test_config_index: 0,
            code_config_index: 0,
            settings: state.settings,
            progress: state.progress,
            test_config: state.test_config,
            code_config: state.code_config,
            active_session: None,
            summary: None,
            notification: None,
            storage_paths,
            paused_from: Screen::Training,
            seed_counter,
        }
    }

    fn next_seed(&mut self) -> u64 {
        self.seed_counter = self.seed_counter.wrapping_add(1);
        self.seed_counter.wrapping_mul(7919)
    }

    pub fn to_persisted_state(&self) -> PersistedState {
        PersistedState {
            settings: self.settings.clone(),
            progress: self.progress.clone(),
            test_config: self.test_config.clone(),
            code_config: self.code_config.clone(),
        }
    }

    fn save(&mut self) {
        let state = self.to_persisted_state();
        if let Err(err) = save_state(&self.storage_paths, &state) {
            self.set_notice(format!("Save failed: {err}"));
        }
    }

    fn set_notice(&mut self, msg: impl Into<String>) {
        self.notification = Some((msg.into(), Instant::now()));
    }

    pub fn today_goal_progress(&self) -> (u32, u32) {
        let today = ProgressData::today_key();
        let done = self
            .progress
            .daily_minutes
            .get(&today)
            .copied()
            .unwrap_or(0);
        (done, self.settings.daily_goal_minutes)
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        let action = translate(key, self.screen);
        self.handle_action(action);
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => {
                if self.screen == Screen::Training || self.screen == Screen::TypingTest {
                    self.pause_current();
                } else {
                    self.running = false;
                    self.save();
                }
            }
            Action::Up => self.navigate_up(),
            Action::Down => self.navigate_down(),
            Action::Left => self.adjust_current(false),
            Action::Right => self.adjust_current(true),
            Action::Select => self.select_current(),
            Action::Back => self.go_back(),
            Action::Pause => self.pause_current(),
            Action::Backspace => self.handle_backspace(),
            Action::Char(c) => self.handle_char(c),
            Action::None => {}
        }
    }

    fn navigate_up(&mut self) {
        match self.screen {
            Screen::Home => {
                self.home_index = if self.home_index == 0 {
                    HomeItem::ALL.len() - 1
                } else {
                    self.home_index - 1
                }
            }
            Screen::Lessons => {
                self.lesson_index = if self.lesson_index == 0 {
                    LessonItem::ALL.len() - 1
                } else {
                    self.lesson_index - 1
                }
            }
            Screen::Settings => {
                self.settings_index = if self.settings_index == 0 {
                    SettingsItem::ALL.len() - 1
                } else {
                    self.settings_index - 1
                }
            }
            Screen::Pause => {
                self.pause_index = if self.pause_index == 0 {
                    PauseItem::ALL.len() - 1
                } else {
                    self.pause_index - 1
                }
            }
            Screen::TypingTestConfig => {
                self.test_config_index = if self.test_config_index == 0 {
                    3
                } else {
                    self.test_config_index - 1
                }
            }
            Screen::CodeConfig => {
                self.code_config_index = if self.code_config_index == 0 {
                    3
                } else {
                    self.code_config_index - 1
                }
            }
            _ => {}
        }
    }

    fn navigate_down(&mut self) {
        match self.screen {
            Screen::Home => self.home_index = (self.home_index + 1) % HomeItem::ALL.len(),
            Screen::Lessons => self.lesson_index = (self.lesson_index + 1) % LessonItem::ALL.len(),
            Screen::Settings => {
                self.settings_index = (self.settings_index + 1) % SettingsItem::ALL.len()
            }
            Screen::Pause => self.pause_index = (self.pause_index + 1) % PauseItem::ALL.len(),
            Screen::TypingTestConfig => self.test_config_index = (self.test_config_index + 1) % 4,
            Screen::CodeConfig => self.code_config_index = (self.code_config_index + 1) % 4,
            _ => {}
        }
    }

    fn adjust_current(&mut self, forward: bool) {
        match self.screen {
            Screen::TypingTestConfig => self.adjust_test_config(forward),
            Screen::CodeConfig => self.adjust_code_config(forward),
            Screen::Settings => self.adjust_setting(forward),
            _ => {}
        }
    }

    fn adjust_test_config(&mut self, forward: bool) {
        match self.test_config_index {
            0 => {
                self.test_config.duration = cycle_duration(self.test_config.duration, forward);
                self.save();
            }
            1 => {
                self.test_config.text_type = cycle_text_type(self.test_config.text_type, forward);
                self.save();
            }
            _ => {}
        }
    }

    fn adjust_code_config(&mut self, forward: bool) {
        match self.code_config_index {
            0 => {
                self.code_config.language = cycle_language(self.code_config.language, forward);
                self.save();
            }
            1 => {
                self.code_config.topic = cycle_code_topic(self.code_config.topic, forward);
                self.save();
            }
            _ => {}
        }
    }

    fn adjust_setting(&mut self, forward: bool) {
        let selected = &SettingsItem::ALL[self.settings_index];
        match selected {
            SettingsItem::ShowKeyboard => {
                self.settings.show_keyboard = !self.settings.show_keyboard
            }
            SettingsItem::Theme => self.settings.theme = cycle_theme(self.settings.theme, forward),
            SettingsItem::Sound => self.settings.sound_on_error = !self.settings.sound_on_error,
            SettingsItem::Difficulty => {
                self.settings.difficulty = if self.settings.difficulty == TrainingDifficulty::Strict
                {
                    TrainingDifficulty::FreeFlow
                } else {
                    TrainingDifficulty::Strict
                }
            }
            SettingsItem::UnlockAccuracy => {
                let step = if forward { 1.0 } else { -1.0 };
                self.settings.unlock_accuracy_threshold =
                    (self.settings.unlock_accuracy_threshold + step).clamp(80.0, 99.0);
            }
            SettingsItem::UnlockWpm => {
                let step = if forward { 1.0 } else { -1.0 };
                self.settings.unlock_wpm_threshold =
                    (self.settings.unlock_wpm_threshold + step).clamp(10.0, 80.0);
            }
            SettingsItem::FocusMode => self.settings.focus_mode = !self.settings.focus_mode,
            SettingsItem::DailyGoal => {
                let step = if forward { 5 } else { -5 };
                let mut value = self.settings.daily_goal_minutes as i32 + step;
                if value < 5 {
                    value = 120;
                } else if value > 120 {
                    value = 5;
                }
                self.settings.daily_goal_minutes = value as u32;
            }
            SettingsItem::ExportCsv
            | SettingsItem::Backup
            | SettingsItem::Import
            | SettingsItem::Back => {}
        }
        self.save();
    }

    fn select_current(&mut self) {
        match self.screen {
            Screen::Home => self.select_home(),
            Screen::Lessons => self.select_lesson(),
            Screen::Settings => self.select_settings(),
            Screen::Pause => self.select_pause(),
            Screen::TypingTestConfig => self.select_test_config(),
            Screen::CodeConfig => self.select_code_config(),
            Screen::SessionSummary => {
                self.summary = None;
                self.screen = Screen::Home;
            }
            Screen::Progress => self.screen = Screen::Home,
            _ => {}
        }
    }

    fn select_home(&mut self) {
        match HomeItem::ALL[self.home_index] {
            HomeItem::StartTraining => self.start_adaptive_training(),
            HomeItem::CodeDrill => self.screen = Screen::CodeConfig,
            HomeItem::TypingTest => self.screen = Screen::TypingTestConfig,
            HomeItem::Lessons => self.screen = Screen::Lessons,
            HomeItem::Progress => self.screen = Screen::Progress,
            HomeItem::Settings => self.screen = Screen::Settings,
            HomeItem::Exit => {
                self.running = false;
                self.save();
            }
        }
    }

    fn select_lesson(&mut self) {
        match LessonItem::ALL[self.lesson_index] {
            LessonItem::GuidedHomeRow => {
                let keys: Vec<char> = "asdfjkl;".chars().collect();
                self.start_lesson(keys, "Guided Home Row", SessionMode::LessonPractice);
            }
            LessonItem::TopRow => self.start_lesson(
                lesson::lesson_top_row(),
                "Top Row Focus",
                SessionMode::LessonPractice,
            ),
            LessonItem::BottomRow => self.start_lesson(
                lesson::lesson_bottom_row(),
                "Bottom Row Focus",
                SessionMode::LessonPractice,
            ),
            LessonItem::WeakKeysDrill => self.start_weak_keys_drill(),
            LessonItem::Back => self.screen = Screen::Home,
        }
    }

    fn select_test_config(&mut self) {
        match self.test_config_index {
            0 | 1 => self.adjust_test_config(true),
            2 => self.start_typing_test(),
            _ => self.screen = Screen::Home,
        }
    }

    fn select_code_config(&mut self) {
        match self.code_config_index {
            0 | 1 => self.adjust_code_config(true),
            2 => self.start_code_drill(),
            _ => self.screen = Screen::Home,
        }
    }

    fn select_settings(&mut self) {
        let selected = &SettingsItem::ALL[self.settings_index];
        match selected {
            SettingsItem::ExportCsv => {
                let state = self.to_persisted_state();
                match export_sessions_csv(&self.storage_paths, &state, None) {
                    Ok(path) => self.set_notice(format!("Exported CSV: {}", path.display())),
                    Err(err) => self.set_notice(format!("Export failed: {err}")),
                }
            }
            SettingsItem::Backup => {
                let state = self.to_persisted_state();
                match backup_progress(&self.storage_paths, &state, None) {
                    Ok(path) => self.set_notice(format!("Backup saved: {}", path.display())),
                    Err(err) => self.set_notice(format!("Backup failed: {err}")),
                }
            }
            SettingsItem::Import => match latest_backup(&self.storage_paths) {
                Ok(Some(path)) => match import_progress(&self.storage_paths, &path) {
                    Ok(state) => {
                        self.settings = state.settings;
                        self.progress = state.progress;
                        self.test_config = state.test_config;
                        self.set_notice(format!("Imported: {}", path.display()));
                    }
                    Err(err) => self.set_notice(format!("Import failed: {err}")),
                },
                Ok(None) => self.set_notice("No backup file found"),
                Err(err) => self.set_notice(format!("Import failed: {err}")),
            },
            SettingsItem::Back => self.screen = Screen::Home,
            _ => self.adjust_setting(true),
        }
    }

    fn select_pause(&mut self) {
        match PauseItem::ALL[self.pause_index] {
            PauseItem::Resume => self.resume_current(),
            PauseItem::Restart => self.restart_current(),
            PauseItem::QuitToHome => {
                self.active_session = None;
                self.screen = Screen::Home;
            }
        }
    }

    fn go_back(&mut self) {
        match self.screen {
            Screen::Home => {
                self.running = false;
                self.save();
            }
            Screen::Lessons
            | Screen::Progress
            | Screen::Settings
            | Screen::TypingTestConfig
            | Screen::CodeConfig => self.screen = Screen::Home,
            Screen::Pause => self.resume_current(),
            Screen::SessionSummary => {
                self.summary = None;
                self.screen = Screen::Home;
            }
            _ => {}
        }
    }

    fn start_adaptive_training(&mut self) {
        let seed = self.next_seed();
        let text = lesson::generate_training_text(
            &self.progress.unlocked_chars,
            &self.progress.key_stats,
            260,
            seed,
        );
        self.start_session(
            SessionPlan::AdaptiveTraining,
            SessionMode::Training,
            "Adaptive Training".to_string(),
            text,
            None,
        );
    }

    fn start_lesson(&mut self, keys: Vec<char>, label: &str, mode: SessionMode) {
        let seed = self.next_seed();
        let text = lesson::generate_training_text(&keys, &self.progress.key_stats, 220, seed);
        self.start_session(
            SessionPlan::Lesson {
                keys,
                label: label.to_string(),
            },
            mode,
            label.to_string(),
            text,
            None,
        );
    }

    fn start_weak_keys_drill(&mut self) {
        let weak = lesson::weakest_keys(&self.progress, 8);
        let seed = self.next_seed();
        let text = lesson::generate_training_text(&weak, &self.progress.key_stats, 220, seed);
        self.start_session(
            SessionPlan::WeakDrill,
            SessionMode::WeakKeysDrill,
            "Weak Keys Drill".to_string(),
            text,
            None,
        );
    }

    fn start_typing_test(&mut self) {
        let seed = self.next_seed();
        let text = lesson::generate_test_text(
            self.test_config.text_type,
            (self.test_config.duration.seconds() as usize).saturating_mul(7),
            seed,
            &self.progress.unlocked_chars,
        );
        let mode = SessionMode::TypingTest;
        self.start_session(
            SessionPlan::TypingTest(self.test_config.clone()),
            mode,
            format!(
                "Typing Test {} {}",
                self.test_config.duration.title(),
                self.test_config.text_type.title()
            ),
            text,
            Some(Duration::from_secs(self.test_config.duration.seconds())),
        );
    }

    fn start_code_drill(&mut self) {
        let seed = self.next_seed();
        let text = code::generate_code_drill_text(
            self.code_config.language,
            self.code_config.topic,
            4,
            seed,
        );
        self.start_session(
            SessionPlan::CodeDrill {
                language: self.code_config.language,
                topic: self.code_config.topic,
            },
            SessionMode::CodeDrill,
            format!(
                "{} {}",
                self.code_config.language.title(),
                self.code_config.topic.title()
            ),
            text,
            None,
        );
    }

    fn start_session(
        &mut self,
        plan: SessionPlan,
        mode: SessionMode,
        label: String,
        text: String,
        time_limit: Option<Duration>,
    ) {
        let now = Instant::now();
        let strict_active = self.settings.difficulty == TrainingDifficulty::Strict
            && mode != SessionMode::TypingTest;

        self.active_session = Some(ActiveSession {
            plan,
            mode,
            label,
            target: text.chars().collect(),
            typed: Vec::new(),
            cursor: 0,
            started_at: now,
            started_wallclock: Local::now(),
            paused_total: Duration::ZERO,
            paused_since: None,
            last_key_at: now,
            time_limit,
            blocked_errors: 0,
            misses_by_key: HashMap::new(),
            wpm_samples: vec![0.0],
            last_sample_sec: 0,
        });

        self.summary = None;
        self.screen = if mode == SessionMode::TypingTest {
            Screen::TypingTest
        } else {
            Screen::Training
        };

        if strict_active {
            self.set_notice("Strict mode active: mistakes must be corrected");
        }
    }

    fn handle_char(&mut self, input_char: char) {
        if self.screen != Screen::Training && self.screen != Screen::TypingTest {
            return;
        }

        let mut should_finish = false;
        let mut append_more_test = false;
        let seed_for_more = self.next_seed();

        if let Some(session) = self.active_session.as_mut() {
            if session.is_paused() {
                return;
            }
            let Some(expected) = session.next_target_char() else {
                return;
            };

            let typed = normalize_input_char(input_char);
            if typed.is_control() {
                return;
            }

            let now = Instant::now();
            let delta_ms = now
                .saturating_duration_since(session.last_key_at)
                .as_millis()
                .min(u128::from(u64::MAX)) as u64;
            session.last_key_at = now;

            let correct = typed.eq_ignore_ascii_case(&expected);
            let key_stat = self
                .progress
                .key_stats
                .entry(expected.to_ascii_lowercase())
                .or_default();
            metrics::update_key_stat(key_stat, correct, delta_ms.max(1));

            let strict_active = self.settings.difficulty == TrainingDifficulty::Strict
                && session.mode != SessionMode::TypingTest;

            if !correct {
                *session.misses_by_key.entry(expected).or_insert(0) += 1;
                if self.settings.sound_on_error {
                    eprint!("\x07");
                }
            }

            if strict_active && !correct {
                session.blocked_errors += 1;
            } else {
                session.typed.push(TypedEntry {
                    expected,
                    typed,
                    correct,
                });
                session.cursor = session.cursor.saturating_add(1);
            }

            if session.mode == SessionMode::TypingTest {
                if session.target.len().saturating_sub(session.cursor) < 40 {
                    append_more_test = true;
                }
            } else if session.cursor >= session.target.len() {
                should_finish = true;
            }
        }

        if append_more_test {
            if let Some(session) = self.active_session.as_mut() {
                let more = lesson::generate_test_text(
                    self.test_config.text_type,
                    140,
                    seed_for_more,
                    &self.progress.unlocked_chars,
                );
                session.target.extend(more.chars());
            }
        }

        if should_finish {
            self.finish_session();
        }
    }

    fn handle_backspace(&mut self) {
        if self.screen != Screen::Training && self.screen != Screen::TypingTest {
            return;
        }

        if let Some(session) = self.active_session.as_mut() {
            if session.cursor > 0 {
                session.cursor -= 1;
                if session.typed.len() > session.cursor {
                    session.typed.pop();
                }
            }
        }
    }

    fn pause_current(&mut self) {
        if self.screen != Screen::Training && self.screen != Screen::TypingTest {
            return;
        }
        if let Some(session) = self.active_session.as_mut() {
            if session.paused_since.is_none() {
                session.paused_since = Some(Instant::now());
            }
            self.paused_from = self.screen;
            self.pause_index = 0;
            self.screen = Screen::Pause;
        }
    }

    fn resume_current(&mut self) {
        if let Some(session) = self.active_session.as_mut() {
            if let Some(since) = session.paused_since.take() {
                session.paused_total += Instant::now().saturating_duration_since(since);
            }
            self.screen = self.paused_from;
        }
    }

    fn restart_current(&mut self) {
        let Some(session) = self.active_session.clone() else {
            self.screen = Screen::Home;
            return;
        };

        match session.plan {
            SessionPlan::AdaptiveTraining => self.start_adaptive_training(),
            SessionPlan::Lesson { keys, label } => {
                self.start_lesson(keys, &label, SessionMode::LessonPractice)
            }
            SessionPlan::WeakDrill => self.start_weak_keys_drill(),
            SessionPlan::TypingTest(config) => {
                self.test_config = config;
                self.start_typing_test();
            }
            SessionPlan::CodeDrill { language, topic } => {
                self.code_config.language = language;
                self.code_config.topic = topic;
                self.start_code_drill();
            }
        }
    }

    pub fn on_tick(&mut self) {
        if let Some((_, at)) = &self.notification {
            if at.elapsed() > Duration::from_secs(4) {
                self.notification = None;
            }
        }

        let mut should_finish = false;
        if let Some(session) = self.active_session.as_mut() {
            if session.is_paused() {
                return;
            }
            let now = Instant::now();
            let elapsed = session.elapsed(now);
            let elapsed_secs = elapsed.as_secs();
            if elapsed_secs > session.last_sample_sec {
                let snap = metrics::compute_live(&session.typed, elapsed, session.blocked_errors);
                session.wpm_samples.push(snap.wpm);
                session.last_sample_sec = elapsed_secs;
            }

            if let Some(limit) = session.time_limit {
                if elapsed >= limit {
                    should_finish = true;
                }
            }
        }

        if should_finish {
            self.finish_session();
        }
    }

    fn finish_session(&mut self) {
        let Some(session) = self.active_session.take() else {
            return;
        };

        let now = Instant::now();
        let elapsed = session.elapsed(now).max(Duration::from_secs(1));
        let snapshot = metrics::compute_live(&session.typed, elapsed, session.blocked_errors);
        let problematic = metrics::problematic_keys(&session.typed, &session.misses_by_key);
        let mut unlocked_new_key = None;
        let (code_language, code_topic) = match &session.plan {
            SessionPlan::CodeDrill { language, topic } => (Some(*language), Some(*topic)),
            _ => (None, None),
        };

        if matches!(
            session.mode,
            SessionMode::Training | SessionMode::LessonPractice
        ) && snapshot.accuracy >= self.settings.unlock_accuracy_threshold
            && snapshot.wpm >= self.settings.unlock_wpm_threshold
        {
            unlocked_new_key = lesson::maybe_unlock_next(
                &mut self.progress,
                &self.settings,
                snapshot.accuracy,
                snapshot.wpm,
            );
        }

        let summary = SessionSummary {
            mode: session.mode,
            wpm: snapshot.wpm,
            accuracy: snapshot.accuracy,
            mistakes: snapshot.errors,
            duration_secs: snapshot.elapsed_secs,
            problematic_keys: problematic.clone(),
            wpm_samples: session.wpm_samples.clone(),
            unlocked_new_key,
            code_language,
            code_topic,
        };

        let result = SessionResult {
            started_at: session.started_wallclock,
            mode: session.mode,
            duration_secs: snapshot.elapsed_secs,
            wpm: snapshot.wpm,
            accuracy: snapshot.accuracy,
            mistakes: snapshot.errors,
            chars_typed: (session.typed.len() as u64).saturating_add(session.blocked_errors),
            problematic_keys: problematic,
            wpm_samples: session.wpm_samples,
            code_language,
            code_topic,
        };

        self.progress.push_session(result);
        self.summary = Some(summary);
        self.screen = Screen::SessionSummary;
        self.save();
    }
}

fn cycle_duration(current: TestDuration, forward: bool) -> TestDuration {
    let idx = TestDuration::ALL
        .iter()
        .position(|d| *d == current)
        .unwrap_or(0);
    if forward {
        TestDuration::ALL[(idx + 1) % TestDuration::ALL.len()]
    } else if idx == 0 {
        TestDuration::ALL[TestDuration::ALL.len() - 1]
    } else {
        TestDuration::ALL[idx - 1]
    }
}

fn cycle_text_type(current: TestTextType, forward: bool) -> TestTextType {
    const ALL: [TestTextType; 3] = [
        TestTextType::RandomWords,
        TestTextType::PseudoWords,
        TestTextType::Quotes,
    ];

    let idx = ALL.iter().position(|t| *t == current).unwrap_or(0);
    if forward {
        ALL[(idx + 1) % ALL.len()]
    } else if idx == 0 {
        ALL[ALL.len() - 1]
    } else {
        ALL[idx - 1]
    }
}

fn cycle_theme(current: crate::models::ColorTheme, forward: bool) -> crate::models::ColorTheme {
    use crate::models::ColorTheme;
    const ALL: [ColorTheme; 6] = [
        ColorTheme::Ocean,
        ColorTheme::Ember,
        ColorTheme::Forest,
        ColorTheme::Slate,
        ColorTheme::Solarized,
        ColorTheme::Neon,
    ];
    let idx = ALL.iter().position(|t| *t == current).unwrap_or(0);
    if forward {
        ALL[(idx + 1) % ALL.len()]
    } else if idx == 0 {
        ALL[ALL.len() - 1]
    } else {
        ALL[idx - 1]
    }
}

fn cycle_language(
    current: crate::models::LanguageMode,
    forward: bool,
) -> crate::models::LanguageMode {
    use crate::models::LanguageMode;
    let all = LanguageMode::ALL;
    let idx = all.iter().position(|v| *v == current).unwrap_or(0);
    if forward {
        all[(idx + 1) % all.len()]
    } else if idx == 0 {
        all[all.len() - 1]
    } else {
        all[idx - 1]
    }
}

fn cycle_code_topic(current: crate::models::CodeTopic, forward: bool) -> crate::models::CodeTopic {
    use crate::models::CodeTopic;
    let all = CodeTopic::ALL;
    let idx = all.iter().position(|v| *v == current).unwrap_or(0);
    if forward {
        all[(idx + 1) % all.len()]
    } else if idx == 0 {
        all[all.len() - 1]
    } else {
        all[idx - 1]
    }
}

fn normalize_input_char(c: char) -> char {
    match c {
        '\n' | '\r' | '\t' => ' ',
        _ => c.to_ascii_lowercase(),
    }
}
