//! NexusOS Color System
//! Inspired by Radix UI's semantic color scales
//! 
//! Each color has a 12-step scale for different use cases:
//! - Steps 1-2: App backgrounds
//! - Steps 3-5: Component backgrounds
//! - Steps 6-8: Borders and separators
//! - Steps 9-10: Solid backgrounds
//! - Steps 11-12: Text

pub mod palette;
pub mod semantic;
pub mod theme;

pub use palette::*;
pub use semantic::*;
pub use theme::*;
