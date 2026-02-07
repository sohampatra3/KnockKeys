use crate::app::App;
use crate::models::{HomeItem, LessonItem};
use crate::ui::Palette;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_home(frame: &mut Frame<'_>, app: &App, p: Palette) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title = Paragraph::new("TypeTrainer - Adaptive Typing Coach")
        .style(Style::default().fg(p.accent).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    let (done, goal) = app.today_goal_progress();
    let ratio = if goal == 0 {
        0.0
    } else {
        (done as f64 / goal as f64).min(1.0)
    };
    let goal = Gauge::default()
        .block(Block::default().title(" Daily Goal ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(p.success).bg(p.panel))
        .ratio(ratio)
        .label(format!("{done}/{goal} min"));
    frame.render_widget(goal, chunks[1]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(chunks[2]);

    let items: Vec<ListItem> = HomeItem::ALL
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let selected = idx == app.home_index;
            let style = if selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(p.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(p.text)
            };
            ListItem::new(Line::from(Span::styled(
                format!("  {}", item.title()),
                style,
            )))
        })
        .collect();

    let menu = List::new(items).block(
        Block::default()
            .title(" Home ")
            .borders(Borders::ALL)
            .style(Style::default().fg(p.text)),
    );
    frame.render_widget(menu, body[0]);

    let streak = app.progress.current_streak();
    let recent = app
        .progress
        .session_history
        .iter()
        .rev()
        .take(3)
        .map(|s| format!("{} | {:.1} WPM | {:.1}%", s.mode.title(), s.wpm, s.accuracy))
        .collect::<Vec<_>>()
        .join("\n");

    let right = Paragraph::new(format!(
        "Unlocked keys: {}\nBest WPM: {:.1}\nCurrent streak: {} day(s)\nTotal time: {} min\n\nRecent sessions:\n{}",
        app.progress.unlocked_chars.iter().collect::<String>(),
        app.progress.best_wpm,
        streak,
        app.progress.total_practice_secs / 60,
        if recent.is_empty() {
            "No sessions yet".to_string()
        } else {
            recent
        }
    ))
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .title(" Snapshot ")
            .borders(Borders::ALL)
            .style(Style::default().fg(p.text)),
    );
    frame.render_widget(right, body[1]);

    let controls = Paragraph::new("Up/Down or j/k: Navigate  |  Enter: Select  |  q: Quit")
        .style(Style::default().fg(p.muted))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(controls, chunks[3]);
}

pub fn render_lessons(frame: &mut Frame<'_>, app: &App, p: Palette) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(4),
        ])
        .split(frame.size());

    let title = Paragraph::new("Lessons & Key Practice")
        .style(Style::default().fg(p.accent).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
        .split(chunks[1]);

    let items: Vec<ListItem> = LessonItem::ALL
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let selected = idx == app.lesson_index;
            let style = if selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(p.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(p.text)
            };
            ListItem::new(Line::from(Span::styled(
                format!("  {}", item.title()),
                style,
            )))
        })
        .collect();

    frame.render_widget(
        List::new(items).block(
            Block::default()
                .title(" Lesson Menu ")
                .borders(Borders::ALL),
        ),
        body[0],
    );

    let desc = match LessonItem::ALL[app.lesson_index] {
        LessonItem::GuidedHomeRow => {
            "Train core home-row keys with coach hints.\nKeys: a s d f j k l ;"
        }
        LessonItem::TopRow => "Reach training for top row keys.\nKeys: q w e r t y u i o p [ ] \\",
        LessonItem::BottomRow => "Reach training for bottom row keys.\nKeys: z x c v b n m , . /",
        LessonItem::WeakKeysDrill => {
            "Adaptive drill that targets your slowest and most error-prone keys."
        }
        LessonItem::Back => "Return to home menu.",
    };

    let right = Paragraph::new(format!(
        "{}\n\nCurrent unlocked: {}\nStrict mode: {}",
        desc,
        app.progress.unlocked_chars.iter().collect::<String>(),
        app.settings.difficulty.title()
    ))
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .title(" Lesson Details ")
            .borders(Borders::ALL),
    );
    frame.render_widget(right, body[1]);

    let footer = Paragraph::new("Enter: Start  |  Esc: Back  |  Up/Down or j/k: Navigate")
        .style(Style::default().fg(p.muted))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, chunks[2]);
}
