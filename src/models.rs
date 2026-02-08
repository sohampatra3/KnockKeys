use chrono::{DateTime, Duration as ChronoDuration, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Training,
    TypingTestConfig,
    CodeConfig,
    TypingTest,
    Lessons,
    Progress,
    AiInsights,
    Settings,
    Pause,
    SessionSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeItem {
    StartTraining,
    CodeDrill,
    TypingTest,
    Lessons,
    Progress,
    AiGuruReport,
    Settings,
    Exit,
}

impl HomeItem {
    pub const ALL: [HomeItem; 8] = [
        HomeItem::StartTraining,
        HomeItem::CodeDrill,
        HomeItem::TypingTest,
        HomeItem::Lessons,
        HomeItem::Progress,
        HomeItem::AiGuruReport,
        HomeItem::Settings,
        HomeItem::Exit,
    ];

    pub fn title(self) -> &'static str {
        match self {
            HomeItem::StartTraining => "Start Training",
            HomeItem::CodeDrill => "Code Drill Mode",
            HomeItem::TypingTest => "Typing Test",
            HomeItem::Lessons => "Lessons (Key Practice)",
            HomeItem::Progress => "Progress & Stats",
            HomeItem::AiGuruReport => "Typing Guru AI Report",
            HomeItem::Settings => "Settings",
            HomeItem::Exit => "Exit",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseItem {
    Resume,
    Restart,
    QuitToHome,
}

impl PauseItem {
    pub const ALL: [PauseItem; 3] = [PauseItem::Resume, PauseItem::Restart, PauseItem::QuitToHome];

    pub fn title(self) -> &'static str {
        match self {
            PauseItem::Resume => "Resume",
            PauseItem::Restart => "Restart",
            PauseItem::QuitToHome => "Quit to Home",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LessonItem {
    GuidedHomeRow,
    TopRow,
    BottomRow,
    MixedKeyboard,
    FullKeyboard,
    WeakKeysDrill,
    Back,
}

impl LessonItem {
    pub const ALL: [LessonItem; 7] = [
        LessonItem::GuidedHomeRow,
        LessonItem::TopRow,
        LessonItem::BottomRow,
        LessonItem::MixedKeyboard,
        LessonItem::FullKeyboard,
        LessonItem::WeakKeysDrill,
        LessonItem::Back,
    ];

    pub fn title(self) -> &'static str {
        match self {
            LessonItem::GuidedHomeRow => "Guided Home Row",
            LessonItem::TopRow => "Top Row Focus",
            LessonItem::BottomRow => "Bottom Row Focus",
            LessonItem::MixedKeyboard => "Mixed Keyboard Practice",
            LessonItem::FullKeyboard => "Full Keyboard Practice",
            LessonItem::WeakKeysDrill => "Weak Keys Drill",
            LessonItem::Back => "Back",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainingDifficulty {
    Strict,
    FreeFlow,
}

impl TrainingDifficulty {
    pub fn title(self) -> &'static str {
        match self {
            TrainingDifficulty::Strict => "Strict",
            TrainingDifficulty::FreeFlow => "Free-flow",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorTheme {
    Ocean,
    Ember,
    Forest,
    Slate,
    Solarized,
    Neon,
}

impl ColorTheme {
    pub fn title(self) -> &'static str {
        match self {
            ColorTheme::Ocean => "Ocean",
            ColorTheme::Ember => "Ember",
            ColorTheme::Forest => "Forest",
            ColorTheme::Slate => "Slate",
            ColorTheme::Solarized => "Solarized",
            ColorTheme::Neon => "Neon",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionMode {
    Training,
    TypingTest,
    LessonPractice,
    WeakKeysDrill,
    CodeDrill,
}

impl SessionMode {
    pub fn title(self) -> &'static str {
        match self {
            SessionMode::Training => "Training",
            SessionMode::TypingTest => "Typing Test",
            SessionMode::LessonPractice => "Lesson Practice",
            SessionMode::WeakKeysDrill => "Weak Keys Drill",
            SessionMode::CodeDrill => "Code Drill",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageMode {
    Cpp,
    C,
    Rust,
    Python,
    Go,
}

impl LanguageMode {
    pub const ALL: [LanguageMode; 5] = [
        LanguageMode::Cpp,
        LanguageMode::C,
        LanguageMode::Rust,
        LanguageMode::Python,
        LanguageMode::Go,
    ];

    pub fn title(self) -> &'static str {
        match self {
            LanguageMode::Cpp => "C++",
            LanguageMode::C => "C",
            LanguageMode::Rust => "Rust",
            LanguageMode::Python => "Python",
            LanguageMode::Go => "Go",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            LanguageMode::Cpp => "cpp",
            LanguageMode::C => "c",
            LanguageMode::Rust => "rust",
            LanguageMode::Python => "python",
            LanguageMode::Go => "go",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeTopic {
    Loops,
    IfElse,
    Sorting,
    Searching,
    Mixed,
}

impl CodeTopic {
    pub const ALL: [CodeTopic; 5] = [
        CodeTopic::Loops,
        CodeTopic::IfElse,
        CodeTopic::Sorting,
        CodeTopic::Searching,
        CodeTopic::Mixed,
    ];

    pub fn title(self) -> &'static str {
        match self {
            CodeTopic::Loops => "Loops",
            CodeTopic::IfElse => "If / Else",
            CodeTopic::Sorting => "Sorting",
            CodeTopic::Searching => "Searching",
            CodeTopic::Mixed => "Mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestDuration {
    Sec15,
    Sec30,
    Sec60,
    Sec120,
}

impl TestDuration {
    pub const ALL: [TestDuration; 4] = [
        TestDuration::Sec15,
        TestDuration::Sec30,
        TestDuration::Sec60,
        TestDuration::Sec120,
    ];

    pub fn seconds(self) -> u64 {
        match self {
            TestDuration::Sec15 => 15,
            TestDuration::Sec30 => 30,
            TestDuration::Sec60 => 60,
            TestDuration::Sec120 => 120,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            TestDuration::Sec15 => "15s",
            TestDuration::Sec30 => "30s",
            TestDuration::Sec60 => "60s",
            TestDuration::Sec120 => "120s",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestTextType {
    RandomWords,
    PseudoWords,
    Quotes,
}

impl TestTextType {
    pub fn title(self) -> &'static str {
        match self {
            TestTextType::RandomWords => "Random Words",
            TestTextType::PseudoWords => "Pseudo-Words",
            TestTextType::Quotes => "Quotes",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub duration: TestDuration,
    pub text_type: TestTextType,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            duration: TestDuration::Sec60,
            text_type: TestTextType::RandomWords,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeConfig {
    pub language: LanguageMode,
    pub topic: CodeTopic,
}

impl Default for CodeConfig {
    fn default() -> Self {
        Self {
            language: LanguageMode::Rust,
            topic: CodeTopic::Mixed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub show_keyboard: bool,
    pub theme: ColorTheme,
    pub sound_on_error: bool,
    pub difficulty: TrainingDifficulty,
    pub unlock_accuracy_threshold: f64,
    pub unlock_wpm_threshold: f64,
    pub focus_mode: bool,
    pub daily_goal_minutes: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_keyboard: true,
            theme: ColorTheme::Ocean,
            sound_on_error: false,
            difficulty: TrainingDifficulty::Strict,
            unlock_accuracy_threshold: 92.0,
            unlock_wpm_threshold: 20.0,
            focus_mode: false,
            daily_goal_minutes: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyStat {
    pub attempts: u64,
    pub correct: u64,
    pub total_time_ms: u64,
}

impl KeyStat {
    pub fn error_rate(&self) -> f64 {
        if self.attempts == 0 {
            0.0
        } else {
            1.0 - (self.correct as f64 / self.attempts as f64)
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.attempts == 0 {
            0.0
        } else {
            self.correct as f64 / self.attempts as f64
        }
    }

    pub fn avg_time_ms(&self) -> f64 {
        if self.attempts == 0 {
            0.0
        } else {
            self.total_time_ms as f64 / self.attempts as f64
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    pub started_at: DateTime<Local>,
    pub mode: SessionMode,
    pub duration_secs: u64,
    pub wpm: f64,
    pub accuracy: f64,
    pub mistakes: u64,
    pub chars_typed: u64,
    pub problematic_keys: Vec<char>,
    pub wpm_samples: Vec<f64>,
    pub code_language: Option<LanguageMode>,
    pub code_topic: Option<CodeTopic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LanguageSpeedStat {
    pub sessions: u64,
    pub total_wpm: f64,
    pub best_wpm: f64,
    pub total_chars: u64,
    pub total_secs: u64,
}

impl LanguageSpeedStat {
    pub fn avg_wpm(&self) -> f64 {
        if self.sessions == 0 {
            0.0
        } else {
            self.total_wpm / self.sessions as f64
        }
    }

    pub fn apply_session(&mut self, session: &SessionResult) {
        self.sessions += 1;
        self.total_wpm += session.wpm;
        self.best_wpm = self.best_wpm.max(session.wpm);
        self.total_chars += session.chars_typed;
        self.total_secs += session.duration_secs;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressData {
    pub version: u32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub total_practice_secs: u64,
    pub total_chars_typed: u64,
    pub best_wpm: f64,
    pub unlocked_chars: Vec<char>,
    pub key_stats: HashMap<char, KeyStat>,
    pub session_history: Vec<SessionResult>,
    pub daily_minutes: HashMap<String, u32>,
    #[serde(default)]
    pub language_stats: HashMap<String, LanguageSpeedStat>,
}

impl Default for ProgressData {
    fn default() -> Self {
        Self {
            version: 1,
            created_at: Local::now(),
            updated_at: Local::now(),
            total_practice_secs: 0,
            total_chars_typed: 0,
            best_wpm: 0.0,
            unlocked_chars: vec!['f', 'j', 'd', 'k', 's', 'l', 'a', ';'],
            key_stats: HashMap::new(),
            session_history: Vec::new(),
            daily_minutes: HashMap::new(),
            language_stats: HashMap::new(),
        }
    }
}

impl ProgressData {
    pub fn today_key() -> String {
        Local::now().format("%Y-%m-%d").to_string()
    }

    pub fn current_streak(&self) -> u32 {
        let today = Local::now().date_naive();
        self.streak_from(today)
    }

    fn streak_from(&self, start: NaiveDate) -> u32 {
        let mut streak = 0;
        let mut day = start;
        loop {
            let key = day.format("%Y-%m-%d").to_string();
            let practiced = self.daily_minutes.get(&key).copied().unwrap_or(0);
            if practiced == 0 {
                break;
            }
            streak += 1;
            day -= ChronoDuration::days(1);
        }
        streak
    }

    pub fn record_daily_minutes(&mut self, minutes: u32) {
        let key = Self::today_key();
        *self.daily_minutes.entry(key).or_insert(0) += minutes;
    }

    pub fn push_session(&mut self, session: SessionResult) {
        self.best_wpm = self.best_wpm.max(session.wpm);
        self.total_practice_secs += session.duration_secs;
        self.total_chars_typed += session.chars_typed;
        self.updated_at = Local::now();

        let minutes = ((session.duration_secs as f64) / 60.0).ceil() as u32;
        self.record_daily_minutes(minutes.max(1));

        if let Some(lang) = session.code_language {
            let entry = self
                .language_stats
                .entry(lang.key().to_string())
                .or_default();
            entry.apply_session(&session);
        }

        self.session_history.push(session);
        if self.session_history.len() > 300 {
            let trim = self.session_history.len() - 300;
            self.session_history.drain(0..trim);
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypedEntry {
    pub expected: char,
    pub typed: char,
    pub correct: bool,
}

#[derive(Debug, Clone)]
pub struct SessionSummary {
    pub mode: SessionMode,
    pub wpm: f64,
    pub accuracy: f64,
    pub mistakes: u64,
    pub duration_secs: u64,
    pub problematic_keys: Vec<char>,
    pub wpm_samples: Vec<f64>,
    pub unlocked_new_key: Option<char>,
    pub code_language: Option<LanguageMode>,
    pub code_topic: Option<CodeTopic>,
}

#[derive(Debug, Clone)]
pub enum SettingsItem {
    ShowKeyboard,
    Theme,
    Sound,
    Difficulty,
    UnlockAccuracy,
    UnlockWpm,
    FocusMode,
    DailyGoal,
    ExportCsv,
    Backup,
    Import,
    Back,
}

impl SettingsItem {
    pub const ALL: [SettingsItem; 12] = [
        SettingsItem::ShowKeyboard,
        SettingsItem::Theme,
        SettingsItem::Sound,
        SettingsItem::Difficulty,
        SettingsItem::UnlockAccuracy,
        SettingsItem::UnlockWpm,
        SettingsItem::FocusMode,
        SettingsItem::DailyGoal,
        SettingsItem::ExportCsv,
        SettingsItem::Backup,
        SettingsItem::Import,
        SettingsItem::Back,
    ];

    pub fn title(&self) -> &'static str {
        match self {
            SettingsItem::ShowKeyboard => "Keyboard Visualization",
            SettingsItem::Theme => "Color Theme",
            SettingsItem::Sound => "Error Sound",
            SettingsItem::Difficulty => "Training Difficulty",
            SettingsItem::UnlockAccuracy => "Unlock Accuracy Threshold",
            SettingsItem::UnlockWpm => "Unlock WPM Threshold",
            SettingsItem::FocusMode => "Focus Mode",
            SettingsItem::DailyGoal => "Daily Goal (minutes)",
            SettingsItem::ExportCsv => "Export Stats to CSV",
            SettingsItem::Backup => "Backup Progress JSON",
            SettingsItem::Import => "Import Progress JSON",
            SettingsItem::Back => "Back",
        }
    }
}
