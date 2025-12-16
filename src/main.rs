use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Row, Table},
};
use std::{io, time::Duration};
use sysinfo::System;

struct App {
    system: System,
    running: bool,
}

impl App {
    fn new() -> Self {
        Self {
            system: System::new_all(),
            running: true,
        }
    }

    fn on_tick(&mut self) {
        self.system.refresh_all();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    app.running = false;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = std::time::Instant::now();
        }

        if !app.running {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();

    let processes: Vec<Row> = app.system.processes().values().take(20).map(|p| {
        Row::new(vec![
            p.pid().to_string(),
            p.name().to_string(),
            format!("{:.1}%", p.cpu_usage()),
            format!("{} MB", p.memory() / 1024 / 1024),
        ])
    }).collect();

    let widths = [
        Constraint::Length(10),
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(30),
    ];

    let table = Table::new(processes, widths)
        .header(Row::new(vec!["PID", "Name", "CPU", "Memory"]).style(Style::default().fg(Color::Yellow)))
        .block(Block::default().title(" KiloMon - Rust Process Killer ").borders(Borders::ALL));

    f.render_widget(table, size);
}