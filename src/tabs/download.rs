use ratatui::widgets::Widget;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Paragraph};
use crate::tabs::tab_renderer::TabRenderer;
use crate::tabs::SelectedTab;

pub struct DownloadTab;

impl TabRenderer for DownloadTab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Download tab content goes here.")
            .block(SelectedTab::Download.block())
            .render(area, buf);
    }
}