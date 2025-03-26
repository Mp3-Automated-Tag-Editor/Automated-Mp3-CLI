use ratatui::widgets::Widget;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Paragraph};
use crate::tabs::tab_renderer::TabRenderer;
use crate::tabs::SelectedTab;

pub struct PlayTab;

impl TabRenderer for PlayTab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Play tab content goes here.")
            .block(SelectedTab::Play.block())
            .render(area, buf);
    }
}