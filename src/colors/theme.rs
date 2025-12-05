//! Theme - Complete UI theme configuration
//! Defines all colors used in the OS UI

use super::palette::ColorIndex;
use super::semantic::SEMANTIC;

/// Complete theme for the OS UI
#[derive(Clone, Copy)]
pub struct Theme {
    // Desktop
    pub desktop_bg: ColorIndex,
    pub desktop_gradient: ColorIndex,
    
    // Taskbar
    pub taskbar_bg: ColorIndex,
    pub taskbar_border: ColorIndex,
    pub taskbar_text: ColorIndex,
    
    // Windows
    pub window_bg: ColorIndex,
    pub window_border: ColorIndex,
    pub window_shadow: ColorIndex,
    pub titlebar_bg: ColorIndex,
    pub titlebar_text: ColorIndex,
    pub titlebar_button_bg: ColorIndex,
    
    // Buttons
    pub button_bg: ColorIndex,
    pub button_border_light: ColorIndex,
    pub button_border_dark: ColorIndex,
    pub button_text: ColorIndex,
    
    // Terminal/Shell
    pub terminal_bg: ColorIndex,
    pub terminal_text: ColorIndex,
    pub terminal_cursor: ColorIndex,
    pub terminal_prompt: ColorIndex,
}

/// Default light theme
pub const LIGHT_THEME: Theme = Theme {
    // Desktop - blue gradient
    desktop_bg: 9,      // Light blue
    desktop_gradient: 1, // Blue
    
    // Taskbar - gray
    taskbar_bg: 7,       // Light gray
    taskbar_border: 15,  // White highlight
    taskbar_text: 0,     // Black
    
    // Windows - white with blue titlebar
    window_bg: 15,       // White
    window_border: 0,    // Black
    window_shadow: 8,    // Dark gray
    titlebar_bg: 1,      // Blue
    titlebar_text: 15,   // White
    titlebar_button_bg: 7, // Light gray
    
    // Buttons - 3D gray
    button_bg: 7,        // Light gray
    button_border_light: 15, // White
    button_border_dark: 8,   // Dark gray
    button_text: 0,      // Black
    
    // Terminal - dark
    terminal_bg: 0,      // Black
    terminal_text: 7,    // Light gray
    terminal_cursor: 15, // White
    terminal_prompt: 10, // Light green
};

/// Dark theme for terminal-focused use
pub const DARK_THEME: Theme = Theme {
    desktop_bg: 0,
    desktop_gradient: 8,
    
    taskbar_bg: 8,
    taskbar_border: 7,
    taskbar_text: 15,
    
    window_bg: 8,
    window_border: 7,
    window_shadow: 0,
    titlebar_bg: 0,
    titlebar_text: 15,
    titlebar_button_bg: 8,
    
    button_bg: 8,
    button_border_light: 7,
    button_border_dark: 0,
    button_text: 15,
    
    terminal_bg: 0,
    terminal_text: 10,
    terminal_cursor: 15,
    terminal_prompt: 10,
};

/// Get the current active theme
pub static CURRENT_THEME: Theme = LIGHT_THEME;
