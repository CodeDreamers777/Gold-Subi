use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::App,
    ui::util::{BACKGROUND_COLOR, BORDER_COLOR, HIGHLIGHT_COLOR, PRIMARY_COLOR, TEXT_COLOR},
};

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    let trades = &app.trades_list.items;
    
    // Compute stats
    let total_trades = trades.len() as f64;
    let mut wins = 0.0;
    let mut _losses = 0.0;
    let mut gross_profit = 0.0;
    let mut gross_loss = 0.0;
    let mut net_pnl = 0.0;

    // For equity curve
    let mut equity_data = vec![(0.0, 0.0)];
    let mut current_equity = 0.0;
    
    // Sort trades chronologically for the equity curve
    let mut sorted_trades = trades.clone();
    sorted_trades.sort_by(|a, b| a.date.cmp(&b.date));

    for (i, trade) in sorted_trades.iter().enumerate() {
        let pnl = trade.pnl.unwrap_or(0.0);
        if pnl > 0.0 {
            wins += 1.0;
            gross_profit += pnl;
        } else if pnl < 0.0 {
            _losses += 1.0;
            gross_loss += pnl.abs();
        }
        
        current_equity += pnl;
        equity_data.push(((i + 1) as f64, current_equity));
        net_pnl += pnl;
    }

    let win_rate = if total_trades > 0.0 { (wins / total_trades) * 100.0 } else { 0.0 };
    let profit_factor = if gross_loss > 0.0 { gross_profit / gross_loss } else { gross_profit };
    
    let stats_text = format!(
        "Total Trades: {}\nWin Rate: {:.1}%\nGross Profit: ${:.2}\nGross Loss: ${:.2}\nNet PnL: ${:.2}\nProfit Factor: {:.2}",
        total_trades, win_rate, gross_profit, gross_loss, net_pnl, profit_factor
    );

    let pnl_color = if net_pnl >= 0.0 { Color::Green } else { Color::Red };

    let summary = Paragraph::new(stats_text)
        .block(Block::default().title("Performance Summary").borders(Borders::ALL).border_style(Style::default().fg(BORDER_COLOR)))
        .style(Style::default().fg(TEXT_COLOR));
    
    f.render_widget(summary, chunks[0]);

    // Equity Curve Chart
    let min_equity = equity_data.iter().map(|&(_, y)| y).fold(f64::INFINITY, f64::min).min(0.0);
    let max_equity = equity_data.iter().map(|&(_, y)| y).fold(f64::NEG_INFINITY, f64::max).max(0.0);
    let max_x = equity_data.len() as f64;

    let datasets = vec![
        Dataset::default()
            .name("Equity")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(pnl_color))
            .data(&equity_data),
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Equity Curve ($)").borders(Borders::ALL).border_style(Style::default().fg(BORDER_COLOR)))
        .x_axis(
            Axis::default()
                .title("Trades")
                .style(Style::default().fg(TEXT_COLOR))
                .bounds([0.0, max_x])
                .labels(vec![Span::raw("0"), Span::raw(format!("{}", max_x))]),
        )
        .y_axis(
            Axis::default()
                .title("PnL")
                .style(Style::default().fg(TEXT_COLOR))
                .bounds([min_equity, max_equity])
                .labels(vec![
                    Span::raw(format!("{:.0}", min_equity)),
                    Span::raw(format!("{:.0}", (min_equity + max_equity) / 2.0)),
                    Span::raw(format!("{:.0}", max_equity)),
                ]),
        );

    f.render_widget(chart, chunks[1]);
}
