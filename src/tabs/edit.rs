use ratatui::widgets::Widget;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Paragraph};
use crate::tabs::tab_renderer::TabRenderer;
use crate::tabs::SelectedTab;

pub struct EditTab;

impl TabRenderer for EditTab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Edit tab content goes here.")
            .block(SelectedTab::Edit.block())
            .render(area, buf);
    }
}