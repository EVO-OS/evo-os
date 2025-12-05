//! Window Manager for NexusOS

use crate::vga::{Color, SCREEN};

const TITLE_BAR_HEIGHT: usize = 14;

/// A window with title bar and content area
pub struct Window {
    pub title: &'static str,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Window {
    /// Create a new window
    pub const fn new(title: &'static str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { title, x, y, width, height }
    }

    /// Get content area X coordinate
    pub fn content_x(&self) -> usize {
        self.x + 2
    }

    /// Get content area Y coordinate
    pub fn content_y(&self) -> usize {
        self.y + TITLE_BAR_HEIGHT + 2
    }

    /// Draw the window
    pub fn draw(&self) {
        let screen = SCREEN.lock();

        // Shadow
        screen.fill_rect(self.x + 3, self.y + 3, self.width, self.height, Color::DarkGray);

        // Window background
        screen.fill_rect(self.x, self.y, self.width, self.height, Color::White);

        // Border
        screen.draw_rect(self.x, self.y, self.width, self.height, Color::Black);

        // Title bar
        screen.fill_rect(self.x + 1, self.y + 1, self.width - 2, TITLE_BAR_HEIGHT, Color::Blue);

        // Title text
        screen.draw_string(self.x + 4, self.y + 3, self.title, Color::White, Color::Blue);

        // Close button
        let btn_x = self.x + self.width - 14;
        let btn_y = self.y + 2;
        screen.fill_rect(btn_x, btn_y, 11, 10, Color::LightGray);
        screen.draw_rect(btn_x, btn_y, 11, 10, Color::Black);
        screen.draw_char(btn_x + 2, btn_y + 1, 'X', Color::Black, Color::LightGray);

        // Title bar bottom line
        screen.hline(self.x + 1, self.y + TITLE_BAR_HEIGHT + 1, self.width - 2, Color::DarkGray);
    }
}
