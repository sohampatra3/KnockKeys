pub mod home;
pub mod settings;
pub mod stats;
pub mod test;
pub mod training;

use crate::app::App;
use crate::models::{ColorTheme, Screen};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub background: Color,
    pub panel: Color,
    pub text: Color,
    pub muted: Color,
    pub accent: Color,
    pub success: Color,
    pub danger: Color,
    pub warning: Color,
}

pub fn palette(theme: ColorTheme) -> Palette {
    match theme {
        ColorTheme::Ocean => Palette {
            background: Color::Rgb(15, 22, 35),
            panel: Color::Rgb(24, 34, 52),
            text: Color::Rgb(225, 235, 245),
            muted: Color::Rgb(120, 150, 170),
            accent: Color::Rgb(70, 190, 225),
            success: Color::Rgb(120, 210, 130),
            danger: Color::Rgb(238, 98, 98),
            warning: Color::Rgb(248, 189, 76),
        },
        ColorTheme::Ember => Palette {
            background: Color::Rgb(32, 19, 14),
            panel: Color::Rgb(49, 29, 22),
            text: Color::Rgb(245, 231, 216),
            muted: Color::Rgb(180, 146, 121),
            accent: Color::Rgb(255, 150, 72),
            success: Color::Rgb(134, 211, 120),
            danger: Color::Rgb(242, 103, 91),
            warning: Color::Rgb(255, 203, 94),
        },
        ColorTheme::Forest => Palette {
            background: Color::Rgb(16, 24, 17),
            panel: Color::Rgb(26, 39, 27),
            text: Color::Rgb(223, 240, 221),
            muted: Color::Rgb(125, 165, 120),
            accent: Color::Rgb(104, 208, 132),
            success: Color::Rgb(146, 227, 116),
            danger: Color::Rgb(236, 95, 98),
            warning: Color::Rgb(232, 201, 101),
        },
    }
}

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let p = palette(app.settings.theme);

    match app.screen {
        Screen::Home => home::render_home(frame, app, p),
        Screen::Lessons => home::render_lessons(frame, app, p),
        Screen::Training | Screen::TypingTest => training::render_session(frame, app, p),
        Screen::TypingTestConfig => test::render_config(frame, app, p),
        Screen::Progress => stats::render_progress(frame, app, p),
        Screen::Settings => settings::render_settings(frame, app, p),
        Screen::Pause => {
            training::render_session(frame, app, p);
            training::render_pause_overlay(frame, app, p);
        }
        Screen::SessionSummary => test::render_summary(frame, app, p),
    }

    if let Some((note, _)) = &app.notification {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.size());

        let widget = Paragraph::new(note.as_str())
            .style(Style::default().fg(p.warning))
            .block(Block::default().borders(Borders::TOP));
        frame.render_widget(widget, rows[1]);
    }
}
