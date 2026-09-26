use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Table, Paragraph},
    Frame,
};

use crate::{
    app::App,
    ui::util::{BACKGROUND_COLOR, BORDER_COLOR, HIGHLIGHT_COLOR, TEXT_COLOR},
};

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    let trades = &app.trades_list.items;
    
    let header_cells = ["Date", "Pair", "Dir", "Entry", "Exit", "PnL", "Setup"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(BACKGROUND_COLOR))
        .height(1)
        .bottom_margin(1);

    let rows = trades.iter().map(|item| {
        let pnl = item.pnl.unwrap_or(0.0);
        let pnl_style = if pnl > 0.0 {
            Style::default().fg(Color::Green)
        } else if pnl < 0.0 {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(TEXT_COLOR)
        };

        let exit_str = if let Some(e) = item.exit_price { format!("{:.4}", e) } else { "Open".to_string() };
        
        let cells = vec![
            Cell::from(item.date.clone()),
            Cell::from(item.pair.clone()),
            Cell::from(item.direction.clone()),
            Cell::from(format!("{:.4}", item.entry_price)),
            Cell::from(exit_str),
            Cell::from(format!("{:.2}", pnl)).style(pnl_style),
            Cell::from(item.setup.clone()),
        ];
        Row::new(cells).height(1)
    });

    let t = Table::new(rows, [
        Constraint::Length(20),
        Constraint::Length(10),
        Constraint::Length(6),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Min(10),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Trade History").border_style(Style::default().fg(BORDER_COLOR)))
    .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

    f.render_stateful_widget(t, chunks[0], &mut app.trades_list.state);

    // Trade Details panel
    let detail_text = if let Some(selected) = app.trades_list.state.selected() {
        if !trades.is_empty() {
            let trade = &trades[selected];
            format!(
                "Pair: {}\nDirection: {}\nEntry: {:.4}\nSL: {:?}\nTP: {:?}\nExit: {:?}\nLot Size: {:.2}\nPnL: {:?}\nSetup: {}\nSession: {}\n\nNotes:\n{}",
                trade.pair, trade.direction, trade.entry_price, trade.stop_loss, trade.take_profit, trade.exit_price, trade.lot_size, trade.pnl, trade.setup, trade.session, trade.notes
            )
        } else {
            "No trades found.".to_string()
        }
    } else {
        "Select a trade to view details".to_string()
    };

    let detail = Paragraph::new(detail_text)
        .block(Block::default().borders(Borders::ALL).title("Trade Details").border_style(Style::default().fg(BORDER_COLOR)))
        .style(Style::default().fg(TEXT_COLOR));

    f.render_widget(detail, chunks[1]);
}
