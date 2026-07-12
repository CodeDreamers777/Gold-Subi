// src/ui/entries_tab.rs
use chrono::NaiveDate;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap, Padding},
    Frame,
};
use crate::app::App;
use crate::ui::util::{PRIMARY_COLOR, BORDER_COLOR, HIGHLIGHT_COLOR, TEXT_COLOR, SUBTLE_TEXT};

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    
    // Create inner layout for the entries tab
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);
    
    let items: Vec<ListItem> = app
        .entries_list
        .items
        .iter()
        .map(|e| {
            let mut style = Style::default().fg(TEXT_COLOR);
            if e.date == today {
                style = Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD);
            }
            
            // Use formatted date display for entries list
            let date_display = if let Ok(parsed_date) = NaiveDate::parse_from_str(&e.date, "%Y-%m-%d") {
                parsed_date.format("%b %d, %Y").to_string()
            } else {
                e.date.clone()
            };
            let is_selected = app.entries_list.state.selected().map_or(false, |idx| {
                if let Some(entry) = app.entries_list.items.get(idx) {
                    entry.date == e.date
                } else {
                    false
                }
            });

            let prefix = if is_selected { "▶ " } else { "• " };
            
            ListItem::new(Line::from(vec![
                Span::styled(prefix, style.clone()),
                Span::styled(date_display, style),
                Span::styled(
                    format!(" ({} bytes)", e.size),
                    Style::default().fg(SUBTLE_TEXT),
                ),
            ]))
        })
        .collect();
    
    let entries_list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled(
                    "Journal Entries",
                    Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(BORDER_COLOR))
        )
        .highlight_style(
            Style::default()
                .bg(HIGHLIGHT_COLOR)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
    
    f.render_stateful_widget(entries_list, chunks[0], &mut app.entries_list.state);
    
    // Preview content
    let selected = app.entries_list.state.selected();
    let mut word_count = 0;
    let mut char_count = 0;
    
    let content = match selected {
        Some(i) if !app.entries_list.items.is_empty() => {
            let entry = &app.entries_list.items[i];
            
            // Calculate stats
            char_count = entry.content.chars().count();
            word_count = entry.content.split_whitespace().count();
            
            // Basic Markdown Syntax Highlighting
            let mut lines = Vec::new();
            for line in entry.content.lines() {
                if line.starts_with("# ") {
                    lines.push(Line::from(Span::styled(
                        line.to_string(),
                        Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)
                    )));
                } else if line.starts_with("## ") {
                    lines.push(Line::from(Span::styled(
                        line.to_string(),
                        Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD)
                    )));
                } else if line.starts_with("### ") {
                    lines.push(Line::from(Span::styled(
                        line.to_string(),
                        Style::default().fg(crate::ui::util::ACCENT_COLOR).add_modifier(Modifier::BOLD)
                    )));
                } else if line.starts_with("- ") || line.starts_with("* ") {
                    lines.push(Line::from(Span::styled(
                        line.to_string(),
                        Style::default().fg(crate::ui::util::SECONDARY_TEXT)
                    )));
                } else if line.starts_with("> ") {
                    lines.push(Line::from(Span::styled(
                        line.to_string(),
                        Style::default().fg(SUBTLE_TEXT).add_modifier(Modifier::ITALIC)
                    )));
                } else {
                    lines.push(Line::from(Span::styled(
                        line.to_string(),
                        Style::default().fg(TEXT_COLOR)
                    )));
                }
            }
            Text::from(lines)
        }
        _ => Text::from(Span::styled(
            "Select an entry to view its content",
            Style::default().fg(SUBTLE_TEXT).add_modifier(Modifier::ITALIC)
        )),
    };
    
    let preview_title = if word_count > 0 {
        Line::from(vec![
            Span::styled(" Preview ", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)),
            Span::styled(format!("({} words, {} chars) ", word_count, char_count), Style::default().fg(SUBTLE_TEXT).add_modifier(Modifier::ITALIC)),
        ])
    } else {
        Line::from(Span::styled(" Preview ", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)))
    };
    
    let preview = Paragraph::new(content)
        .block(
            Block::default()
                .title(preview_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(BORDER_COLOR))
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: false });
    
    f.render_widget(preview, chunks[1]);
}
