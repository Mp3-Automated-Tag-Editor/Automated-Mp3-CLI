use ratatui::{buffer::Buffer, layout::Rect};

use crate::App;

pub trait TabRenderer {
    fn render(&mut self, area: Rect, buf: &mut Buffer, app: &App);
}