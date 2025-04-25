# Spreadsheet Rust
A versatile and feature-rich spreadsheet application implemented in Rust with both terminal-based and web interfaces.
### Authors
- Laksh Goel (2023CS10848)
- Yuvika Chaudhary (2023CS10353)
- Adit Jindal (2023CS50353)
## Features
- **Dual Interface**: Both terminal and web-based interfaces for versatile usage
- **Formula Support**: Calculate cell values with various formulas and operations
- **Range Functions**: Support for SUM, AVG, MIN, MAX, and STDEV across cell ranges
- **Dependency Management**: Automatic recalculation of dependent cells
- **Cycle Detection**: Prevention of circular dependencies in formulas
- **Undo/Redo**: Track and reverse changes to the spreadsheet
- **Data Visualization**: Create charts and graphs in the web interface
- **Image Generation**: Visual representation of spreadsheet data as pixel art/images
- **JSON Import/Export**: Save and load spreadsheets in JSON format

## Prerequisites
- Rust (latest stable version recommended)
- pkg-config
- libfontconfig1-dev pkg-config (for Linux systems)
- wasm-bindgen (for web extension)
- trunk (for web extension)

## Quick Start
### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/SpreadsheetRust.git
   cd SpreadsheetRust
   ```
2. Build the project:
   ```bash
   make
   ```
3. Run the extension version:
   ```bash
    make ext1
    ```

### Usage with Makefile
This project includes a Makefile with various commands to simplify operation:
- `make`: Build the terminal version (default)
- `make run ARGS="<rows> <columns>"`: Run the terminal spreadsheet with specified dimensions.
  - Example: `make run ARGS="20 20"` creates a 20x20 spreadsheet.
- `make ext1`: Build and run the web extension.
- `make test`: Run all tests for the project.
- `make clean`: Clean build artifacts.
- `make docs`: Generate documentation.

## Terminal Interface Commands
- `A1=5`: Set cell A1 to value 5
- `B2=SUM(A1:A10)`: Calculate sum of range A1:A10.
- `C3=A1+B2`: Perform arithmetic operations.
- `D4=AVG(B1:B5)`: Calculate average of range B1:B5.
- `w, a, s, d`: Navigate the spreadsheet (up, left, down, right).
- `scroll_to A10`: Jump to a specific cell.
- `enable_output, disable_output`: Toggle output display.
- `save filename.json`: Save current spreadsheet state.
- `load filename.json`: Load spreadsheet from file.
- `q`: Quit the application.

## Web Interface
The web interface provides a modern graphical user experience with additional features:

- Intuitive cell editing and formula entry
- Range selection with mouse
- Dynamic charting and visualization
- Pixel art/image generation from data
- Undo/Redo buttons
- Theme switching (light/dark mode)

## Project Structure
- `terminal/`: Terminal interface implementation
- `extension/`: Web interface implementation
    - `backend/`: Backend logic for the web extension
    - `frontend/`: Frontend components for the web UI
    - `parser/`: Command parsing for both interfaces
- `tests/`: Unit and integration tests

## Testing
The project includes comprehensive tests:
    
    - Backend functionality tests
    - Frontend interaction tests
    - Formula parsing and evaluation tests
    - Graph dependency tests

## Documentation
Generate documentation with:
```bash
make report
cargo doc --open
```

<!-- For terminal:
cargo run #rows #cols - to specify size to initiate
cargo run #rows #cols path - to open from a given json file as per the path. If the path does not work, then a new sheet as per the given dimensions is initiated.

For web:
trunk serve - creates a new sheet of 100 x 100 (default size)
setting environment variables for rows and columns: - to create new sheet of the specified size
Windows - $env:MY_ROWS="#"; $env:MY_COLS="#"; trunk serve
Linux - MY_ROWS=# MY_COLS=# trunk serve

After using env variables, they can be reset using - 
Windows: Remove-Item Env:VAR_NAME
Linux: unset VAR_NAME -->