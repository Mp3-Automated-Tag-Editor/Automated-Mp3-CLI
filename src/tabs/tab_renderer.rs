use ratatui::{buffer::Buffer, layout::Rect};

pub trait TabRenderer {
    fn render(&self, area: Rect, buf: &mut Buffer);
}