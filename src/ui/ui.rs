// src/ui/ui.rs
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};
use crate::{
    app::App,
    ui::{calendar_tab, entries_tab, help, settings_tab},
    ui::util::{PRIMARY_COLOR, BORDER_COLOR, BACKGROUND_COLOR, HIGHLIGHT_COLOR, TEXT_COLOR, SUBTLE_TEXT},
};

pub fn ui(f: &mut Frame, app: &mut App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Top bar (Title + Tabs)
            Constraint::Min(0),     // Main content
            Constraint::Length(1),  // Status bar
        ])
        .split(f.area());

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Title
            Constraint::Percentage(40), // Tabs
            Constraint::Percentage(30), // Clock
        ])
        .split(chunks[0]);

    // App Title
    let title = Paragraph::new(Line::from(vec![
        Span::styled(" 🦀 Rusty", Style::default().fg(crate::ui::util::ACCENT_COLOR).add_modifier(Modifier::BOLD)),
        Span::styled("Notes ", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)),
    ]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM | Borders::RIGHT)
            .border_style(Style::default().fg(BORDER_COLOR))
    )
    .alignment(Alignment::Left);
    f.render_widget(title, header_chunks[0]);

    // Render tabs
    let titles: Vec<Line> = ["Entries", "Calendar", "Settings"]
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(HIGHLIGHT_COLOR)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(rest, Style::default().fg(TEXT_COLOR)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(BORDER_COLOR))
        )
        .select(app.tab_index)
        .style(Style::default().fg(TEXT_COLOR))
        .highlight_style(
            Style::default()
                .fg(PRIMARY_COLOR)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, header_chunks[1]);

    // Realtime Clock & Greeting
    let now = chrono::Local::now();
    let hour = now.format("%H").to_string().parse::<u32>().unwrap_or(12);
    let greeting = if hour < 12 {
        "Good Morning 🌅"
    } else if hour < 18 {
        "Good Afternoon ☀️"
    } else {
        "Good Evening 🌙"
    };

    let clock_text = format!("{} | {} ", greeting, now.format("%I:%M:%S %p"));
    let clock = Paragraph::new(Line::from(Span::styled(
        clock_text,
        Style::default().fg(crate::ui::util::SECONDARY_TEXT).add_modifier(Modifier::ITALIC),
    )))
    .block(
        Block::default()
            .borders(Borders::BOTTOM | Borders::LEFT)
            .border_style(Style::default().fg(BORDER_COLOR))
    )
    .alignment(Alignment::Right);
    
    f.render_widget(clock, header_chunks[2]);

    match app.tab_index {
        0 => entries_tab::render(f, app, chunks[1]),
        1 => calendar_tab::render(f, app, chunks[1]),
        2 => settings_tab::render(f, app, chunks[1]),
        _ => unreachable!(),
    }

    // Render status bar
    let status = Line::from(vec![
        Span::raw(" "),
        if !app.status_message.is_empty() {
            Span::styled(&app.status_message, Style::default().fg(PRIMARY_COLOR))
        } else {
            Span::styled(
                format!("Press 'h' for help | {} entries", app.entries_list.items.len()),
                Style::default().fg(SUBTLE_TEXT),
            )
        },
    ]);

    let status_bar = Paragraph::new(status)
        .style(Style::default().bg(BACKGROUND_COLOR))
        .alignment(Alignment::Left);

    f.render_widget(status_bar, chunks[2]);

    // Render help overlay if requested
    if app.show_help {
        help::render(f, f.area());
    }
}
