use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ::elasticsearch::http::transport::SingleNodeConnectionPool;
use ::elasticsearch::Elasticsearch;
use ratatui::prelude::*;
use std::{env, io, time::{Duration, Instant}};
use url::Url;

mod app;
mod elasticsearch;
mod tracing;
mod ui;

use crate::app::App;
use crate::tracing::initialize_logging;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Elasticsearch client
    let client = create_elasticsearch_client()?;

    initialize_logging().unwrap_or_default();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new(client);
    let res = run_app(&mut terminal, app, tick_rate).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{}", err);
    }

    Ok(())
}

fn create_elasticsearch_client() -> Result<Elasticsearch> {
    let url = Url::parse(env::var("ELASTICSEARCH_URL").unwrap_or_default().as_str()).unwrap_or("http://localhost:9200".parse()?);
    let cp = SingleNodeConnectionPool::new(url);
    let transport = ::elasticsearch::http::transport::TransportBuilder::new(cp).build();
    let client = Elasticsearch::new(transport?);
    Ok(client)
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();
    app.on_tick().await?;
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('o') => app.selected_tab = 0,
                    KeyCode::Char('n') => app.selected_tab = 1,
                    KeyCode::Char('i') => app.selected_tab = 2,
                    KeyCode::Left => app.on_left(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Down => app.on_down(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick().await?;
            last_tick = Instant::now();
        }
    }
}
