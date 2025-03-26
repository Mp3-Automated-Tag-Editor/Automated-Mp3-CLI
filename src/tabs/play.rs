use ratatui::widgets::Widget;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Paragraph};
use crate::tabs::tab_renderer::TabRenderer;
use crate::tabs::SelectedTab;
use crate::App;

pub struct PlayTab;

impl TabRenderer for PlayTab {
    fn render(&self, area: Rect, buf: &mut Buffer, app: &App) {
        Paragraph::new("Play tab content goes here.")
            .block(SelectedTab::Play.block())
            .render(area, buf);
    }
}