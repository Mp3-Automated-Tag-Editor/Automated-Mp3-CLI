use ratatui::{
    buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, text::Line, widgets::{Block, Borders, Padding, Paragraph, Widget}
};
use crate::{App, tabs::tab_renderer::TabRenderer, AppMode};
use ratatui::style::palette::tailwind::{PURPLE};

pub struct ScraperTab;

impl ScraperTab {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        let is_editing = app.mode == AppMode::InsideTab;
        let highlight_color = if is_editing { PURPLE.c500 } else { Color::White };

        // Split area into two columns
        let chunks = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(area);
        
        // Define left column block (but don't render it yet)
        let left_column_block = Block::bordered()
            .title("Input Parameters")
            .borders(Borders::ALL)
            .border_style(highlight_color);

        // Compute inner layout before rendering left_column_block
        let inner_chunks = Layout::vertical([
            Constraint::Length(15),  // Space for the new text
            Constraint::Length(3),  // Download Directory field
            Constraint::Length(3),  // Start Process button
        ])
        .split(left_column_block.inner(chunks[0]));
        
        // Now render the left column block
        left_column_block.render(chunks[0], buf);

        // Text before the Download Directory input
        let points_to_note = vec![
            Line::from("1. Make sure to select a directory which contains Mp3 files only."),
            Line::from("2. Mp3 files that contain incomplete Metadata will also be searched and indexed."),
            Line::from("3. To download indexed database, kindly turn on Developer Settings in Settings, as this is turned off by default."),
            Line::from("4. Make sure to configure the application, including number of threads to be used to hasten the indexing process."),
            Line::from("5. Remember, the trial allows 100 Deep Searches only, kindly buy more credits to index more songs."),
        ];

        Paragraph::new(points_to_note)
            .block(
                Block::bordered()
                    .title("Information")
                    .padding(Padding::horizontal(1)),
            )
            .wrap(ratatui::widgets::Wrap { trim: true })
            .render(inner_chunks[0], buf); // Render text before the directory input

        // Render input fields inside the bordered block
        let inputs = [
            ("Download Directory", &app.scraper_directory, 0),
        ];

        for (title, value, index) in inputs {
            let field_highlight = if app.edit_selected_field == index && is_editing {
                Color::Yellow
            } else {
                Color::White
            };

            Paragraph::new(format!("{}", value))
                .block(
                    Block::bordered()
                        .title(title)
                        .border_style(field_highlight)
                        .padding(Padding::horizontal(1)),
                )
                .render(inner_chunks[index + 1], buf); // Render the directory input
        }

        // Start Process Button inside the border
        let button_highlight = if app.edit_selected_field == 1 && is_editing {
            Color::Cyan
        } else {
            Color::White
        };

        Paragraph::new("Start Process")
            .block(
                Block::bordered()
                    .title("=")
                    .border_style(button_highlight)
                    .padding(Padding::horizontal(1)),
            )
            .render(inner_chunks[2], buf);

        // Right Column: Logs
        Paragraph::new(format!(
            "Directory: {}\n",
            app.scraper_directory
        ))
        .block(
            Block::bordered()
                .title("Logs")
                .padding(Padding::horizontal(1)),
        )
        .render(chunks[1], buf);
    }
}

impl TabRenderer for ScraperTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        self.render(area, buf, app);
    }
}
