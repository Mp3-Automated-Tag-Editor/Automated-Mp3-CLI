use ratatui::widgets::Widget;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Paragraph};
use crate::tabs::tab_renderer::TabRenderer;
use crate::tabs::SelectedTab;
use crate::App;

pub struct EditTab;

impl TabRenderer for EditTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        Paragraph::new("Edit tab content goes here.")
            .block(SelectedTab::Edit.block())
            .render(area, buf);
    }
}