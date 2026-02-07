use crate::app::App;
use crate::ui::Palette;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Sparkline, Wrap};
use ratatui::Frame;

pub fn render_progress(frame: &mut Frame<'_>, app: &App, p: Palette) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Min(10),
            Constraint::Length(6),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title = Paragraph::new("Progress & Analytics")
        .style(Style::default().fg(p.accent).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    let streak = app.progress.current_streak();
    let (today, goal) = app.today_goal_progress();
    let top = Paragraph::new(format!(
        "Daily streak: {} day(s)\nTotal practice time: {} min\nBest WPM: {:.1}\nToday: {}/{} min",
        streak,
        app.progress.total_practice_secs / 60,
        app.progress.best_wpm,
        today,
        goal
    ))
    .block(Block::default().title(" Overview ").borders(Borders::ALL));
    frame.render_widget(top, chunks[1]);

    let mid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(chunks[2]);

    let mut recent_lines = Vec::new();
    for s in app.progress.session_history.iter().rev().take(8) {
        recent_lines.push(Line::from(format!(
            "{} | {}s | {:.1} WPM | {:.1}%",
            s.mode.title(),
            s.duration_secs,
            s.wpm,
            s.accuracy
        )));
    }
    if recent_lines.is_empty() {
        recent_lines.push(Line::from("No sessions yet"));
    }

    let recent = Paragraph::new(recent_lines)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(" Recent Sessions ")
                .borders(Borders::ALL),
        );
    frame.render_widget(recent, mid[0]);

    let heatmap = Paragraph::new(heatmap_lines(app, p))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(" Per-key Heatmap (error rate) ")
                .borders(Borders::ALL),
        );
    frame.render_widget(heatmap, mid[1]);

    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(chunks[3]);

    let trend_data: Vec<u64> = app
        .progress
        .session_history
        .iter()
        .rev()
        .take(10)
        .map(|s| s.wpm as u64)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let graph = Sparkline::default()
        .block(
            Block::default()
                .title(" WPM Trend (last 10) ")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(p.accent))
        .data(&trend_data);
    frame.render_widget(graph, bottom[0]);

    let slowest = format!(
        "Slowest keys:\n{}\n\nLanguage speed:\n{}",
        slowest_keys(app),
        language_speed_summary(app)
    );
    let slow_panel = Paragraph::new(slowest).wrap(Wrap { trim: true }).block(
        Block::default()
            .title(" Key + Language Stats ")
            .borders(Borders::ALL),
    );
    frame.render_widget(slow_panel, bottom[1]);

    let footer = Paragraph::new("Esc: Back to home")
        .style(Style::default().fg(p.muted))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, chunks[4]);
}

fn heatmap_lines(app: &App, p: Palette) -> Vec<Line<'static>> {
    let rows = ["qwertyuiop", "asdfghjkl;", "zxcvbnm,./"];
    let mut lines = Vec::new();

    for row in rows {
        let mut spans = Vec::new();
        for c in row.chars() {
            let stat = app.progress.key_stats.get(&c);
            let error = stat.map(|s| s.error_rate()).unwrap_or(0.0);
            let color = if error > 0.25 {
                p.danger
            } else if error > 0.12 {
                p.warning
            } else if error > 0.05 {
                Color::Rgb(210, 195, 120)
            } else {
                p.success
            };

            let bg = if app.progress.unlocked_chars.contains(&c) {
                p.panel
            } else {
                p.background
            };

            spans.push(Span::styled(
                format!(" {} ", c),
                Style::default()
                    .fg(color)
                    .bg(bg)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::raw(" "));
        }
        lines.push(Line::from(spans));
    }

    lines
}

fn slowest_keys(app: &App) -> String {
    let mut pairs: Vec<(char, f64)> = app
        .progress
        .key_stats
        .iter()
        .filter(|(_, s)| s.attempts >= 3)
        .map(|(c, s)| (*c, s.avg_time_ms()))
        .collect();

    pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    if pairs.is_empty() {
        return "Not enough data yet".to_string();
    }

    pairs
        .into_iter()
        .take(6)
        .map(|(c, ms)| format!("{}: {:.0}ms", c, ms))
        .collect::<Vec<_>>()
        .join("\n")
}

fn language_speed_summary(app: &App) -> String {
    let mut rows: Vec<(String, f64, f64)> = app
        .progress
        .language_stats
        .iter()
        .map(|(k, v)| (k.clone(), v.avg_wpm(), v.best_wpm))
        .collect();
    rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    if rows.is_empty() {
        return "No language drills yet".to_string();
    }

    rows.into_iter()
        .take(5)
        .map(|(k, avg, best)| format!("{}: avg {:.1} / best {:.1}", pretty_lang_key(&k), avg, best))
        .collect::<Vec<_>>()
        .join("\n")
}

fn pretty_lang_key(key: &str) -> &str {
    match key {
        "cpp" => "C++",
        "c" => "C",
        "rust" => "Rust",
        "python" => "Python",
        "go" => "Go",
        _ => key,
    }
}
