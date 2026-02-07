mod app;
mod engine;
mod input;
mod models;
mod storage;
mod ui;

use anyhow::{Context, Result};
use app::App;
use clap::{Parser, Subcommand};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Write};
use std::panic;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use storage::persistence::{
    backup_progress, default_paths, export_sessions_csv, import_progress, load_state, save_state,
};

#[derive(Parser, Debug)]
#[command(name = "typetrainer")]
#[command(author, version, about = "Terminal typing trainer inspired by keybr")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Run,
    ExportCsv {
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    Backup {
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    Import {
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    install_panic_hook();
    let cli = Cli::parse();
    let paths = default_paths()?;

    match cli.command.unwrap_or(Command::Run) {
        Command::Run => run_tui(paths),
        Command::ExportCsv { out } => {
            let state = load_state(&paths)?;
            let file = export_sessions_csv(&paths, &state, out.as_deref())?;
            println!("Exported CSV to {}", file.display());
            Ok(())
        }
        Command::Backup { out } => {
            let state = load_state(&paths)?;
            let file = backup_progress(&paths, &state, out.as_deref())?;
            println!("Backup saved to {}", file.display());
            Ok(())
        }
        Command::Import { path } => {
            let state = import_progress(&paths, &path)?;
            save_state(&paths, &state)?;
            println!("Imported progress from {}", path.display());
            Ok(())
        }
    }
}

fn run_tui(paths: storage::persistence::StoragePaths) -> Result<()> {
    let state = load_state(&paths)?;

    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, Hide)
        .context("Failed to enter alternate screen")?;

    let guard = TerminalGuard;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal backend")?;
    terminal.clear().context("Failed to clear terminal")?;

    let mut app = App::new(state, paths);
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    while app.running {
        terminal.draw(|f| ui::render(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_millis(0));

        if event::poll(timeout).context("Event poll failed")? {
            match event::read().context("Event read failed")? {
                Event::Key(key) => app.handle_key_event(key),
                Event::Resize(_, _) => {}
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    drop(terminal);
    drop(guard);
    Ok(())
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = restore_terminal();
    }
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture, Show)?;
    stdout.flush()?;
    Ok(())
}

fn install_panic_hook() {
    let previous = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        previous(info);
    }));
}
