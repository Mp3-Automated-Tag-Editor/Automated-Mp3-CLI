use ratatui::widgets::Widget;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Paragraph};
use crate::tabs::tab_renderer::TabRenderer;
use crate::tabs::SelectedTab;
use crate::App;

pub struct SettingsTab;

impl TabRenderer for SettingsTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        Paragraph::new("Settings tab content goes here.")
            .block(SelectedTab::Settings.block())
            .render(area, buf);
    }
}