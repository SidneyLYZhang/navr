# QuickNav Makefile

.PHONY: build test install clean fmt lint doc release install-shell

# Default target
all: build

# Build the project
build:
	cargo build --release

# Build in debug mode
debug:
	cargo build

# Run tests
test:
	cargo test

# Install locally (requires cargo)
install:
	cargo install --path .

# Install with shell integration
install-full: install install-shell

# Install shell integration
install-shell:
	@echo "Installing shell integration..."
	@quicknav shell install bash 2>/dev/null || echo "Bash not detected"
	@quicknav shell install zsh 2>/dev/null || echo "Zsh not detected"
	@quicknav shell install fish 2>/dev/null || echo "Fish not detected"

# Uninstall
uninstall:
	cargo uninstall navr

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/

# Format code
fmt:
	cargo fmt

# Run clippy linter
lint:
	cargo clippy -- -D warnings

# Generate documentation
doc:
	cargo doc --open

# Build for release
release:
	cargo build --release
	strip target/release/navr
	strip target/release/navr-shell

# Run the application
run:
	cargo run --

# Run with verbose logging
run-verbose:
	cargo run -- -v

# Create distribution package
dist: release
	mkdir -p dist
	cp target/release/navr dist/
	cp target/release/navr-shell dist/
	cp README.md dist/
	cp LICENSE dist/ 2>/dev/null || echo "LICENSE not found"
	cp install.sh dist/
	tar czf navr-$(shell uname -s)-$(shell uname -m).tar.gz -C dist .
	rm -rf dist/

# Setup development environment
dev-setup:
	rustup component add rustfmt clippy
	cargo install cargo-watch cargo-edit

# Watch for changes and rebuild
watch:
	cargo watch -x build

# Run benchmarks
bench:
	cargo bench

# Check code without building
check:
	cargo check

# Update dependencies
update:
	cargo update

# Show help
help:
	@echo "Navr Makefile targets:"
	@echo ""
	@echo "  build        - Build the project in release mode"
	@echo "  debug        - Build in debug mode"
	@echo "  test         - Run tests"
	@echo "  install      - Install locally via cargo"
	@echo "  install-full - Install with shell integration"
	@echo "  uninstall    - Uninstall navr"
	@echo "  clean        - Clean build artifacts"
	@echo "  fmt          - Format code with rustfmt"
	@echo "  lint         - Run clippy linter"
	@echo "  doc          - Generate and open documentation"
	@echo "  release      - Build optimized release binary"
	@echo "  dist         - Create distribution package"
	@echo "  run          - Run the application"
	@echo "  dev-setup    - Setup development environment"
	@echo "  watch        - Watch for changes and rebuild"
	@echo "  help         - Show this help message"
