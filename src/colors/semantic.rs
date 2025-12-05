//! Semantic Colors - Purpose-based color definitions
//! Like Radix UI's semantic tokens

use super::palette::{ColorIndex, ColorScale, BLUE, GRAY, RED, GREEN, YELLOW, CYAN};

/// Semantic color roles for the OS UI
pub struct SemanticColors {
    /// Primary brand color
    pub primary: ColorScale,
    /// Neutral/gray colors
    pub neutral: ColorScale,
    /// Success states
    pub success: ColorScale,
    /// Warning states
    pub warning: ColorScale,
    /// Error/danger states
    pub danger: ColorScale,
    /// Info states
    pub info: ColorScale,
}

/// Default semantic color mapping
pub const SEMANTIC: SemanticColors = SemanticColors {
    primary: BLUE,
    neutral: GRAY,
    success: GREEN,
    warning: YELLOW,
    danger: RED,
    info: CYAN,
};

// ============================================================================
// Quick access functions for common use cases
// ============================================================================

/// Get background color for app
#[inline]
pub const fn app_bg() -> ColorIndex {
    SEMANTIC.neutral.bg
}

/// Get primary solid color (buttons, etc)
#[inline]
pub const fn primary_solid() -> ColorIndex {
    SEMANTIC.primary.solid
}

/// Get primary text color on solid backgrounds
#[inline]
pub const fn primary_text() -> ColorIndex {
    SEMANTIC.primary.text
}

/// Get neutral text color (on light backgrounds)
#[inline]
pub const fn text() -> ColorIndex {
    SEMANTIC.neutral.text
}

/// Get subtle text color
#[inline]
pub const fn text_subtle() -> ColorIndex {
    SEMANTIC.neutral.text_low
}

/// Get border color
#[inline]
pub const fn border() -> ColorIndex {
    SEMANTIC.neutral.border
}

/// Get success color
#[inline]
pub const fn success() -> ColorIndex {
    SEMANTIC.success.solid
}

/// Get warning color
#[inline]
pub const fn warning() -> ColorIndex {
    SEMANTIC.warning.solid
}

/// Get danger/error color
#[inline]
pub const fn danger() -> ColorIndex {
    SEMANTIC.danger.solid
}
