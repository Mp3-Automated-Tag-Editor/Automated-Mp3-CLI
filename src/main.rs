mod tabs;
use tabs::SelectedTab;
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
    widgets::{Block, Padding, Tabs, Widget},
    Terminal,
};
use std::io::{self, stdout};
use strum::IntoEnumIterator;

fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let app_result = App::default().run(terminal);
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    app_result
}

#[derive(Default)]
struct App {
    state: AppState,
    mode: AppMode,
    selected_tab: SelectedTab,
    pub scraper_directory: String,
    pub download_url: String,
    pub download_output: String,
    pub download_quality: String,
    pub edit_selected_field: usize,
    home_scroll: u16,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppMode {
    #[default]
    Navigation,
    InsideTab,
}

impl App {
    fn run(mut self, mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.size()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match self.mode {
                    AppMode::Navigation => self.handle_navigation_mode(key.code),
                    AppMode::InsideTab => self.handle_inside_tab_mode(key.code),
                }
            }
        }
        Ok(())
    }

    fn handle_navigation_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
            KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
            KeyCode::Char('q') | KeyCode::Esc => self.quit(),
            KeyCode::Enter => self.mode = AppMode::InsideTab,
            _ => {}
        }
    }

    fn handle_inside_tab_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.mode = AppMode::Navigation,
    
            KeyCode::Up => {
                if self.selected_tab == SelectedTab::Download {
                    if self.edit_selected_field > 1 {
                        self.edit_selected_field -= 1;
                    } else {
                        self.edit_selected_field = 4;
                    }
                }

                if self.selected_tab == SelectedTab::Scraper {
                    if self.edit_selected_field == 1 {
                        self.edit_selected_field = 0;
                    } else {
                        self.edit_selected_field = 1;
                    }
                }

                if self.selected_tab == SelectedTab::Home {
                    self.home_scroll = self.home_scroll.saturating_sub(1);
                }
            }

            KeyCode::Down => {
                if self.selected_tab == SelectedTab::Download {
                    if self.edit_selected_field < 4 {
                        self.edit_selected_field += 1;
                    } else {
                        self.edit_selected_field = 1;
                    }
                }

                if self.selected_tab == SelectedTab::Scraper {
                    if self.edit_selected_field == 0 {
                        self.edit_selected_field = 1;
                    } else {
                        self.edit_selected_field = 0;
                    }
                }

                if self.selected_tab == SelectedTab::Home {
                    self.home_scroll += 1; // Increase scroll offset
                }
            }
    
            KeyCode::Backspace => {
                if self.selected_tab == SelectedTab::Download {
                    match self.edit_selected_field {
                        1 => { self.download_url.pop(); }
                        2 => { self.download_output.pop(); }
                        3 => { self.download_quality.pop(); }
                        _ => {}
                    }
                }

                if self.selected_tab == SelectedTab::Scraper {
                    match self.edit_selected_field {
                        0 => { self.scraper_directory.pop(); }
                        _ => {}
                    }
                }
            }
    
            KeyCode::Char(c) => {
                match self.selected_tab {
                    SelectedTab::Download => {
                        match self.edit_selected_field {
                            1 => self.download_url.push(c),
                            2 => self.download_output.push(c),
                            3 => self.download_quality.push(c),
                            _ => {}
                        }
                    }
            
                    SelectedTab::Scraper => {
                        match self.edit_selected_field {
                            0 => self.scraper_directory.push(c),
                            _ => {}
                        }
                    }
            
                    _ => {}
                }
            }            
    
            _ => {}
        }
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

        let horizontal = Layout::horizontal([Min(0), Length(36)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);

        let mut renderer = self.selected_tab.renderer();
        renderer.render(inner_area, buf, self);
        
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
    "Automated Mp3 Tag Editor CLI - v1.0".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Enter to edit/view | ▲ ▼ to scroll | Esc to go back | q to quit")
        .centered()
        .render(area, buf);
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut renderer = self.renderer();
        renderer.render(area, buf, &App::default());
    }
}

impl SelectedTab {
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
            Self::Download => tailwind::RED,
            Self::Scraper => tailwind::PURPLE,            
            Self::Edit => tailwind::ORANGE,
            Self::Play => tailwind::EMERALD,
            Self::Settings => tailwind::GRAY,
        }
    }
}