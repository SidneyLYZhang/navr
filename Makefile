# Navr Makefile
# Fast directory navigation tool - Build automation

.PHONY: all build test install clean fmt lint doc release dist check update help

# Default target
all: build

#==============================================================================
# Build Targets
#==============================================================================

# Build the project in release mode
build:
	@echo "Building navr in release mode..."
	cargo build --release

# Build in debug mode (faster compilation, slower runtime)
debug:
	@echo "Building navr in debug mode..."
	cargo build

# Run the application from source
run:
	cargo run --

# Run with verbose logging
run-verbose:
	RUST_LOG=debug cargo run --

#==============================================================================
# Testing
#==============================================================================

# Run all tests
test:
	cargo test

# Run tests with output
test-verbose:
	cargo test -- --nocapture

# Run benchmarks
bench:
	cargo bench

#==============================================================================
# Installation
#==============================================================================

# Install locally via cargo (requires cargo)
install:
	cargo install --path .

# Install with shell integration
install-full: install install-shell

# Install shell integration for detected shells
install-shell:
	@echo "Installing shell integration..."
	@for shell in bash zsh fish; do \
		if command -v $$shell >/dev/null 2>&1; then \
			echo "  → Installing for $$shell..."; \
			navr shell init $$shell >> ~/.tmp_navr_$$shell 2>/dev/null && \
			echo "    Run 'navr shell init $$shell' and add to your shell config manually" || \
			echo "    $$shell integration not available"; \
			rm -f ~/.tmp_navr_$$shell; \
		fi \
	done
	@echo ""
	@echo "Shell integration guide:"
	@echo "  bash:  eval \"\$$(navr shell init bash)\"  >> ~/.bashrc"
	@echo "  zsh:   eval \"\$$(navr shell init zsh)\"   >> ~/.zshrc"
	@echo "  fish:  navr shell init fish | source       >> ~/.config/fish/config.fish"

# Uninstall
uninstall:
	cargo uninstall navr

#==============================================================================
# Code Quality
#==============================================================================

# Format code with rustfmt
fmt:
	cargo fmt

# Check formatting without modifying files
fmt-check:
	cargo fmt -- --check

# Run clippy linter
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Run clippy with all features
lint-all:
	cargo clippy --all-targets --all-features -- -W clippy::all -D warnings

# Check code without building
check:
	cargo check

# Check all targets and features
check-all:
	cargo check --all-targets --all-features

#==============================================================================
# Documentation
#==============================================================================

# Generate documentation
doc:
	cargo doc --no-deps

# Generate and open documentation
doc-open:
	cargo doc --open --no-deps

#==============================================================================
# Release & Distribution
#==============================================================================

# Build optimized release binary
release:
	@echo "Building optimized release binary..."
	cargo build --release
ifeq ($(OS),Windows_NT)
	@echo "Stripping binary (Windows)..."
else
	@strip target/release/navr 2>/dev/null || echo "Strip not available"
endif

# Create distribution package
dist: release
	@echo "Creating distribution package..."
	@mkdir -p dist
	@cp target/release/navr dist/
	@cp README.md dist/
	@cp LICENSE dist/ 2>/dev/null || echo "LICENSE not found, skipping"
	@cp install.sh dist/
	@cp -r examples dist/ 2>/dev/null || true
ifeq ($(OS),Windows_NT)
	@powershell -Command "Compress-Archive -Path dist\* -DestinationPath navr-$(shell rustc --print cfg | grep target_arch | cut -d'"' -f2)-pc-windows-msvc.zip -Force"
else
	@tar czf navr-$(shell uname -s)-$(shell uname -m).tar.gz -C dist .
endif
	@rm -rf dist/
	@echo "Distribution package created"

#==============================================================================
# Development
#==============================================================================

# Setup development environment
dev-setup:
	rustup component add rustfmt clippy
	cargo install cargo-watch cargo-edit cargo-expand

# Watch for changes and rebuild
watch:
	cargo watch -x build

# Watch for changes and run tests
watch-test:
	cargo watch -x test

# Update dependencies
update:
	cargo update

# Update to latest compatible versions
update-latest:
	cargo update --aggressive

#==============================================================================
# Cleaning
#==============================================================================

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/
	rm -rf dist/
	rm -f navr-*.tar.gz navr-*.zip

# Clean everything including Cargo.lock (use with caution)
clean-all: clean
	rm -f Cargo.lock

#==============================================================================
# Platform-specific
#==============================================================================

# Build for specific targets
build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu

build-macos:
	cargo build --release --target x86_64-apple-darwin

build-windows:
	cargo build --release --target x86_64-pc-windows-msvc

# Cross-compilation (requires appropriate targets to be installed)
build-aarch64:
	cargo build --release --target aarch64-unknown-linux-gnu

#==============================================================================
# CI/CD Helpers
#==============================================================================

# Run all checks (for CI)
ci: fmt-check lint check test

# Full verification before release
verify: clean fmt lint check test build

#==============================================================================
# Help
#==============================================================================

# Show help
help:
	@echo "Navr Makefile targets:"
	@echo ""
	@echo "Build:"
	@echo "  build          - Build the project in release mode"
	@echo "  debug          - Build in debug mode (faster compilation)"
	@echo "  run            - Run the application from source"
	@echo "  run-verbose    - Run with verbose logging"
	@echo ""
	@echo "Testing:"
	@echo "  test           - Run all tests"
	@echo "  test-verbose   - Run tests with output"
	@echo "  bench          - Run benchmarks"
	@echo ""
	@echo "Installation:"
	@echo "  install        - Install locally via cargo"
	@echo "  install-full   - Install with shell integration"
	@echo "  install-shell  - Install shell integration"
	@echo "  uninstall      - Uninstall navr"
	@echo ""
	@echo "Code Quality:"
	@echo "  fmt            - Format code with rustfmt"
	@echo "  fmt-check      - Check formatting without modifying"
	@echo "  lint           - Run clippy linter"
	@echo "  check          - Check code without building"
	@echo ""
	@echo "Documentation:"
	@echo "  doc            - Generate documentation"
	@echo "  doc-open       - Generate and open documentation"
	@echo ""
	@echo "Release & Distribution:"
	@echo "  release        - Build optimized release binary"
	@echo "  dist           - Create distribution package"
	@echo ""
	@echo "Development:"
	@echo "  dev-setup      - Setup development environment"
	@echo "  watch          - Watch for changes and rebuild"
	@echo "  update         - Update dependencies"
	@echo ""
	@echo "Cleaning:"
	@echo "  clean          - Clean build artifacts"
	@echo "  clean-all      - Clean everything including Cargo.lock"
	@echo ""
	@echo "CI/CD:"
	@echo "  ci             - Run all checks (for CI)"
	@echo "  verify         - Full verification before release"
	@echo ""
	@echo "Help:"
	@echo "  help           - Show this help message"

# Detect OS
ifeq ($(OS),Windows_NT)
    DETECTED_OS := Windows
else
    DETECTED_OS := $(shell uname -s)
endif

# Print detected OS on startup
print-os:
	@echo "Detected OS: $(DETECTED_OS)"
