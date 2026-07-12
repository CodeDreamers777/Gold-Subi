use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::util::{
    PRIMARY_COLOR, BORDER_COLOR, ACCENT_COLOR, TEXT_COLOR,
};

pub fn render(f: &mut Frame, area: Rect) {
    let help_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);

    let title_block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(PRIMARY_COLOR));

    let title_text = Paragraph::new("Keyboard Shortcuts").block(title_block.clone());

    f.render_widget(title_text, help_layout[0]);
    f.render_widget(title_block, help_layout[0]);

    // Render the rest of the help content here
    let help_lines = vec![
        create_key_line("q", "", "Quit the application"),
        create_key_line("←/→", "", "Navigate between tabs"),
        create_key_line("Enter", "", "Open selected entry"),
        create_key_line("Esc", "", "Close modals or go back"),
        create_key_line("n", "", "Create a new entry"),
        create_key_line("d", "", "Delete selected entry"),
        create_key_line("e", "", "Edit selected entry"),
        create_key_line("j/k", "", "Move down/up in list"),
    ];

    let help_paragraph = Paragraph::new(help_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Shortcuts")
                .border_style(Style::default().fg(BORDER_COLOR)),
        )
        .style(Style::default().fg(TEXT_COLOR));

    f.render_widget(help_paragraph, help_layout[1]);
}

pub fn create_key_line<'a>(key: &'a str, alt_key: &'a str, description: &'a str) -> Line<'a> {
    let mut spans = vec![
        Span::styled(
            format!(" {} ", key),
            Style::default()
                .fg(Color::Black)
                .bg(ACCENT_COLOR)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    if !alt_key.is_empty() {
        spans.push(Span::raw(" or "));
        spans.push(Span::styled(
            format!(" {} ", alt_key),
            Style::default()
                .fg(Color::Black)
                .bg(ACCENT_COLOR)
                .add_modifier(Modifier::BOLD),
        ));
    }

    spans.push(Span::raw(" "));
    spans.push(Span::styled(description, Style::default().fg(TEXT_COLOR)));

    Line::from(spans)
}
