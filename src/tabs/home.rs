use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Paragraph, Widget, Wrap},
};

use ratatui::style::palette::tailwind::{BLUE, WHITE};

use crate::{tabs::tab_renderer::TabRenderer, App, AppMode};
use crate::tabs::SelectedTab;

pub struct HomeTab;

impl HomeTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        let is_editing = app.mode == AppMode::InsideTab;
        let highlight_color = if is_editing { WHITE } else { BLUE.c700 };
        
        // ASCII Art Header (with dots)
        let ascii_header = vec![
            r#".....___         __           __  ___     _____    ______               ______    ___ __              "#,
            r#"..../   | __  __/ /_____     /  |/  ____ |__  /   /_  ______ _____ _   / ________/ (_/ /_____  _____"#,
            r#".../ /| |/ / / / __/ __ \   / /|_/ / __ \ /_ <     / / / __ `/ __ `/  / __/ / __  / / __/ __ \/ ___/"#,
            r#"../ ___ / /_/ / /_/ /_/ /  / /  / / /_/ ___/ /    / / / /_/ / /_/ /  / /___/ /_/ / / /_/ /_/ / /    "#,
            r#"./_/  |_\__,_/\__/\____/  /_/  /_/ .___/____/    /_/  \__,_/\__, /  /_____/\__,_/_/\__/\____/_/     "#,
            r#"................................/_/......................../____/....................................."#,
        ];
    
        // Subheader
        let subheader = "A JRS Studios app - v0.1.0";
    
        // Welcome Paragraph
        let welcome_text = vec![
            Line::from("Welcome to the Auto-Mp3 Tag Editor! This CLI tool allows you to:"),
            Line::from("- Scrape metadata for your music collection."),
            Line::from("- Download music from Spotify and YouTube playlists."),
            Line::from("- Edit metadata for your songs."),
            Line::from("- Play your music directly from the CLI!"),
            Line::from(""),
            Line::from("This app is designed to make managing your music library a breeze."),
        ];
    
        // About Paragraph
        let about_text = vec![
            Line::from("Hey there! Thanks for using the Automated Mp3 Tag Editor, by JRS Studios."),
            Line::from("This project was once a simple Python CLI tool, developed to complete a"),
            Line::from("Software Engineering course. 3 years later, it has evolved into a full-stack"),
            Line::from("Rust-based Desktop Application with an ML-powered backend, capable of handling"),
            Line::from("offline music. We have grand plans, including developing a mobile app to"),
            Line::from("support the music identified here. So thank you once again for using our app."),
            Line::from("Please feel free to contribute, and reach out to us for suggestions as well as"),
            Line::from("bug reports. Keep Scraping!"),
        ];
    
        // Usage Instructions
        let usage_text = vec![
            Line::from("How to use this app:"),
            Line::from("- Use the ◄ and ► arrow keys to navigate between tabs."),
            Line::from("- Press 'q' or 'Esc' to quit the application."),
            Line::from("- Press 'Space' to start/stop music playback."),
            Line::from("- Follow on-screen instructions for each tab to perform actions."),
        ];
    
        // Links
        let links_text = vec![
            Line::from("For more information, visit:"),
            Line::from("- GitHub: https://github.com/your-repo/auto-mp3-tag-editor"),
            Line::from("- README: https://github.com/your-repo/auto-mp3-tag-editor#readme"),
        ];
    
        // Combine all text into a single vector of Line objects
        let mut styled_text = Vec::new();
    
        // Add ASCII Header (Blue and Bold)
        for line in ascii_header {
            styled_text.push(Line::from(line).style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)));
        }
    
        // Add an empty line for spacing
        styled_text.push(Line::from(""));
    
        // Add Subheader (Yellow)
        styled_text.push(Line::from(subheader).style(Style::default().fg(Color::Yellow)));
    
        // Add an empty line for spacing
        styled_text.push(Line::from(""));
    
        // Add Welcome Paragraph (Default Style)
        styled_text.extend(welcome_text);
    
        // Add an empty line for spacing
        styled_text.push(Line::from(""));
    
        // Add About Paragraph (Italic)
        styled_text.extend(about_text.into_iter().map(|line| line.style(Style::default().add_modifier(Modifier::ITALIC))));
    
        // Add an empty line for spacing
        styled_text.push(Line::from(""));
    
        // Add Usage Instructions (Green)
        styled_text.extend(usage_text.into_iter().map(|line| line.style(Style::default().fg(Color::Green))));
    
        // Add an empty line for spacing
        styled_text.push(Line::from(""));
    
        // Add Links (Cyan and Underlined)
        styled_text.extend(links_text.into_iter().map(|line| line.style(Style::default().fg(Color::Cyan).add_modifier(Modifier::UNDERLINED))));
    
        // Render the styled paragraph with wrapping
        Paragraph::new(styled_text)
            .block(SelectedTab::Home.block().border_style(highlight_color))
            .wrap(Wrap { trim: true }) // Enable text wrapping
            .scroll((app.home_scroll, 0))
            .render(area, buf);
    }
}

impl TabRenderer for HomeTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App) {
        self.render(area, buf, app);
    }
}
