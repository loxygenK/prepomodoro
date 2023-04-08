use crate::entity::{CheckList, CheckListElement, Task, TimeSegments};
use crate::msg;
use crate::ui::components::checklist::CheckListWidget;
use crate::ui::{TerminalFrame, TUI};
use crossterm::cursor::MoveDown;
use crossterm::event::{KeyCode, KeyModifiers};
use std::io;
use tui::layout::{Alignment, Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

pub fn show_checklist(
    tui: &mut TUI,
    task: &Task,
    checklist: &Vec<CheckListElement>,
) -> Result<(), io::Error> {
    let mut list = CheckListWidget::new(checklist);
    loop {
        let event = tui.wait(60.0)?;
        if event.is_pressed_with_mod(KeyCode::Char('c'), KeyModifiers::CONTROL) {
            break;
        }

        list.handle_event(event);

        if list.confirmed() {
            break;
        }

        tui.draw(|frame| {
            let chunks = Layout::default()
                .constraints([Constraint::Min(3), Constraint::Ratio(1, 1)])
                .margin(2)
                .split(frame.size());

            frame.render_widget(
                Paragraph::new(vec![
                    Spans::from(Span::styled(
                        format!("* {} *", task.name),
                        Style::default().fg(Color::LightYellow),
                    )),
                    Spans::from(msg::checklist_ready()),
                ])
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false })
                    .style(Style::default().add_modifier(Modifier::BOLD)),
                chunks[0],
            );

            let block = Block::default()
                .title(vec![
                    Span::from(msg::checklist_header()),
                    Span::from(" "),
                    Span::styled(
                        &task.name,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);

            list.draw(frame, block, frame.size(), chunks[1]);
        })?;
    }

    Ok(())
}
