use ratatui::widgets::Widget;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Paragraph};
use crate::tabs::tab_renderer::TabRenderer;
use crate::tabs::SelectedTab;

pub struct SettingsTab;

impl TabRenderer for SettingsTab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Settings tab content goes here.")
            .block(SelectedTab::Settings.block())
            .render(area, buf);
    }
}