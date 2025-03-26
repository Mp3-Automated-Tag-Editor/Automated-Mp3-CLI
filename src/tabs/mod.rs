// src/tabs/mod.rs

use strum::{Display, EnumIter, FromRepr};

pub mod home;
pub mod scraper;
pub mod download;
pub mod edit;
pub mod play;
pub mod settings;
pub mod tab_renderer; // Add the new module

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq, Eq)]
pub enum SelectedTab {
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

impl SelectedTab {
    /// Get the renderer for the selected tab
    pub fn renderer(&self) -> Box<dyn tab_renderer::TabRenderer> {
        match self {
            Self::Home => Box::new(home::HomeTab),
            Self::Scraper => Box::new(scraper::ScraperTab),
            Self::Download => Box::new(download::DownloadTab::new()),
            Self::Edit => Box::new(edit::EditTab),
            Self::Play => Box::new(play::PlayTab),
            Self::Settings => Box::new(settings::SettingsTab),
        }
    }
}