use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
    Terminal,
};
use ratatui::style::{Style, Modifier};
use ratatui::widgets::Wrap;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use std::io::{self, stdout};

fn main() -> Result<()> {
    // Setup error handling
    color_eyre::install()?;

    // Enable raw mode for terminal input handling
    enable_raw_mode()?;
    
    let mut stdout = stdout();
    // Switch to alternate screen to avoid messing up the default terminal display
    execute!(stdout, EnterAlternateScreen)?;

    // Initialize the Ratatui terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the application
    let app_result = App::default().run(terminal); // Pass `terminal` by value

    // Restore the terminal state before exiting
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    app_result
}

#[derive(Default)]
struct App {
    state: AppState,
    selected_tab: SelectedTab,
    scroll_offset: usize,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Home")]
    Home,
    #[strum(to_string = "Scraper")]
    Scraper,
    #[strum(to_string = "Download")]
    Download,
    #[strum(to_string = "Edit")]
    Edit,
    #[strum(to_string = "Play")]
    Play,
    #[strum(to_string = "Settings")]
    Settings,
}

impl App {
    fn run(mut self, mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.size()))?; // Use `frame.size()`
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf);
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Ratatui Tabs Example".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::Home => self.render_home(area, buf),
            Self::Scraper => self.render_scraper(area, buf),
            Self::Download => self.render_download(area, buf),
            Self::Edit => self.render_edit(area, buf),
            Self::Play => self.render_play(area, buf),
            Self::Settings => self.render_settings(area, buf),
        }
    }
}

impl SelectedTab {
    fn render_home(self, area: Rect, buf: &mut Buffer) {
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
            .block(self.block())
            .wrap(Wrap { trim: true }) // Enable text wrapping
            .render(area, buf);
    }

    fn render_scraper(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Scraper tab content goes here.")
            .block(self.block())
            .render(area, buf);
    }

    fn render_download(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Download tab content goes here.")
            .block(self.block())
            .render(area, buf);
    }

    fn render_edit(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Edit tab content goes here.")
            .block(self.block())
            .render(area, buf);
    }

    fn render_play(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Play tab content goes here.")
            .block(self.block())
            .render(area, buf);
    }

    fn render_settings(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Settings tab content goes here.")
            .block(self.block())
            .render(area, buf);
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Home => tailwind::BLUE,
            Self::Scraper => tailwind::PURPLE,
            Self::Download => tailwind::RED,
            Self::Edit => tailwind::ORANGE,
            Self::Play => tailwind::EMERALD,
            Self::Settings => tailwind::GRAY,
        }
    }
}