use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::{event, terminal};
use std::io;
use std::ops::ControlFlow;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::{Frame, Terminal};

pub mod checklist;
pub mod components;
pub mod dialog;
mod layout;

pub type TerminalBackend = CrosstermBackend<io::Stdout>;
pub type TerminalFrame<'a> = Frame<'a, TerminalBackend>;

pub struct TUI {
    terminal: Terminal<TerminalBackend>,
    cleared: bool,
}

#[derive(Default, Debug, Clone)]
pub struct FrameEvent {
    pub keys: Vec<KeyEvent>,
}

impl TUI {
    pub fn start() -> Result<Self, io::Error> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();

        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture
        )?;

        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;

        Ok(TUI {
            terminal,
            cleared: false,
        })
    }

    pub fn draw(&mut self, drawer: impl FnOnce(&mut TerminalFrame)) -> Result<(), io::Error> {
        self.terminal.draw(drawer).map(|_| ())
    }

    pub fn wait(&mut self, rate: f64) -> Result<FrameEvent, io::Error> {
        let timeout = Duration::from_millis((1.0 / (rate as f32) * 1000.0).round() as u64);

        let mut frame_event = FrameEvent::default();
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                frame_event.keys.push(key);
            }
        }

        Ok(frame_event)
    }

    pub fn finish(mut self) -> Result<(), io::Error> {
        self.finish_in_borrow()
    }

    fn finish_in_borrow(&mut self) -> Result<(), io::Error> {
        terminal::disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        self.cleared = true;
        Ok(())
    }
}

impl Drop for TUI {
    fn drop(&mut self) {
        if self.cleared {
            return;
        }

        self.finish_in_borrow()
            .expect("Could not automatically finish TUI in drop call");

        #[cfg(debug_assertions)]
        {
            eprintln!("********************");
            eprintln!(
                "[!] TUI was not finished before TUI drops. You must call finish() before dropping"
            );
            eprintln!("********************");
        }
    }
}

impl FrameEvent {
    pub fn is_pressed_with_mod(&self, code: KeyCode, mods: KeyModifiers) -> bool {
        self.keys
            .iter()
            .any(|event| event.code == code && event.modifiers == mods)
    }

    pub fn is_pressed(&self, code: KeyCode) -> bool {
        self.keys.iter().any(|event| event.code == code)
    }
}
