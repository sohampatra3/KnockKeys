use crate::app::App;
use crate::models::{SettingsItem, TrainingDifficulty};
use crate::ui::Palette;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_settings(frame: &mut Frame<'_>, app: &App, p: Palette) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(4)])
        .split(frame.size());

    let title = Paragraph::new("Settings")
        .style(Style::default().fg(p.accent).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(48), Constraint::Percentage(52)])
        .split(chunks[1]);

    let items: Vec<ListItem> = SettingsItem::ALL
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let value = setting_value(app, item);
            let selected = idx == app.settings_index;
            let style = if selected {
                Style::default().fg(Color::Black).bg(p.accent).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(p.text)
            };
            ListItem::new(Line::from(Span::styled(
                format!("  {:<30} {}", item.title(), value),
                style,
            )))
        })
        .collect();

    frame.render_widget(
        List::new(items).block(
            Block::default()
                .title(" Preferences ")
                .borders(Borders::ALL)
                .style(Style::default().fg(p.text)),
        ),
        body[0],
    );

    let selected = &SettingsItem::ALL[app.settings_index];
    let details = setting_help(selected, app);
    let right = Paragraph::new(details)
        .wrap(Wrap { trim: true })
        .block(Block::default().title(" Details ").borders(Borders::ALL));
    frame.render_widget(right, body[1]);

    let footer = Paragraph::new(
        "Up/Down: select  |  Left/Right: change value  |  Enter: apply/action  |  Esc: back",
    )
    .style(Style::default().fg(p.muted))
    .block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, chunks[2]);
}

fn setting_value(app: &App, item: &SettingsItem) -> String {
    match item {
        SettingsItem::ShowKeyboard => on_off(app.settings.show_keyboard),
        SettingsItem::Theme => app.settings.theme.title().to_string(),
        SettingsItem::Sound => on_off(app.settings.sound_on_error),
        SettingsItem::Difficulty => app.settings.difficulty.title().to_string(),
        SettingsItem::UnlockAccuracy => format!("{:.0}%", app.settings.unlock_accuracy_threshold),
        SettingsItem::UnlockWpm => format!("{:.0}", app.settings.unlock_wpm_threshold),
        SettingsItem::FocusMode => on_off(app.settings.focus_mode),
        SettingsItem::DailyGoal => format!("{} min", app.settings.daily_goal_minutes),
        SettingsItem::ExportCsv => "(run)".to_string(),
        SettingsItem::Backup => "(run)".to_string(),
        SettingsItem::Import => "(latest backup)".to_string(),
        SettingsItem::Back => String::new(),
    }
}

fn setting_help(item: &SettingsItem, app: &App) -> String {
    match item {
        SettingsItem::ShowKeyboard => {
            "Show an ASCII US-QWERTY keyboard and highlight the target key/finger zones.".to_string()
        }
        SettingsItem::Theme => "Switch between Ocean, Ember, and Forest terminal themes.".to_string(),
        SettingsItem::Sound => {
            "Emit terminal bell on errors. Useful when focus is not on visual feedback.".to_string()
        }
        SettingsItem::Difficulty => {
            if app.settings.difficulty == TrainingDifficulty::Strict {
                "Strict mode blocks progress on incorrect keys until corrected.".to_string()
            } else {
                "Free-flow mode lets you continue typing after mistakes.".to_string()
            }
        }
        SettingsItem::UnlockAccuracy => {
            "Minimum accuracy needed before adaptive training unlocks new keys.".to_string()
        }
        SettingsItem::UnlockWpm => {
            "Minimum WPM needed before adaptive training unlocks new keys.".to_string()
        }
        SettingsItem::FocusMode => {
            "Focus Mode hides the live metrics sidebar during typing sessions.".to_string()
        }
        SettingsItem::DailyGoal => "Target amount of daily practice in minutes.".to_string(),
        SettingsItem::ExportCsv => {
            "Export session history to CSV under ~/.typetrainer/exports/.".to_string()
        }
        SettingsItem::Backup => {
            "Save full progress/settings JSON backup under ~/.typetrainer/backups/.".to_string()
        }
        SettingsItem::Import => {
            "Restore from latest backup file in ~/.typetrainer/backups/.".to_string()
        }
        SettingsItem::Back => "Return to the home screen.".to_string(),
    }
}

fn on_off(v: bool) -> String {
    if v {
        "ON".to_string()
    } else {
        "OFF".to_string()
    }
}
