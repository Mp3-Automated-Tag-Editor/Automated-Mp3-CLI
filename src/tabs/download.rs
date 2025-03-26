use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Widget, Padding},
};
use crate::{tabs::tab_renderer::TabRenderer, App, AppMode};
use std::process::{Command, Stdio};
use std::io::{self, BufRead};
use std::thread;
use std::sync::{Arc, Mutex};

pub struct DownloadTab {
    output_stream: Arc<Mutex<Vec<String>>>,
    has_prompted: bool, // Track if we have prompted the user
}

impl DownloadTab {
    pub fn new() -> Self {
        DownloadTab {
            output_stream: Arc::new(Mutex::new(Vec::new())),
            has_prompted: false,
        }
    }

    // Start the download process
    pub fn start_download_process(&self) {
        let output_stream = Arc::clone(&self.output_stream);

        // Spawn the external process (download.exe)
        thread::spawn(move || {
            let mut child = Command::new(r".\mp3-cli-new\src\Sub CLIs\Download CLI\auto-mp3-downloader-event_driven.exe")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start .exe process");

            // Capture the output
            if let Some(stdout) = child.stdout.take() {
                let reader = io::BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line_str) = line {
                        let mut output = output_stream.lock().unwrap();
                        output.push(line_str);  // Store each line of output
                    }
                }
            }

            let _ = child.wait().expect("Failed to wait on .exe process");
        });
    }

    // Render the DownloadTab
    pub fn render(&self, area: Rect, buf: &mut Buffer, app_mode: AppMode) {
        let highlight_color = match app_mode {
            AppMode::InsideTab => Color::Green, // Highlight when editing
            _ => Color::White,
        };

        let paragraph = Paragraph::new("Download tab content goes here.")
            .block(
                Block::bordered()
                    .border_style(highlight_color)
                    .padding(Padding::horizontal(1)),
            );

        paragraph.render(area, buf);

        // Display the output of the .exe process
        self.render_output(area, buf);

        // If the user hasn't been prompted yet, show the prompt
        if !self.has_prompted {
            self.render_start_prompt(area, buf);
        }
    }

    fn render_output(&self, area: Rect, buf: &mut Buffer) {
        let output = self.output_stream.lock().unwrap();
        let output_text = output.join("\n"); // Combine the lines of output into a single string

        let paragraph = Paragraph::new(output_text)
            .block(
                Block::bordered()
                    .border_style(Color::White)
                    .padding(Padding::horizontal(1)),
            );

        paragraph.render(area, buf);
    }

    fn render_start_prompt(&self, area: Rect, buf: &mut Buffer) {
        let prompt_text = "Press Enter again to start the download process...";

        let paragraph = Paragraph::new(prompt_text)
            .block(
                Block::bordered()
                    .border_style(Color::Yellow)
                    .padding(Padding::horizontal(1)),
            );

        paragraph.render(area, buf);
    }

    // Handle Enter key press to start download
    pub fn handle_enter_key(&mut self) {
        if !self.has_prompted {
            self.has_prompted = true; // Set the flag after the user has been prompted
        } else {
            self.start_download_process(); // Start the download process when Enter is pressed again
        }
    }
}

impl TabRenderer for DownloadTab {
    fn render(&self, area: Rect, buf: &mut Buffer, app: &App) {
        self.render(area, buf, app.mode);
    }
}
