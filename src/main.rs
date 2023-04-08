use config::Configuration;
use crossterm::event::KeyCode::Modifier;
use crossterm::event::{KeyCode, KeyModifiers, ModifierKeyCode};
use std::io;
use std::ops::ControlFlow;

use crate::entity::Task;
use crate::ui::checklist::show_checklist;
use crate::ui::TUI;

pub mod config;
pub mod entity;
mod msg;
pub mod ui;

fn main() -> Result<(), io::Error> {
    let content = std::fs::read_to_string("./examples/work.yaml").unwrap();
    let parsed = serde_yaml::from_str::<Configuration>(&content).unwrap();

    dbg!(&parsed);

    let config: Vec<Task> = parsed.tasks.into_iter().map(Task::from).collect();

    dbg!(&config);

    dbg!(&config[0].segments.flatten());

    let mut tui = TUI::start().unwrap();

    show_checklist(&mut tui, &config[0], &config[0].checklist.before_start)?;
    tui.finish()?;

    Ok(())
}
