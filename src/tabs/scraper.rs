use ratatui::widgets::Widget;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Paragraph};
use crate::tabs::tab_renderer::TabRenderer;
use crate::tabs::SelectedTab;

pub struct ScraperTab;

impl TabRenderer for ScraperTab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Scraper tab content goes here.")
            .block(SelectedTab::Scraper.block())
            .render(area, buf);
    }
}