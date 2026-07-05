mod app;
mod event;
mod resume;
mod ui;

use std::io::{stdout, Write};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::{Terminal, backend::CrosstermBackend};
use anyhow::Result;

use crate::app::App;
use crate::event::handle_action;

fn main() -> Result<()> {
    let resume = resume::load("resume.toml")?;
    
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(resume);

    let _guard = TerminalGuard;

    loop {
        terminal.draw(|f| {
            let area = f.area();
            ui::render(&app, f, area);
        })?;
        
        match handle_action()? {
            Some(action) => {
                app.update(action);
                if !app.running {
                    break;
                }
            }
            None => continue, // Ignore unrecognized keys
        }
    }

    // Terminal is restored when `_guard` goes out of scope.
    Ok(())
}

/// Drop guard that restores the terminal when it goes out of scope.
/// This ensures restoration even if a panic occurs.
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Ignore errors on cleanup
        let _ = stdout().execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
        let _ = stdout().flush();
    }
}