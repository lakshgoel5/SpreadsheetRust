/// Common types module used throughout the terminal spreadsheet application.
///
/// This module defines fundamental types like Coordinates that are
/// used across different components of the spreadsheet.
use std::fmt;

/// Represents coordinates within the spreadsheet grid.
///
/// Used to identify cells by their row and column positions, and to
/// represent positions for operations and rendering.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Coordinates {
    /// The row index (0-based)
    pub row: i32,
    
    /// The column index (0-based)
    pub col: i32,
}

impl fmt::Display for Coordinates {
    /// Formats the Coordinates for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter
    ///
    /// # Returns
    ///
    /// A Result indicating whether the formatting succeeded
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}
