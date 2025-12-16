mod app;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::{Duration, Instant}};
use crate::app::App;
use crate::ui::ui;

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = Instant::now();

    let mut last_input_time = Instant::now();
    let debounce_rate = Duration::from_millis(50);

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            let event = event::read()?;

            if last_input_time.elapsed() >= debounce_rate {
                match event {
                    Event::Key(key) => {
                        match key.code {
                            KeyCode::Char('q') => app.running = false,
                            KeyCode::Down => app.next(),
                            KeyCode::Up => app.previous(),
                            KeyCode::Char('k') => app.kill_selected_process(),
                            KeyCode::Char('s') => app.toggle_sort(),
                            _ => {}
                        }
                        last_input_time = Instant::now();
                    },

                    Event::Mouse(mouse) => {
                        match mouse.kind {
                            MouseEventKind::ScrollDown => app.next(),
                            MouseEventKind::ScrollUp => app.previous(),
                            _ => {}
                        }
                        last_input_time = Instant::now();
                    }

                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.running {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, event::DisableMouseCapture)?;
    Ok(())
}