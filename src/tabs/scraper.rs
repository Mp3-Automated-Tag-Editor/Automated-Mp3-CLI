use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Widget, Padding},
};
use crate::{App, tabs::tab_renderer::TabRenderer, AppMode};

pub struct ScraperTab;

impl ScraperTab {
    pub fn render(&self, area: Rect, buf: &mut Buffer, app_mode: AppMode, input: &str) {
        let highlight_color = match app_mode {
            AppMode::InsideTab => Color::Green, // Highlight when editing
            _ => Color::White,
        };

        let paragraph = Paragraph::new(format!("Directory: {}", input))
            .block(
                Block::bordered()
                    .border_style(highlight_color)
                    .padding(Padding::horizontal(1)),
            );

        paragraph.render(area, buf);
    }
}

impl TabRenderer for ScraperTab {
    fn render(&self, area: Rect, buf: &mut Buffer, app: &App) {
        self.render(area, buf, app.mode, &app.scraper_input);
    }
}
