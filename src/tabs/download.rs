use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget, Gauge},
};
use crate::{App, tabs::tab_renderer::TabRenderer, AppMode};
use ratatui::style::palette::tailwind::{RED, GRAY};

pub struct DownloadTab;

impl DownloadTab {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        let is_editing = app.mode == AppMode::InsideTab;
        let highlight_color = if is_editing { RED.c500 } else { Color::White };

        // Split area into two columns
        let chunks = Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)]).split(area);

        // Define left column block (but don't render it yet)
        let left_column_block = Block::bordered()
            .title("Input Parameters")
            .borders(Borders::ALL)
            .border_style(highlight_color);

        // Compute inner layout: Adding space for "Points to Note"
        let inner_chunks = Layout::vertical([
            Constraint::Length(12), // Space for "Points to Note"
            Constraint::Length(3),  // URL input
            Constraint::Length(3),  // Output Path input
            Constraint::Length(3),  // Quality input
            Constraint::Length(3),  // Start Process Button
        ])
        .split(left_column_block.inner(chunks[0]));

        // Now render the left column block
        left_column_block.render(chunks[0], buf);

        // Render "Points to Note" before input fields
        let points_to_note = vec![
            Line::from("Some Points to Note:"),
            Line::from("1. You can use Spotify Playlist Links, single Spotify music links, YouTube Playlists, and YouTube Links for the downloader."),
            Line::from(""),
            Line::from("2. Set a specific bitrate. In case of constraints, it will default to the available bitrate."),
            Line::from(""),
        ];

        Paragraph::new(points_to_note)
            .block(
                Block::bordered()
                    .title("Information") // Optional title
                    .padding(Padding::horizontal(1)),
            )
            .wrap(ratatui::widgets::Wrap { trim: true })
            .render(inner_chunks[0], buf); // Place it in its dedicated slot

        // Render input fields inside the bordered block
        let inputs = [
            ("URL", &app.download_url, 1),
            ("Output Path", &app.download_output, 2),
            ("Quality", &app.download_quality, 3),
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
                .render(inner_chunks[index], buf);
        }

        // Start Process Button inside the border
        let button_highlight = if app.edit_selected_field == 4 && is_editing {
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
            .render(inner_chunks[4], buf);

        // Right Column: Split into Logs and Progress Gauge
        let right_column_chunks = Layout::vertical([
            Constraint::Percentage(90),  // Space for Logs
            Constraint::Percentage(10),  // Space for Progress Gauge
        ])
        .split(chunks[1]);

        // Render Logs section
        Paragraph::new(format!(
            "URL: {}\nOutput Path: {}\nQuality: {}",
            app.download_url, app.download_output, app.download_quality
        ))
        .block(
            Block::bordered()
                .title("Logs")
                .padding(Padding::horizontal(1)),
        )
        .render(right_column_chunks[0], buf);

        // Right Column: Progress Gauge (bottom section)
        let progress_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .gauge_style(
                Style::default()
                    .fg(RED.c500)
                    .bg(GRAY.c700),
            )
            .percent(50); // Replace with app.download_progress if applicable

        // Render empty paragraph for space before the gauge
        Paragraph::new("")
            .block(
                Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(0)),
            )
            .render(right_column_chunks[1], buf); // Render empty space before the gauge

        // Render the progress gauge
        progress_gauge.render(right_column_chunks[1], buf);
    }
}

impl TabRenderer for DownloadTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        self.render(area, buf, app);
    }
}
