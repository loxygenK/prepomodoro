use crate::ui::layout::centered_rect;
use crate::ui::{FrameEvent, TerminalBackend};
use crossterm::event::KeyCode;
use std::iter;
use tui::layout::{Alignment, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

pub struct Dialog<'a, 'b> {
    title: Span<'a>,
    caption: Span<'b>,
    ok_selected: bool,
    confirmed: Option<bool>,
}

impl<'a, 'b> Dialog<'a, 'b> {
    pub fn new(title: Span<'a>, caption: Span<'b>) -> Self {
        Self {
            title,
            caption,
            ok_selected: false,
            confirmed: None,
        }
    }

    pub fn handle_event(&mut self, event: FrameEvent) {
        if event.is_pressed(KeyCode::Left) || event.is_pressed(KeyCode::Right) {
            self.ok_selected = !self.ok_selected
        }

        if event.is_pressed(KeyCode::Enter) {
            self.confirmed = Some(self.ok_selected);
        }
    }

    pub fn draw(&self, frame: &mut Frame<TerminalBackend>, global_rect: Rect) {
        if self.confirmed.is_some() {
            return;
        }

        let area = centered_rect(50, 30, global_rect);

        let popup_block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let mut content = iter::once(Spans::from("\n"))
            .cycle()
            .take((area.height / 2 - 1) as usize)
            .collect::<Vec<_>>();

        content.push(Spans::from(self.caption.clone()));

        let paragraph = Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(popup_block);

        frame.render_widget(paragraph, area);
    }
}
