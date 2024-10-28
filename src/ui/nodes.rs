use ratatui::{symbols, Frame};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Span, Style};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, GraphType, List, Row, Table};
use std::ops::Index;
use humansize::DECIMAL;
use crate::app::App;

fn draw_charts(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // CPU Usage Chart
    let cpu_dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&app.cpu_usage_history);

    let cpu_chart = Chart::new(vec![cpu_dataset])
        .block(
            Block::default()
                .title(Span::styled(
                    "CPU Usage (%)",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().fg(Color::Cyan))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Time")
                .style(Style::default().fg(Color::Gray))
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .title("Usage")
                .style(Style::default().fg(Color::Gray))
                .labels(["0", "50", "100"])
                .bounds([0.0, 100.0]),
        );

    f.render_widget(cpu_chart, chunks[0]);

    // Memory Usage Chart
    let mem_dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Yellow))
        .data(&app.memory_usage_history);

    let mem_chart = Chart::new(vec![mem_dataset])
        .block(
            Block::default()
                .title(Span::styled(
                    "Memory Usage (%)",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().fg(Color::Yellow))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Time")
                .style(Style::default().fg(Color::Gray))
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .title("Usage")
                .style(Style::default().fg(Color::Gray))
                .labels(["0", "50", "100"])
                .bounds([0.0, 100.0]),
        );

    f.render_widget(mem_chart, chunks[1]);
}

pub fn draw_nodes(f: &mut Frame, app: &App, area: Rect) {
    let biggest_node_name = app
        .node_stats
        .iter()
        .max_by_key(|(name, _)| name.len())
        .map(|(name, _)| name.len())
        .unwrap_or(0);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(biggest_node_name as u16 + 6), Constraint::Min(0), Constraint::Max(biggest_node_name as u16 + 40)].as_ref())
        .split(area);

    let nodes_names: Vec<Span> = app
        .node_stats
        .keys()
        .enumerate()
        .map(|(idx, name)| {
            if idx == app.selected_node {
                Span::styled(
                    name,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw(name)
            }
        })
        .collect();

    // List of nodes names
    let list = List::new(nodes_names)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title_style(Style::default().fg(Color::White))
                .title("Nodes"),
        )
        .style(Style::default().fg(Color::Red))
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(list, chunks[0]);
    draw_charts(f, app, chunks[1]);

    // Node Metrics
    draw_infos(f, app, chunks[2]);
}

fn draw_infos(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Min(0)].as_ref())
        .split(area);

    let current_nodeinfo = app.node_stats.index(app.selected_node).last();
    let memory_usage = format!(
        "{}%",
        current_nodeinfo.map(|s| s.os.mem.used_percent)
            .unwrap_or(0)
    );
    let cpu_usage = format!("{}%", current_nodeinfo.map(|s| s.os.cpu.percent).unwrap_or(0));
    let ip_address = current_nodeinfo.map(|s| s.host.as_str()).unwrap_or("N/A").to_string();

    let metrics = Table::new(
        vec![
            Row::new(vec!["Node Name", app.node_stats.keys().nth(app.selected_node).unwrap()]),
            Row::new(vec!["Host", &ip_address]),
            Row::new(vec!["CPU Usage", &cpu_usage]),
            Row::new(vec!["Memory Usage", &memory_usage]),
        ],
        [Constraint::Length(4)],
    )
    .block(Block::default()
        .title("Node Metrics")
        .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL))
    .header(
        Row::new(vec!["Metric", "Value"])
            .style(Style::default().fg(Color::Yellow))
            .bottom_margin(1),
    )
    .widths([Constraint::Percentage(50), Constraint::Percentage(50)])
    .column_spacing(1)
    .row_highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

    f.render_widget(metrics, chunks[0]);
    draw_gauges(f, app, chunks[1]);
}

fn draw_gauges(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Min(0)].as_ref())
        .split(area);

    let current_nodeinfo = app.node_stats.index(app.selected_node).last();

    let disk_label = format!(
        "{} / {}",
        current_nodeinfo
            .map(|s| humansize::format_size(s.fs.total.available_in_bytes as usize, DECIMAL).to_string())
            .unwrap_or_else(|| "N/A".to_string()),
        current_nodeinfo
            .map(|s| humansize::format_size(s.fs.total.total_in_bytes as usize, DECIMAL).to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );

    let disk_gauge = Gauge::default()
        .block(Block::default().title("Disk Usage").borders(Borders::ALL).title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)))
        .label(disk_label)
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(current_nodeinfo.map(|s| s.fs.total.available_in_bytes*100/s.fs.total.total_in_bytes).unwrap_or(0) as f64 / 100.0);

    f.render_widget(disk_gauge, chunks[0]);

    let heap_label = format!(
        "{} / {}",
        current_nodeinfo
            .map(|s| humansize::format_size(s.jvm.mem.heap_used_in_bytes as usize, DECIMAL).to_string())
            .unwrap_or_else(|| "N/A".to_string()),
        current_nodeinfo
            .map(|s| humansize::format_size(s.jvm.mem.heap_max_in_bytes as usize, DECIMAL).to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );

    let heap_gauge = Gauge::default()
        .block(Block::default().title("Heap Usage").borders(Borders::ALL).title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)))
        .label(heap_label)
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(current_nodeinfo.map(|s| s.jvm.mem.heap_used_percent).unwrap_or(0) as f64 / 100.0);

    f.render_widget(heap_gauge, chunks[1]);
}
