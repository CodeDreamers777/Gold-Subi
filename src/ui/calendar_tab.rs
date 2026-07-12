// src/ui/calendar_tab.rs
use chrono::{Datelike, NaiveDate};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::util::{BACKGROUND_COLOR, BORDER_COLOR, HIGHLIGHT_COLOR, PRIMARY_COLOR, SECONDARY_TEXT, TEXT_COLOR};

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let now = chrono::Local::now();
    let current_date = now.naive_local().date();
    let year = current_date.year();
    let month = current_date.month();
    
    // Get the first day of the month
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let start_weekday = first_day.weekday().num_days_from_sunday();
    
    let mut days_in_month = 31;
    for i in 28..=31 {
        if NaiveDate::from_ymd_opt(year, month, i + 1).is_none() {
            days_in_month = i;
            break;
        }
    }
    
    let mut calendar_lines = vec![
        Line::from(vec![
            Span::styled(
                now.format("%B %Y").to_string(),
                Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            " Su  Mo  Tu  We  Th  Fr  Sa ",
            Style::default().fg(SECONDARY_TEXT)
        )),
        Line::from(Span::styled(
            "────────────────────────────",
            Style::default().fg(BORDER_COLOR)
        )),
    ];
    
    let mut current_line = vec![];
    for _ in 0..start_weekday {
        current_line.push(Span::raw("    "));
    }
    
    for day in 1..=days_in_month {
        let date_str = format!("{:2}  ", day);
        if day == current_date.day() {
            current_line.push(Span::styled(
                date_str,
                Style::default().bg(PRIMARY_COLOR).fg(BACKGROUND_COLOR).add_modifier(Modifier::BOLD),
            ));
        } else {
            // Check if there's an entry for this day
            let has_entry = app.entries_list.items.iter().any(|e| {
                if let Ok(d) = NaiveDate::parse_from_str(&e.date, "%Y-%m-%d") {
                    d.year() == year && d.month() == month && d.day() == day
                } else {
                    false
                }
            });
            
            if has_entry {
                current_line.push(Span::styled(
                    date_str,
                    Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD),
                ));
            } else {
                current_line.push(Span::styled(
                    date_str,
                    Style::default().fg(TEXT_COLOR),
                ));
            }
        }
        
        if (start_weekday + day) % 7 == 0 || day == days_in_month {
            calendar_lines.push(Line::from(current_line.clone()));
            current_line.clear();
        }
    }
    
    // Add legend
    calendar_lines.push(Line::from(""));
    calendar_lines.push(Line::from(""));
    calendar_lines.push(Line::from(vec![
        Span::styled("■ ", Style::default().fg(PRIMARY_COLOR)),
        Span::raw("Today   "),
        Span::styled("■ ", Style::default().fg(HIGHLIGHT_COLOR)),
        Span::raw("Has Entry"),
    ]));
    
    let calendar = Paragraph::new(Text::from(calendar_lines))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(Span::styled(" Calendar ", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(BORDER_COLOR)),
        );
    
    f.render_widget(calendar, area);
}
