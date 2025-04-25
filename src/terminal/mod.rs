pub mod backend;
/// Terminal module for the spreadsheet application.
///
/// This module provides the core functionality for the terminal-based version of the
/// spreadsheet, including:
///
/// - Backend logic for cell computation and dependency tracking
/// - Spreadsheet rendering and display in the terminal
/// - Command parsing and processing
/// - Graph-based dependency management
///
/// The terminal version allows for interactive use via command-line input.
pub mod functions;
pub mod graph;
pub mod parser;
pub mod spreadsheet;
pub mod types;
