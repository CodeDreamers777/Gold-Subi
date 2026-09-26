// src/ui/settings_tab.rs
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{app::App, config::settings::get_journal_dir};

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let settings_text = Text::from(vec![
        Line::from(vec![
            Span::styled(
                "🔧 Settings",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Current Editor: ", Style::default().fg(Color::Yellow)),
            Span::styled(&app.editor, Style::default()),
        ]),
        Line::from(""),
        Line::from("Personal entries are stored in:"),
        Line::from(format!("{}", crate::config::settings::get_journal_dir().display())),
        Line::from(""),
        Line::from("Trade entries are stored in:"),
        Line::from(format!("{}", crate::config::settings::get_trades_dir().display())),
    ]);

    let settings = Paragraph::new(settings_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(settings, area);
}
