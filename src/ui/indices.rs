use crate::app::App;
use humansize::DECIMAL;
use itertools::Itertools;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, Borders, Paragraph, Row, Table};
use ratatui::Frame;

pub fn draw_indices(f: &mut Frame, app: &App, area: Rect) {
    let stats = app.cluster_stats.as_ref();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Max(8), Constraint::Min(10)].as_ref())
        .split(area);

    // Cluster Status
    let status = Paragraph::new(vec![Line::from(vec![
        Span::raw("Status: "),
        Span::styled(
            stats.map(|s| s.status.as_str()).unwrap_or("N/A"),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
    ])])
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, chunks[0]);

    let total_indices = stats.map(|s| s.indices.count).unwrap_or(0).to_string();
    let docs_count = stats.map(|s| s.indices.docs.count).unwrap_or(0).to_string();
    let docs_deleted = stats.map(|s| s.indices.docs.deleted).unwrap_or(0).to_string();
    let store_size = stats.map(|s| s.indices.store.size_in_bytes).unwrap_or(0);
    let store_size_str = humansize::format_size(store_size as usize, DECIMAL);

    // Indices Metrics
    let metrics = Table::new(vec![
        Row::new(vec!["Total Indices", &total_indices]),
        Row::new(vec![
            "Total Documents",
            &docs_count,
        ]),
        Row::new(vec![
            "Deleted Documents",
            &docs_deleted,
        ]),
        Row::new(vec![
            "Store Size",
            &store_size_str,
        ]),
    ], [Constraint::Length(6)])
        .block(
            Block::default()
                .title("Indices Metrics")
                .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL),
        )
        .header(
            Row::new(vec!["Metric", "Value"])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1),
        )
        .widths([Constraint::Percentage(50), Constraint::Percentage(50)])
        .column_spacing(1)
        .row_highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(metrics, chunks[1]);


    // barchart of size per indices
    let indices_stats = app.indices_stats.as_ref();

    let indices: Vec<Bar> = indices_stats.map(|s| s.indices.iter().sorted().map(|(name, index)| {
        let size = index.total.store.total_data_set_size_in_bytes as f64;
        let health = index.health.as_str();
        let status = if index.status.as_str() == "open" { "🔓" } else { "🔒" };

        let style = match health {
            "green" => Style::default().fg(Color::Green),
            "yellow" => Style::default().fg(Color::Yellow),
            "red" => Style::default().fg(Color::Red),
            _ => Style::default().fg(Color::White),
        };

        Bar::default()
            .value(size as u64)
            .label(Line::from(format!("{} {}", name.as_str(), status)))
            .text_value(humansize::format_size(size as usize, DECIMAL))
            .style(Style::default().fg(style.fg.unwrap()))
            .value_style(Style::default().fg(Color::Yellow))
    }).collect()).unwrap_or_default();

    let largest_index_name = indices_stats
        .map(|s| {
            s.indices
                .iter()
                .max_by_key(|(name, _)| {
                    name.len()
                })
        }).map(|t| {
        if let Some((name, _)) = t {
            name.len()
        } else {
            0
        }
    }).unwrap_or_default();

    let indices_chart = BarChart::default()
        .block(
            Block::default()
                .title("Indices Size")
                .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
                .style(Style::default().fg(Color::Green))
                .borders(Borders::ALL),
        )
        .data(BarGroup::default().bars(&indices))
        .bar_gap(1)
        .bar_width(largest_index_name as u16 + 3)
        .value_style(Style::default().fg(Color::Yellow))
        .label_style(Style::default().fg(Color::Yellow))
        .bar_style(Style::default().fg(Color::Green));

    f.render_widget(indices_chart, chunks[2]);
}