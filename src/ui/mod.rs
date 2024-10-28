use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style}
    ,
    text::{Line, Span},
    widgets::*,
    Frame,
};
use crate::app::App;
mod nodes;
mod cluster;
mod indices;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_body(f, app, chunks[1]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let titles = ["Overview", "Nodes", "Indices"].iter().map(|t| {
        let (first, rest) = t.split_at(1);
        Line::from(vec![
            Span::styled(
                first,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled(rest, Style::default().fg(Color::White)),
        ])
    });

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Elasticsearch Monitor"),
        )
        .select(app.selected_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn draw_body(f: &mut Frame, app: &App, area: Rect) {
    match app.selected_tab {
        0 => cluster::draw_cluster(f, app, area),
        1 => nodes::draw_nodes(f, app, area),
        2 => indices::draw_indices(f, app, area),
        _ => {}
    }
}

