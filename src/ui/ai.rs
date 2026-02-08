use crate::app::App;
use crate::ui::Palette;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_ai_insights(frame: &mut Frame<'_>, app: &App, p: Palette) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title = Paragraph::new("Typing Guru AI Analytics")
        .style(Style::default().fg(p.accent).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    let report = app
        .ai_report
        .as_deref()
        .unwrap_or("Press Enter to generate a new Typing Guru report.");

    let lines: Vec<Line<'_>> = report.lines().map(Line::from).collect();
    let height_hint = chunks[1].height.saturating_sub(2) as usize;
    let max_scroll = lines.len().saturating_sub(height_hint.max(1));
    let scroll = app.ai_scroll.min(max_scroll) as u16;

    let body = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Full Analytics ")
                .borders(Borders::ALL)
                .style(Style::default().fg(p.text)),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(body, chunks[1]);

    let footer =
        Paragraph::new("Enter: Refresh AI report  |  Up/Down or j/k: Scroll  |  Esc: Back to Home")
            .style(Style::default().fg(p.muted))
            .block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, chunks[2]);
}
