# rust_lab

For terminal:
cargo run #rows #cols - to specify size to initiate
cargo run #rows #cols path - to open from a given json file as per the path. If the path does not work, then a new sheet as per the given dimensions is initiated.

For web:
trunk serve - creates a new sheet of 100 x 100 (default size)
setting environment variables for rows and columns: - to create new sheet of the specified size
Windows - $env:MY_ROWS="#"; $env:MY_COLS="#"; trunk serve
Linux - MY_ROWS=# MY_COLS=# trunk serve