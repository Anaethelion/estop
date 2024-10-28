use crate::app::App;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table, Widget};
use ratatui::Frame;
use ratatui::style::Modifier;
use crate::elasticsearch::cluster::_Nodes;

// Adapted from https://ratatui.rs/examples/widgets/custom_widget/
#[derive(Debug, Clone)]
struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: State,
}

impl Button<'_> {
    const fn colors(&self) -> (Color, Color, Color, Color) {
        let theme = self.theme;
        match self.state {
            State::Green => (theme.background, theme.text, theme.shadow, theme.highlight),
            State::Yellow => (theme.background, theme.text, theme.shadow, theme.highlight),
            State::Red => (theme.background, theme.text, theme.shadow, theme.highlight),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone, Copy)]
struct Theme {
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
}

const YELLOW: Theme = Theme {
    text: Color::Rgb(48, 48, 16),
    background: Color::Rgb(144, 144, 48),
    highlight: Color::Rgb(192, 192, 64),
    shadow: Color::Rgb(96, 96, 32),
};

const RED: Theme = Theme {
    text: Color::Rgb(48, 16, 16),
    background: Color::Rgb(144, 48, 48),
    highlight: Color::Rgb(192, 64, 64),
    shadow: Color::Rgb(96, 32, 32),
};

const GREEN: Theme = Theme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(48, 144, 48),
    highlight: Color::Rgb(64, 192, 64),
    shadow: Color::Rgb(32, 96, 32),
};

impl<'a> Button<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Self {
        Button {
            label: label.into(),
            theme: GREEN,
            state: State::Green,
        }
    }

    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub const fn state(mut self, state: State) -> Self {
        self.state = state;
        self
    }
}

impl<'a> Widget for Button<'a> {
    #[allow(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (background, text, shadow, highlight) = self.colors();
        buf.set_style(area, Style::new().bg(background).fg(text));

        // render top line if there's enough space
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                Style::new().fg(highlight).bg(background),
            );
        }
        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                Style::new().fg(shadow).bg(background),
            );
        }
        // render label centered
        buf.set_line(
            area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            &self.label,
            area.width,
        );
    }
}

pub fn draw_cluster(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Min(0), Constraint::Min(0)].as_ref())
        .split(area);

    // Cluster Name
    let name = app.cluster_stats.as_ref().map_or_else(
        || "N/A".to_string(),
        |stats| stats.cluster_name.clone(),
    );

    // Cluster UUID
    let uuid = app.cluster_stats.as_ref().map_or_else(
        || "N/A".to_string(),
        |stats| stats.cluster_uuid.clone(),
    );

    let nodes = app.cluster_stats.as_ref().map_or_else(
        _Nodes::default,
        |stats| stats._nodes.clone(),
    );

    let node_count = nodes.total.to_string();
    let node_failed = nodes.failed.to_string();
    let node_successful = nodes.successful.to_string();

    let table = Table::new(vec![
        Row::new(vec!["Cluster Name", name.as_str()]),
        Row::new(vec!["Cluster UUID", uuid.as_str()]),
        Row::new(vec!["", ""]),
        Row::new(vec!["Nodes", ""]),
        Row::new(vec!["", ""]),
        Row::new(vec!["Total", node_count.as_str()]),
        Row::new(vec!["Successful", node_successful.as_str()]),
        Row::new(vec!["Failed", node_failed.as_str()]),
    ], [Constraint::Length(2)])
        .block(Block::default().title("Cluster Info").borders(Borders::ALL).title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)))
        .widths([Constraint::Percentage(50), Constraint::Percentage(50)])
        .column_spacing(1)
        ;

    f.render_widget(table, chunks[0]);

    // list nodes with name, ip and status
    let table = Table::new(
        app.node_stats.iter().map(|(name, nodes)| {
            if let Some(node) = nodes.last() {
               Row::new(vec![
                   name.as_str(),
                   node.ip.as_str(),
               ])
            } else {
                Row::new(vec![
                    name.as_str(),
                    "N/A",
                ])
            }
        }),
        [Constraint::Length(2)]
    )
        .block(Block::default().title("Nodes").borders(Borders::ALL).title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)))
        .widths([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
        .column_spacing(1)
        ;

    f.render_widget(table, chunks[1]);

    // Cluster Status
    let status = app.cluster_stats.as_ref().map_or_else(
        || "N/A".to_string(),
        |stats| stats.status.clone(),
    );
    match status.as_str() {
        "green" => f.render_widget(Button::new(status).theme(GREEN).state(State::Green), chunks[2]),
        "yellow" => f.render_widget(Button::new(status).theme(YELLOW).state(State::Yellow), chunks[2]),
        "red" => f.render_widget(Button::new(status).theme(RED).state(State::Red), chunks[2]),
        _ => f.render_widget(Button::new(status).theme(GREEN).state(State::Green), chunks[2]),
    }

    let b = Paragraph::new("Cluster Status")
        .block(
            Block::new()
                .title("Cluster Health")
                .borders(Borders::ALL)
                .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        );

    f.render_widget(b, chunks[2]);
}