use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Borders, Widget};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Paragraph};
use crate::tabs::tab_renderer::TabRenderer;
use crate::tabs::SelectedTab;
use crate::App;

pub struct PlayTab;

impl TabRenderer for PlayTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        // Split area into two equal columns
        let chunks: [Rect; 2] = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .areas(area)
        .try_into()
        .unwrap();

        // Left column
        Paragraph::new("Left Column Content")
            .block(Block::default().borders(Borders::ALL).title("Left"))
            .render(chunks[0], buf);

        // Right column
        Paragraph::new("Right Column Content")
            .block(Block::default().borders(Borders::ALL).title("Right"))
            .render(chunks[1], buf);
    }
}