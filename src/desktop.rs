//! Desktop Environment for NexusOS

use crate::vga::{Color, SCREEN, SCREEN_WIDTH, SCREEN_HEIGHT};

const TASKBAR_HEIGHT: usize = 20;

/// Desktop with taskbar
pub struct Desktop;

impl Desktop {
    pub const fn new() -> Self {
        Self
    }

    /// Draw the desktop
    pub fn draw(&self) {
        let screen = SCREEN.lock();

        // Desktop background - gradient effect
        for y in 0..(SCREEN_HEIGHT - TASKBAR_HEIGHT) {
            let color = if y < 80 {
                Color::LightBlue
            } else if y < 140 {
                Color::Blue
            } else {
                Color::Blue
            };
            screen.hline(0, y, SCREEN_WIDTH, color);
        }

        // Taskbar
        let taskbar_y = SCREEN_HEIGHT - TASKBAR_HEIGHT;
        screen.fill_rect(0, taskbar_y, SCREEN_WIDTH, TASKBAR_HEIGHT, Color::LightGray);
        
        // Taskbar top highlight
        screen.hline(0, taskbar_y, SCREEN_WIDTH, Color::White);

        // Start button
        screen.fill_rect(2, taskbar_y + 2, 50, TASKBAR_HEIGHT - 4, Color::LightGray);
        screen.hline(2, taskbar_y + 2, 50, Color::White);
        screen.vline(2, taskbar_y + 2, TASKBAR_HEIGHT - 4, Color::White);
        screen.hline(2, taskbar_y + TASKBAR_HEIGHT - 3, 50, Color::DarkGray);
        screen.vline(51, taskbar_y + 2, TASKBAR_HEIGHT - 4, Color::DarkGray);
        screen.draw_string(8, taskbar_y + 6, "Start", Color::Black, Color::LightGray);

        // Clock area
        let clock_x = SCREEN_WIDTH - 55;
        screen.fill_rect(clock_x, taskbar_y + 2, 53, TASKBAR_HEIGHT - 4, Color::LightGray);
        screen.draw_rect(clock_x, taskbar_y + 2, 53, TASKBAR_HEIGHT - 4, Color::DarkGray);
        screen.draw_string(clock_x + 8, taskbar_y + 6, "12:00", Color::Black, Color::LightGray);
    }
}
