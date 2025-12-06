//! Terminal Display for EvoOS Shell

use crate::vga::{SCREEN, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::colors::theme::CURRENT_THEME;
use crate::colors::palette::ColorIndex;

const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 8;
const COLS: usize = SCREEN_WIDTH / CHAR_WIDTH;   // 40 columns
const ROWS: usize = SCREEN_HEIGHT / CHAR_HEIGHT; // 25 rows

/// Terminal state
pub struct Terminal {
    cursor_x: usize,
    cursor_y: usize,
    fg_color: ColorIndex,
    bg_color: ColorIndex,
}

impl Terminal {
    /// Create new terminal
    pub const fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            fg_color: 7,  // Light gray
            bg_color: 0,  // Black
        }
    }

    /// Initialize terminal display
    pub fn init(&mut self) {
        self.fg_color = CURRENT_THEME.terminal_text;
        self.bg_color = CURRENT_THEME.terminal_bg;
        self.clear();
        self.print_banner();
    }

    /// Clear terminal screen
    pub fn clear(&mut self) {
        let screen = SCREEN.lock();
        screen.clear_with_color(self.bg_color);
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// Print boot banner
    fn print_banner(&mut self) {
        self.set_color(CURRENT_THEME.terminal_prompt);
        self.println("╔═══════════════════════════════════════╗");
        self.println("║         EvoOS Shell v0.1              ║");
        self.println("║     Type 'help' for commands          ║");
        self.println("╚═══════════════════════════════════════╝");
        self.set_color(CURRENT_THEME.terminal_text);
        self.println("");
    }

    /// Set text color
    pub fn set_color(&mut self, color: ColorIndex) {
        self.fg_color = color;
    }

    /// Print a character
    pub fn putc(&mut self, c: char) {
        match c {
            '\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
            '\r' => {
                self.cursor_x = 0;
            }
            '\t' => {
                self.cursor_x = (self.cursor_x + 4) & !3;
            }
            _ => {
                if self.cursor_x < COLS && self.cursor_y < ROWS {
                    let screen = SCREEN.lock();
                    let x = self.cursor_x * CHAR_WIDTH;
                    let y = self.cursor_y * CHAR_HEIGHT;
                    screen.draw_char_raw(x, y, c, self.fg_color, self.bg_color);
                    self.cursor_x += 1;
                }
            }
        }

        // Handle line wrap
        if self.cursor_x >= COLS {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }

        // Handle scroll
        if self.cursor_y >= ROWS {
            self.scroll();
            self.cursor_y = ROWS - 1;
        }
    }

    /// Print a string
    pub fn print(&mut self, s: &str) {
        for c in s.chars() {
            self.putc(c);
        }
    }

    /// Print a string with newline
    pub fn println(&mut self, s: &str) {
        self.print(s);
        self.putc('\n');
    }

    /// Print shell prompt
    pub fn prompt(&mut self) {
        self.set_color(CURRENT_THEME.terminal_prompt);
        self.print("evo$ ");
        self.set_color(CURRENT_THEME.terminal_text);
    }

    /// Scroll screen up by one line
    fn scroll(&mut self) {
        let screen = SCREEN.lock();
        // Move all lines up
        for y in 0..(ROWS - 1) {
            for x in 0..SCREEN_WIDTH {
                let src_y = (y + 1) * CHAR_HEIGHT + (x % CHAR_HEIGHT);
                let dst_y = y * CHAR_HEIGHT + (x % CHAR_HEIGHT);
                // Copy pixel by pixel (simplified)
                screen.copy_line(dst_y, src_y);
            }
        }
        // Clear last line
        screen.fill_rect(
            0, 
            (ROWS - 1) * CHAR_HEIGHT, 
            SCREEN_WIDTH, 
            CHAR_HEIGHT, 
            self.bg_color
        );
    }
}

/// Global terminal instance
pub static mut TERM: Terminal = Terminal::new();

/// Get terminal reference
pub fn terminal() -> &'static mut Terminal {
    unsafe { &mut TERM }
}
