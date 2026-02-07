use crate::app::App;
use crate::ui::Palette;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_code_config(frame: &mut Frame<'_>, app: &App, p: Palette) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title = Paragraph::new("Code Drill Configuration")
        .style(Style::default().fg(p.accent).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(chunks[1]);

    let rows = vec![
        format!("Language: {}", app.code_config.language.title()),
        format!("Topic: {}", app.code_config.topic.title()),
        "Start code drill".to_string(),
        "Back".to_string(),
    ];

    let items: Vec<ListItem> = rows
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            let selected = idx == app.code_config_index;
            let style = if selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(p.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(p.text)
            };
            ListItem::new(Line::from(Span::styled(format!("  {row}"), style)))
        })
        .collect();

    frame.render_widget(
        List::new(items).block(
            Block::default()
                .title(" Code Mode ")
                .borders(Borders::ALL)
                .style(Style::default().fg(p.text)),
        ),
        body[0],
    );

    let lang_key = app.code_config.language.key();
    let lang_stats = app.progress.language_stats.get(lang_key);

    let detail = if let Some(stat) = lang_stats {
        format!(
            "Language speed tracker\n\nLanguage: {}\nSessions: {}\nAvg WPM: {:.1}\nBest WPM: {:.1}\nTotal chars: {}\nTotal time: {} min\n\nTopic: {}\n\nPractice snippets include loops, if/else, sorting, and searching patterns.",
            app.code_config.language.title(),
            stat.sessions,
            stat.avg_wpm(),
            stat.best_wpm,
            stat.total_chars,
            stat.total_secs / 60,
            app.code_config.topic.title()
        )
    } else {
        format!(
            "Language speed tracker\n\nNo sessions yet for {}.\n\nTopic: {}\n\nPractice snippets include loops, if/else, sorting, and searching patterns.",
            app.code_config.language.title(),
            app.code_config.topic.title()
        )
    };

    let right = Paragraph::new(detail).wrap(Wrap { trim: true }).block(
        Block::default()
            .title(" Selected Language Stats ")
            .borders(Borders::ALL)
            .style(Style::default().fg(p.text)),
    );
    frame.render_widget(right, body[1]);

    let footer = Paragraph::new("Left/Right: adjust  |  Enter: select  |  Esc: back")
        .style(Style::default().fg(p.muted))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, chunks[2]);
}
