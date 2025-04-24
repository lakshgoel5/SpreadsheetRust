# Makefile

# Name of the binary (defined in Cargo.toml)
BIN_NAME=spreadsheet
EXT1=extension

# Target path for the release binary
TARGET_PATH=target/release/$(BIN_NAME)

# Default target: build the release binary
all: $(TARGET_PATH)

# How to build the binary
$(TARGET_PATH):
	cargo build --release

run: $(TARGET_PATH)
	$(TARGET_PATH) $(ARGS)
# Optional clean target
clean:
	cargo clean
