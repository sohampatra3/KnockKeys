use crate::app::{
    App, AI_FIELD_API_KEY, AI_FIELD_BACK, AI_FIELD_BASE_URL, AI_FIELD_GENERATE, AI_FIELD_MODEL,
    AI_FIELD_PROVIDER, AI_FIELD_SCROLL,
};
use crate::ui::Palette;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
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

    let content = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(6)])
        .split(chunks[1]);

    let mut form_lines = vec![
        field_line(
            "Provider",
            format!("{}  (left/right to cycle)", app.ai_config.provider.title()),
            app.ai_field_index == AI_FIELD_PROVIDER,
            p,
        ),
        field_line(
            "API key",
            api_key_label(&app.ai_config.api_key),
            app.ai_field_index == AI_FIELD_API_KEY,
            p,
        ),
        field_line(
            "Base URL",
            app.ai_config.base_url.clone(),
            app.ai_field_index == AI_FIELD_BASE_URL,
            p,
        ),
        field_line(
            "Model",
            app.ai_config.model.clone(),
            app.ai_field_index == AI_FIELD_MODEL,
            p,
        ),
        field_line(
            "Report scroll",
            format!("offset {}  (left/right scroll)", app.ai_scroll),
            app.ai_field_index == AI_FIELD_SCROLL,
            p,
        ),
    ];
    form_lines.push(button_line(
        "Generate Report",
        app.ai_field_index == AI_FIELD_GENERATE,
        p,
    ));
    form_lines.push(button_line(
        "Back to Home",
        app.ai_field_index == AI_FIELD_BACK,
        p,
    ));

    let form = Paragraph::new(form_lines).wrap(Wrap { trim: true }).block(
        Block::default()
            .title(" AI Config ")
            .borders(Borders::ALL)
            .style(Style::default().fg(p.text)),
    );
    frame.render_widget(form, content[0]);

    let report = app.ai_report.as_deref().unwrap_or(
        "No report yet. Select your provider, paste an API key, then choose Generate Report.",
    );

    let lines: Vec<Line<'_>> = report.lines().map(Line::from).collect();
    let height_hint = content[1].height.saturating_sub(2) as usize;
    let max_scroll = lines.len().saturating_sub(height_hint.max(1));
    let scroll = app.ai_scroll.min(max_scroll) as u16;

    let report_widget = Paragraph::new(lines)
        .style(Style::default().fg(p.text))
        .block(
            Block::default()
                .title(" Full Analytics ")
                .borders(Borders::ALL)
                .border_style(if app.ai_field_index == AI_FIELD_SCROLL {
                    Style::default().fg(p.accent)
                } else {
                    Style::default().fg(p.muted)
                }),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(report_widget, content[1]);

    let footer = Paragraph::new(
        "Up/Down: select field  |  Left/Right: adjust  |  Type+Backspace: edit  |  Enter: activate  |  Esc: back",
    )
    .style(Style::default().fg(p.muted))
    .block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, chunks[2]);
}

fn field_line(label: &str, value: String, selected: bool, p: Palette) -> Line<'static> {
    if selected {
        let style = Style::default()
            .fg(Color::Black)
            .bg(p.accent)
            .add_modifier(Modifier::BOLD);
        return Line::from(Span::styled(format!("> {label}: {value}"), style));
    }

    Line::from(vec![
        Span::styled(format!("  {label}: "), Style::default().fg(p.muted)),
        Span::styled(value, Style::default().fg(p.text)),
    ])
}

fn button_line(label: &str, selected: bool, p: Palette) -> Line<'static> {
    if selected {
        let style = Style::default()
            .fg(Color::Black)
            .bg(p.accent)
            .add_modifier(Modifier::BOLD);
        return Line::from(Span::styled(format!("> [ {label} ]"), style));
    }
    Line::from(Span::styled(
        format!("  [ {label} ]"),
        Style::default().fg(p.text),
    ))
}

fn api_key_label(key: &str) -> String {
    let trimmed = key.trim();
    if trimmed.is_empty() {
        return "(empty - paste key here)".to_string();
    }

    let chars: Vec<char> = trimmed.chars().collect();
    let count = chars.len();
    if count <= 8 {
        return format!("{} ({} chars)", "*".repeat(count), count);
    }

    let head: String = chars.iter().take(4).collect();
    let tail: String = chars.iter().skip(count - 4).collect();
    let masked = format!("{head}{}{tail}", "*".repeat(count - 8));
    format!("{masked} ({count} chars)")
}
