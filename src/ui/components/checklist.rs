use crate::msg;
use crate::ui::dialog::Dialog;
use crate::ui::layout::centered_rect;
use crate::ui::{FrameEvent, TerminalBackend};
use crossterm::event::KeyCode;
use std::alloc::handle_alloc_error;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, List, ListItem};
use tui::Frame;

pub struct CheckListWidget {
    focused: usize,
    elements: Vec<CheckListElement>,
    state: State,
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    Selecting,
    ConfirmDialog,
    Confirmed,
}

pub struct CheckListElement {
    caption: String,
    marked: bool,
}

impl CheckListWidget {
    pub fn new(elements: &[String]) -> Self {
        Self {
            elements: elements
                .to_vec()
                .into_iter()
                .map(|caption| CheckListElement {
                    caption,
                    marked: false,
                })
                .collect(),
            focused: 0,
            state: State::Selecting,
        }
    }

    pub fn handle_event(&mut self, event: FrameEvent) {
        match self.state {
            State::Selecting => self.handle_selecting_event(event),
            State::ConfirmDialog => self.handle_dialog_event(event),
            State::Confirmed => { /* do nothing */ }
        }
    }

    fn handle_selecting_event(&mut self, event: FrameEvent) {
        if event.is_pressed(KeyCode::Up) || event.is_pressed(KeyCode::Char('k')) {
            self.focused = self.focused.saturating_sub(1);
        }

        if event.is_pressed(KeyCode::Down) || event.is_pressed(KeyCode::Char('j')) {
            if self.focused < (self.elements.len() - 1) {
                self.focused += 1;
            }
        }

        if event.is_pressed(KeyCode::Char(' ')) {
            let mut current_marked = &mut self.elements[self.focused].marked;
            *current_marked = !*current_marked;
        }

        if event.is_pressed(KeyCode::Enter) {
            if self.elements.iter().all(|elem| elem.marked) {
                self.state = State::Confirmed
            } else {
                self.state = State::ConfirmDialog
            }
        }
    }

    fn handle_dialog_event(&mut self, event: FrameEvent) {
        if event.is_pressed(KeyCode::Enter) {
            self.state = State::Confirmed;
        }

        if event.is_pressed(KeyCode::Esc) {
            self.state = State::Selecting;
        }
    }

    pub fn draw(
        &self,
        frame: &mut Frame<TerminalBackend>,
        parent_block: Block,
        global_rect: Rect,
        rect: Rect,
    ) {
        let mut list_elems = self
            .elements
            .iter()
            .enumerate()
            .map(|(i, elem)| elem.to_listitem(i == self.focused))
            .collect::<Vec<_>>();

        list_elems.push(ListItem::new(""));
        list_elems.push(ListItem::new(msg::enter_notify()));

        frame.render_widget(List::new(list_elems).block(parent_block), rect);

        if self.state == State::ConfirmDialog {
            Dialog::new(Span::from("Confirm"), Span::from(msg::enter_check()))
                .draw(frame, global_rect);
        }
    }

    pub fn confirmed(&self) -> bool {
        self.state == State::Confirmed
    }
}

impl CheckListElement {
    fn to_listitem(&self, focused: bool) -> ListItem {
        ListItem::new(Spans::from(vec![
            Span::from(if focused { " 》 " } else { "    " }),
            Span::from(if self.marked { "[✓] " } else { "[ ] " }),
            Span::from(self.caption.as_str()),
        ]))
            .style(
                Style::default()
                    .add_modifier(if focused {
                        Modifier::BOLD | Modifier::UNDERLINED
                    } else {
                        Modifier::empty()
                    })
                    .fg(if self.marked {
                        Color::LightGreen
                    } else {
                        Color::Reset
                    }),
            )
    }
}
