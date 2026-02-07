use crate::app::App;
use crate::engine::{lesson, metrics};
use crate::models::{PauseItem, SessionMode};
use crate::ui::Palette;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;
use std::time::Instant;

pub fn render_session(frame: &mut Frame<'_>, app: &App, p: Palette) {
    let area = frame.size();

    let Some(session) = &app.active_session else {
        let fallback = Paragraph::new("No active session").style(Style::default().fg(p.text));
        frame.render_widget(fallback, area);
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(6),
            if app.settings.show_keyboard {
                Constraint::Length(8)
            } else {
                Constraint::Length(0)
            },
        ])
        .split(area);

    let title = format!(
        " {} | {} | Esc: pause | Backspace: correct ",
        session.mode.title(),
        session.label
    );
    let header = Paragraph::new(title)
        .style(Style::default().fg(p.accent).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    if app.settings.focus_mode {
        render_text_panel(frame, chunks[1], app, p);
    } else {
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
            .split(chunks[1]);
        render_text_panel(frame, body[0], app, p);
        render_status_panel(frame, body[1], app, p);
    }

    render_coach_panel(frame, chunks[2], app, p);

    if app.settings.show_keyboard {
        render_keyboard_panel(frame, chunks[3], app, p);
    }
}

fn render_text_panel(frame: &mut Frame<'_>, area: Rect, app: &App, p: Palette) {
    let Some(session) = &app.active_session else {
        return;
    };

    let mut line = Vec::with_capacity(session.target.len() + 1);
    for (i, ch) in session.target.iter().enumerate() {
        let mut style = Style::default().fg(p.muted);
        let mut display = *ch;

        if i < session.typed.len() {
            let typed = &session.typed[i];
            display = typed.typed;
            style = if typed.correct {
                Style::default().fg(p.success)
            } else {
                Style::default().fg(p.danger)
            };
        }

        if i == session.cursor {
            style = style
                .bg(p.accent)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD);
        }

        line.push(Span::styled(display.to_string(), style));
    }

    if session.cursor >= session.target.len() {
        line.push(Span::styled(" ", Style::default().bg(p.accent)));
    }

    let text = Text::from(vec![Line::from(line)]);
    let panel = Paragraph::new(text).wrap(Wrap { trim: false }).block(
        Block::default()
            .title(" Target Text ")
            .borders(Borders::ALL)
            .style(Style::default().fg(p.text)),
    );
    frame.render_widget(panel, area);
}

fn render_status_panel(frame: &mut Frame<'_>, area: Rect, app: &App, p: Palette) {
    let Some(session) = &app.active_session else {
        return;
    };

    let now = Instant::now();
    let elapsed = session.elapsed(now);
    let m = metrics::compute_live(&session.typed, elapsed, session.blocked_errors);
    let remaining = session.remaining_secs(now);

    let mut lines = vec![
        Line::from(vec![
            Span::styled("WPM ", Style::default().fg(p.muted)),
            Span::styled(
                format!("{:.1}", m.wpm),
                Style::default().fg(p.accent).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("CPM ", Style::default().fg(p.muted)),
            Span::styled(format!("{:.1}", m.cpm), Style::default().fg(p.text)),
        ]),
        Line::from(vec![
            Span::styled("Accuracy ", Style::default().fg(p.muted)),
            Span::styled(
                format!("{:.1}%", m.accuracy),
                Style::default().fg(p.success),
            ),
        ]),
        Line::from(vec![
            Span::styled("Errors ", Style::default().fg(p.muted)),
            Span::styled(format!("{}", m.errors), Style::default().fg(p.danger)),
        ]),
        Line::from(vec![
            Span::styled("Elapsed ", Style::default().fg(p.muted)),
            Span::styled(format_duration(m.elapsed_secs), Style::default().fg(p.text)),
        ]),
    ];

    if let Some(rem) = remaining {
        lines.push(Line::from(vec![
            Span::styled("Remaining ", Style::default().fg(p.muted)),
            Span::styled(format!("{}s", rem), Style::default().fg(p.warning)),
        ]));
    }

    let panel = Paragraph::new(lines).block(
        Block::default()
            .title(" Live Metrics ")
            .borders(Borders::ALL)
            .style(Style::default().fg(p.text)),
    );
    frame.render_widget(panel, area);
}

fn render_coach_panel(frame: &mut Frame<'_>, area: Rect, app: &App, p: Palette) {
    let Some(session) = &app.active_session else {
        return;
    };

    let next = session.next_target_char().unwrap_or(' ');
    let (finger, guidance) = lesson::coach_hint(next);
    let mode_hint = match session.mode {
        SessionMode::TypingTest => "Test mode: keep rhythm and avoid bursts",
        SessionMode::WeakKeysDrill => "Drill mode: emphasize weak keys and consistent timing",
        _ => "Training mode: prioritize clean keystrokes over speed",
    };

    let next_key = if next == ' ' {
        "[SPACE]".to_string()
    } else {
        format!("[ {} ]", next)
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("Next Key ", Style::default().fg(p.muted)),
            Span::styled(
                next_key,
                Style::default().fg(p.accent).add_modifier(Modifier::BOLD),
            ),
            Span::styled("   Finger ", Style::default().fg(p.muted)),
            Span::styled(
                finger,
                Style::default().fg(p.warning).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Guidance: ", Style::default().fg(p.muted)),
            Span::styled(guidance, Style::default().fg(p.text)),
        ]),
        Line::from(vec![
            Span::styled("Coach: ", Style::default().fg(p.muted)),
            Span::styled(mode_hint, Style::default().fg(p.text)),
        ]),
    ];

    let panel = Paragraph::new(lines).block(
        Block::default()
            .title(" Coach ")
            .borders(Borders::ALL)
            .style(Style::default().fg(p.text)),
    );
    frame.render_widget(panel, area);
}

fn render_keyboard_panel(frame: &mut Frame<'_>, area: Rect, app: &App, p: Palette) {
    let Some(session) = &app.active_session else {
        return;
    };
    let target = session.next_target_char();

    let rows = vec![
        vec![
            "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=",
        ],
        vec![
            "q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "[", "]", "\\",
        ],
        vec!["a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'"],
        vec!["z", "x", "c", "v", "b", "n", "m", ",", ".", "/"],
        vec!["space"],
    ];

    let mut lines = Vec::new();
    for row in rows {
        let mut spans = Vec::new();
        for key in row {
            let (label, key_char) = if key == "space" {
                ("_____space_____", ' ')
            } else {
                (key, key.chars().next().unwrap_or(' '))
            };

            let style = key_style(key_char, target, p);
            spans.push(Span::styled(format!("[{}]", label), style));
            spans.push(Span::raw(" "));
        }
        lines.push(Line::from(spans));
    }

    let panel = Paragraph::new(lines).block(
        Block::default()
            .title(" ASCII Keyboard (US QWERTY) ")
            .borders(Borders::ALL)
            .style(Style::default().fg(p.text)),
    );
    frame.render_widget(panel, area);
}

pub fn render_pause_overlay(frame: &mut Frame<'_>, app: &App, p: Palette) {
    let popup = centered_rect(42, 34, frame.size());
    frame.render_widget(Clear, popup);

    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(
        "Session Paused",
        Style::default().fg(p.accent).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for (idx, item) in PauseItem::ALL.iter().enumerate() {
        let selected = idx == app.pause_index;
        let style = if selected {
            Style::default()
                .fg(Color::Black)
                .bg(p.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(p.text)
        };
        lines.push(Line::from(Span::styled(
            format!("  {}", item.title()),
            style,
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Esc: Resume",
        Style::default().fg(p.muted),
    )));

    let widget = Paragraph::new(lines).block(
        Block::default()
            .title(" Pause Menu ")
            .borders(Borders::ALL)
            .style(Style::default().fg(p.text).bg(p.panel)),
    );
    frame.render_widget(widget, popup);
}

fn key_style(key: char, target: Option<char>, p: Palette) -> Style {
    let target_match = match target {
        Some(' ') => key == ' ',
        Some(t) => lesson::primary_key_for_char(t) == key,
        None => false,
    };

    if target_match {
        return Style::default()
            .fg(Color::Black)
            .bg(p.accent)
            .add_modifier(Modifier::BOLD);
    }

    if "asdfjkl;".contains(key) {
        return Style::default().fg(p.warning).add_modifier(Modifier::BOLD);
    }

    let finger = lesson::finger_for_char(key);
    let color = match finger {
        "Left pinky" => Color::Rgb(215, 120, 120),
        "Left ring" => Color::Rgb(215, 160, 100),
        "Left middle" => Color::Rgb(219, 205, 110),
        "Left index" => Color::Rgb(130, 195, 125),
        "Right index" => Color::Rgb(110, 188, 205),
        "Right middle" => Color::Rgb(132, 156, 225),
        "Right ring" => Color::Rgb(183, 142, 222),
        "Right pinky" => Color::Rgb(222, 126, 186),
        "Thumb" => p.success,
        _ => p.muted,
    };

    Style::default().fg(color)
}

fn centered_rect(width_percent: u16, height_percent: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height_percent) / 2),
            Constraint::Percentage(height_percent),
            Constraint::Percentage((100 - height_percent) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_percent) / 2),
            Constraint::Percentage(width_percent),
            Constraint::Percentage((100 - width_percent) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}

fn format_duration(secs: u64) -> String {
    let m = secs / 60;
    let s = secs % 60;
    format!("{:02}:{:02}", m, s)
}
