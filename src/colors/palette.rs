//! Color Palette - VGA 256-color mappings
//! Based on Radix UI color philosophy

/// Base VGA palette index type
pub type ColorIndex = u8;

/// Color scales (similar to Radix's 12-step scales, adapted for VGA)
#[derive(Debug, Clone, Copy)]
pub struct ColorScale {
    /// Background (lightest)
    pub bg: ColorIndex,
    /// Background subtle
    pub bg_subtle: ColorIndex,
    /// Component background
    pub component: ColorIndex,
    /// Component hover
    pub component_hover: ColorIndex,
    /// Component active
    pub component_active: ColorIndex,
    /// Border subtle
    pub border_subtle: ColorIndex,
    /// Border
    pub border: ColorIndex,
    /// Border hover
    pub border_hover: ColorIndex,
    /// Solid background
    pub solid: ColorIndex,
    /// Solid hover
    pub solid_hover: ColorIndex,
    /// Text low contrast
    pub text_low: ColorIndex,
    /// Text high contrast
    pub text: ColorIndex,
}

// ============================================================================
// BLUE SCALE (Primary)
// ============================================================================
pub const BLUE: ColorScale = ColorScale {
    bg: 9,              // Light blue bg
    bg_subtle: 1,       // Blue subtle
    component: 9,       // Light blue
    component_hover: 1, // Blue
    component_active: 1,
    border_subtle: 1,
    border: 1,
    border_hover: 1,
    solid: 1,           // Blue solid
    solid_hover: 9,     // Light blue
    text_low: 1,
    text: 15,           // White on blue
};

// ============================================================================
// GRAY SCALE (Neutral)
// ============================================================================
pub const GRAY: ColorScale = ColorScale {
    bg: 15,             // White
    bg_subtle: 7,       // Light gray
    component: 7,       // Light gray
    component_hover: 8, // Dark gray
    component_active: 8,
    border_subtle: 7,
    border: 8,          // Dark gray
    border_hover: 0,    // Black
    solid: 8,           // Dark gray
    solid_hover: 0,     // Black
    text_low: 8,        // Dark gray text
    text: 0,            // Black text
};

// ============================================================================
// RED SCALE (Danger/Error)
// ============================================================================
pub const RED: ColorScale = ColorScale {
    bg: 12,             // Light red
    bg_subtle: 4,       // Red
    component: 12,
    component_hover: 4,
    component_active: 4,
    border_subtle: 4,
    border: 4,
    border_hover: 4,
    solid: 4,           // Red
    solid_hover: 12,    // Light red
    text_low: 4,
    text: 15,           // White on red
};

// ============================================================================
// GREEN SCALE (Success)
// ============================================================================
pub const GREEN: ColorScale = ColorScale {
    bg: 10,             // Light green
    bg_subtle: 2,       // Green
    component: 10,
    component_hover: 2,
    component_active: 2,
    border_subtle: 2,
    border: 2,
    border_hover: 2,
    solid: 2,           // Green
    solid_hover: 10,    // Light green
    text_low: 2,
    text: 15,           // White on green
};

// ============================================================================
// YELLOW SCALE (Warning)
// ============================================================================
pub const YELLOW: ColorScale = ColorScale {
    bg: 14,             // Yellow
    bg_subtle: 6,       // Brown
    component: 14,
    component_hover: 6,
    component_active: 6,
    border_subtle: 6,
    border: 6,
    border_hover: 0,
    solid: 14,          // Yellow
    solid_hover: 6,     // Brown
    text_low: 6,
    text: 0,            // Black on yellow
};

// ============================================================================
// CYAN SCALE (Info)
// ============================================================================
pub const CYAN: ColorScale = ColorScale {
    bg: 11,             // Light cyan
    bg_subtle: 3,       // Cyan
    component: 11,
    component_hover: 3,
    component_active: 3,
    border_subtle: 3,
    border: 3,
    border_hover: 3,
    solid: 3,           // Cyan
    solid_hover: 11,    // Light cyan
    text_low: 3,
    text: 0,            // Black on cyan
};
