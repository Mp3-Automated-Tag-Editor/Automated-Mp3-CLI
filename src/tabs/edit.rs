use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::material::WHITE, Color, Style},
    widgets::{Block, Borders, Padding, Paragraph, Row, Table, TableState, Widget},
};
use crate::tabs::tab_renderer::TabRenderer;
use crate::{App, AppMode};
use ratatui::style::palette::tailwind::{ORANGE, BLUE, GRAY};
use serde::{Deserialize};
use serde_json::Result;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Clone)]
struct Metadata {
    file_name: String,
    percentage: u16,
    title: String,
    artist: String,
    album: String,
    path: String,
    year: u16,
    genre: String,
    track: u16,
    status: String,
}

pub struct EditTab;

impl EditTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        let is_editing = app.mode == AppMode::InsideTab;
        let highlight_color = if is_editing { ORANGE.c500 } else { Color::White };

        // Split area into two columns: 75% for left column (table), 25% for right column (fields)
        let chunks = Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]).split(area);

        // Left Column Block (Table)
        let left_column_block = Block::bordered()
            .title("Metadata Table")
            .borders(Borders::ALL)
            .border_style(highlight_color);

        // Load the metadata from the JSON file
        let metadata = self.load_metadata_from_file().unwrap();

        // Define the column headers
        let headers = vec![
            "File Name", "Percentage", "Title", "Artist", "Album", "Path", 
            "Year", "Genre", "Track", "Status",
        ];

        // Render Table with headers
        let rows = metadata.iter().map(|item| {
            Row::new(vec![
                item.file_name.clone(),
                item.percentage.to_string(),
                item.title.clone(),
                item.artist.clone(),
                item.album.clone(),
                item.path.clone(),
                item.year.to_string(),
                item.genre.clone(),
                item.track.to_string(),
                item.status.clone(),
            ])
            .height(1)
        }).collect::<Vec<Row>>();

        let widths = [
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
        ];

        let table = Table::new(rows, widths)
            .header(Row::new(headers).style(Style::default().fg(WHITE)))
            .block(left_column_block)
            .style(Style::default().fg(WHITE))
            .highlight_style(Style::default().fg(ORANGE.c500))
            .widths(&widths);

        // Render the table in the left column
        table.render(chunks[0], buf);

        // Right Column Block: Editable fields for selected row
        let right_column_block = Block::bordered()
            .title("Edit Metadata")
            .borders(Borders::ALL)
            .border_style(highlight_color);

        // Render the right column block
        right_column_block.render(chunks[1], buf);

        // Compute the layout for the right column (editable fields)
        let right_column_chunks = Layout::vertical([
            Constraint::Length(3), // File Name
            Constraint::Length(3), // Percentage
            Constraint::Length(3), // Title
            Constraint::Length(3), // Artist
            Constraint::Length(3), // Album
            Constraint::Length(3), // Path
            Constraint::Length(3), // Year
            Constraint::Length(3), // Genre
            Constraint::Length(3), // Track
            Constraint::Length(3), // Status
            Constraint::Length(3), // Album Artist
            Constraint::Length(3), // Composer
            Constraint::Length(3), // Disc No
            Constraint::Length(3), // Comments
            Constraint::Length(3), // Session Name
            Constraint::Length(5), // Image (space for image)
        ])
        .split(chunks[1]);

        // Render each editable field
        for (title, value, index) in [
            ("File Name", &app.selected_file.file_name, 0),
            ("Percentage", &app.selected_file.percentage.to_string(), 1),
            ("Title", &app.selected_file.title, 2),
            ("Artist", &app.selected_file.artist, 3),
            ("Album", &app.selected_file.album, 4),
            ("Path", &app.selected_file.path, 5),
            ("Year", &app.selected_file.year.to_string(), 6),
            ("Genre", &app.selected_file.genre, 7),
            ("Track", &app.selected_file.track.to_string(), 8),
            ("Status", &app.selected_file.status, 9),
            ("Album Artist", &app.selected_file.album_artist, 10),
            ("Composer", &app.selected_file.composer, 11),
            ("Disc No", &app.selected_file.disc_no.to_string(), 12),
            ("Comments", &app.selected_file.comments, 13),
            ("Session Name", &app.selected_file.session_name, 14),
        ] {
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
                .render(right_column_chunks[index], buf);
        }

        // Render image placeholder (can be updated later with actual image rendering)
        let field_highlight = if app.edit_selected_field == 15 && is_editing {
            Color::Yellow
        } else {
            Color::White
        };

        Paragraph::new("Image Placeholder")
            .block(
                Block::bordered()
                    .title("Image")
                    .border_style(field_highlight)
                    .padding(Padding::horizontal(1)),
            )
            .render(right_column_chunks[15], buf);
    }

    fn load_metadata_from_file(&self) -> Result<Vec<Metadata>> {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let file_path = current_dir.join(r".\src\data.json"); // Adjust the file path

        let data = fs::read_to_string(file_path).expect("Unable to read file");
        let metadata: Vec<Metadata> = serde_json::from_str(&data)?;
        Ok(metadata)
    }
}

impl TabRenderer for EditTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        self.render(area, buf, app);
    }
}
