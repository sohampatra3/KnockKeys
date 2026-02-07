pub mod code;
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
        ColorTheme::Slate => Palette {
            background: Color::Rgb(21, 24, 30),
            panel: Color::Rgb(35, 42, 52),
            text: Color::Rgb(227, 232, 240),
            muted: Color::Rgb(138, 152, 173),
            accent: Color::Rgb(119, 177, 255),
            success: Color::Rgb(123, 214, 156),
            danger: Color::Rgb(244, 114, 114),
            warning: Color::Rgb(251, 191, 84),
        },
        ColorTheme::Solarized => Palette {
            background: Color::Rgb(0, 43, 54),
            panel: Color::Rgb(7, 54, 66),
            text: Color::Rgb(238, 232, 213),
            muted: Color::Rgb(147, 161, 161),
            accent: Color::Rgb(38, 139, 210),
            success: Color::Rgb(133, 153, 0),
            danger: Color::Rgb(220, 50, 47),
            warning: Color::Rgb(181, 137, 0),
        },
        ColorTheme::Neon => Palette {
            background: Color::Rgb(9, 10, 20),
            panel: Color::Rgb(19, 22, 38),
            text: Color::Rgb(231, 255, 247),
            muted: Color::Rgb(121, 142, 150),
            accent: Color::Rgb(53, 255, 196),
            success: Color::Rgb(77, 242, 107),
            danger: Color::Rgb(255, 88, 122),
            warning: Color::Rgb(255, 212, 77),
        },
    }
}

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let p = palette(app.settings.theme);

    match app.screen {
        Screen::Home => home::render_home(frame, app, p),
        Screen::Lessons => home::render_lessons(frame, app, p),
        Screen::CodeConfig => code::render_code_config(frame, app, p),
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
