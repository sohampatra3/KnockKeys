use crate::app::App;
use crate::engine::metrics;
use crate::ui::Palette;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Sparkline};
use ratatui::Frame;

pub fn render_config(frame: &mut Frame<'_>, app: &App, p: Palette) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title = Paragraph::new("Typing Test Configuration")
        .style(Style::default().fg(p.accent).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    let rows = vec![
        format!("Duration: {}", app.test_config.duration.title()),
        format!("Text type: {}", app.test_config.text_type.title()),
        "Start test".to_string(),
        "Back".to_string(),
    ];

    let items: Vec<ListItem> = rows
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            let selected = idx == app.test_config_index;
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
                .title(" Test Setup ")
                .borders(Borders::ALL)
                .style(Style::default().fg(p.text)),
        ),
        chunks[1],
    );

    let help = Paragraph::new("Left/Right: adjust value  |  Enter: select  |  Esc: back")
        .style(Style::default().fg(p.muted))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(help, chunks[2]);
}

pub fn render_summary(frame: &mut Frame<'_>, app: &App, p: Palette) {
    let Some(summary) = &app.summary else {
        let fallback =
            Paragraph::new("No session summary").block(Block::default().borders(Borders::ALL));
        frame.render_widget(fallback, frame.size());
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let header = Paragraph::new(format!("Session Complete - {}", summary.mode.title()))
        .style(Style::default().fg(p.accent).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    let stat_lines = vec![
        Line::from(vec![
            Span::styled("WPM: ", Style::default().fg(p.muted)),
            Span::styled(
                format!("{:.1}", summary.wpm),
                Style::default().fg(p.accent).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Accuracy: ", Style::default().fg(p.muted)),
            Span::styled(
                format!("{:.1}%", summary.accuracy),
                Style::default().fg(p.success),
            ),
        ]),
        Line::from(vec![
            Span::styled("Mistakes: ", Style::default().fg(p.muted)),
            Span::styled(
                format!("{}", summary.mistakes),
                Style::default().fg(p.danger),
            ),
        ]),
        Line::from(vec![
            Span::styled("Duration: ", Style::default().fg(p.muted)),
            Span::styled(
                format!("{}s", summary.duration_secs),
                Style::default().fg(p.text),
            ),
        ]),
    ];

    let stats = Paragraph::new(stat_lines).block(
        Block::default()
            .title(" Results ")
            .borders(Borders::ALL)
            .style(Style::default().fg(p.text)),
    );
    frame.render_widget(stats, chunks[1]);

    let problems = if summary.problematic_keys.is_empty() {
        "None".to_string()
    } else {
        summary
            .problematic_keys
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    };

    let unlock = summary
        .unlocked_new_key
        .map(|k| format!("Unlocked new key: {k}"))
        .unwrap_or_else(|| "No new key unlocked this session".to_string());

    let recap = Paragraph::new(format!(
        "Most problematic keys: {}\n{}\n{}\nLesson recap: accuracy drives speed. Keep shoulders relaxed and return to home row.",
        problems,
        unlock,
        if let Some(lang) = summary.code_language {
            format!(
                "Code mode: {} / {}",
                lang.title(),
                summary
                    .code_topic
                    .map(|t| t.title().to_string())
                    .unwrap_or_else(|| "Mixed".to_string())
            )
        } else {
            "Mode details: General typing".to_string()
        }
    ))
    .block(Block::default().title(" Recap ").borders(Borders::ALL));
    frame.render_widget(recap, chunks[2]);

    let points = metrics::sparkline_points(&summary.wpm_samples, chunks[3].width as usize - 2);
    let graph = Sparkline::default()
        .block(
            Block::default()
                .title(" WPM Over Time ")
                .borders(Borders::ALL),
        )
        .data(&points)
        .style(Style::default().fg(p.accent));
    frame.render_widget(graph, chunks[3]);

    let footer = Paragraph::new("Enter/Esc: Back to home")
        .style(Style::default().fg(p.muted))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, chunks[4]);
}
