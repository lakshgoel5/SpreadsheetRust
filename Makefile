# Makefile

# Name of the binary (defined in Cargo.toml)
BIN_NAME=spreadsheet
EXT1=extension
PDF_FILE = report.pdf
# Target path for the release binary
TARGET_PATH=target/release/$(BIN_NAME)

# List of environment variables to unset
UNSET_ENV=\
	unset RUST_FONTCONFIG_DLOPEN; \
	unset FONTCONFIG_NO_PKG_CONFIG; \
	unset PKG_CONFIG; \
	unset FONTCONFIG_STATIC; \
	unset FONTCONFIG_DYNAMIC; \
	unset PKG_CONFIG_ALL_STATIC; \
	unset PKG_CONFIG_ALL_DYNAMIC; \
	unset PKG_CONFIG_PATH; \
	unset PKG_CONFIG_LIBDIR; \
	unset PKG_CONFIG_SYSROOT_DIR; \
	unset HOST_PKG_CONFIG; \
	unset HOST_PKG_CONFIG_PATH; \
	unset HOST_PKG_CONFIG_LIBDIR; \
	unset HOST_PKG_CONFIG_SYSROOT_DIR; \
	unset PKG_CONFIG_PATH_x86_64_unknown_linux_gnu; \
	unset PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu; \
	unset PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu;

# Default target: build the release binary
all: prebuild $(TARGET_PATH)

prebuild:
	sudo apt update
	sudo apt install -y libfontconfig1-dev pkg-config

# How to build the binary with env vars unset
$(TARGET_PATH): prebuild
	@$(UNSET_ENV) cargo build --release --bin $(BIN_NAME)

ext1:
	@$(UNSET_ENV) cargo build --release --bin $(EXT1)
	./target/release/extension 20 20

ext2:
	trunk serve --open --port 8000
# Run the binary with env vars unset
run: $(TARGET_PATH)
	@$(UNSET_ENV) $(TARGET_PATH) $(ARGS)

docs: $(PDF_FILE)
	cargo doc --open

$(PDF_FILE): report.tex
	pdflatex report.tex \n \n

# Clean target
clean:
	cargo clean

coverage:
	cargo tarpaulin --exclude-files "src/extension/frontend/web.rs" --exclude-files "src/main.rs" --exclude-files "src/extension_main.rs"

test:
	@$(UNSET_ENV) cargo test --release