#!/bin/bash
# Navr Installation Script
# Cross-platform installation script for navr - a fast directory navigation tool

set -euo pipefail

# Configuration
REPO_URL="https://github.com/sidneylyzhang/navr"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
CONFIG_DIR=""
SHELL_TYPE=""
BUILD_DIR=""

# Colors (with fallback for terminals that don't support colors)
if [[ -t 1 ]]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    NC='\033[0m'
else
    RED='' GREEN='' YELLOW='' BLUE='' CYAN='' BOLD='' NC=''
fi

# Logging functions
log_info() { echo -e "${BLUE}ℹ${NC} $1"; }
log_success() { echo -e "${GREEN}✓${NC} $1"; }
log_warn() { echo -e "${YELLOW}⚠${NC} $1"; }
log_error() { echo -e "${RED}✗${NC} $1" >&2; }
log_step() { echo -e "${CYAN}${BOLD}→${NC} $1"; }

print_banner() {
    echo -e "${CYAN}"
    echo " _   _"
    echo "| \\ | | __ ___   ___ __"
    echo "|  \\| |/ _\` \\ \\ / / '__|"
    echo "| |\\  | (_| |\\ V /| |"
    echo "|_| \\_|\\__,_| \\_/ |_|"
    echo -e "${NC}"
    echo -e "${BOLD}Fast directory navigation tool${NC}"
    echo ""
}

detect_os() {
    local os=""
    case "$(uname -s)" in
        Linux*)     os="linux";;
        Darwin*)    os="macos";;
        CYGWIN*|MINGW*|MSYS*) os="windows";;
        *)          os="unknown";;
    esac
    echo "$os"
}

detect_arch() {
    local arch=""
    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64";;
        arm64|aarch64)  arch="aarch64";;
        i386|i686)      arch="i686";;
        *)              arch="unknown";;
    esac
    echo "$arch"
}

detect_shell() {
    # Check parent process name if SHELL variable is not set
    if [[ -z "${SHELL:-}" ]] && command -v ps &>/dev/null; then
        SHELL=$(ps -p $$ -o comm= 2>/dev/null || echo "")
    fi

    if [[ -n "${ZSH_VERSION:-}" ]] || [[ "${SHELL:-}" == *"zsh"* ]]; then
        SHELL_TYPE="zsh"
    elif [[ -n "${BASH_VERSION:-}" ]] || [[ "${SHELL:-}" == *"bash"* ]]; then
        SHELL_TYPE="bash"
    elif [[ "${SHELL:-}" == *"fish"* ]] || command -v fish &>/dev/null; then
        SHELL_TYPE="fish"
    else
        # Default to bash if detection fails
        SHELL_TYPE="bash"
        log_warn "Could not detect shell type, defaulting to bash"
    fi
    log_info "Detected shell: ${BOLD}${SHELL_TYPE}${NC}"
}

detect_config_dir() {
    local os
    os=$(detect_os)

    case "$os" in
        macos)
            CONFIG_DIR="${HOME}/Library/Application Support/navr"
            ;;
        linux)
            CONFIG_DIR="${XDG_CONFIG_HOME:-${HOME}/.config}/navr"
            ;;
        windows)
            CONFIG_DIR="${APPDATA:-${HOME}/AppData/Roaming}/navr"
            ;;
        *)
            CONFIG_DIR="${HOME}/.navr"
            ;;
    esac
}

check_dependencies() {
    log_step "Checking dependencies..."

    local missing_deps=()

    if ! command -v cargo &>/dev/null; then
        missing_deps+=("cargo")
    fi

    if ! command -v git &>/dev/null; then
        missing_deps+=("git")
    fi

    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_error "Missing required dependencies: ${missing_deps[*]}"
        echo ""
        echo "Please install the missing dependencies:"
        echo "  - Rust/Cargo: https://rustup.rs/"
        echo "  - Git: https://git-scm.com/downloads"
        exit 1
    fi

    log_success "All dependencies found"
}

get_build_dir() {
    # Check if we're already in the navr repository
    if [[ -f "Cargo.toml" ]] && grep -q "^name = \"navr\"" "Cargo.toml" 2>/dev/null; then
        BUILD_DIR="."
        log_info "Building from current directory"
    elif [[ -d "navr" ]] && [[ -f "navr/Cargo.toml" ]]; then
        BUILD_DIR="navr"
        log_info "Building from existing navr/ directory"
    else
        BUILD_DIR="navr"
        log_info "Will clone repository to navr/"
    fi
}

build_navr() {
    log_step "Building navr..."

    local start_dir
    start_dir=$(pwd)

    if [[ "$BUILD_DIR" == "." ]]; then
        # Already in the repo
        git pull --ff-only 2>/dev/null || log_warn "Could not pull latest changes"
    elif [[ -d "$BUILD_DIR" ]]; then
        # Existing directory, update it
        cd "$BUILD_DIR"
        git pull --ff-only 2>/dev/null || log_warn "Could not pull latest changes"
    else
        # Clone fresh
        git clone --depth 1 "$REPO_URL" "$BUILD_DIR"
        cd "$BUILD_DIR"
    fi

    # Build with optimizations
    cargo build --release

    cd "$start_dir"
    log_success "Build successful"
}

install_binary() {
    log_step "Installing binary..."

    local binary_path="${BUILD_DIR}/target/release/navr"

    if [[ ! -f "$binary_path" ]]; then
        log_error "Binary not found at $binary_path"
        log_info "Did the build complete successfully?"
        exit 1
    fi

    # Check if we need sudo for the install directory
    if [[ -w "$INSTALL_DIR" ]] || [[ ! -d "$INSTALL_DIR" && -w "$(dirname "$INSTALL_DIR")" ]]; then
        mkdir -p "$INSTALL_DIR" 2>/dev/null || true
        cp "$binary_path" "$INSTALL_DIR/"
    else
        log_warn "Requesting sudo access to install to $INSTALL_DIR"
        sudo mkdir -p "$INSTALL_DIR"
        sudo cp "$binary_path" "$INSTALL_DIR/"
    fi

    # Verify installation
    if [[ -f "${INSTALL_DIR}/navr" ]]; then
        log_success "Installed to ${INSTALL_DIR}/navr"
    else
        log_error "Installation verification failed"
        exit 1
    fi

    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        log_warn "${INSTALL_DIR} is not in your PATH"
        echo "   Add the following to your shell configuration:"
        echo "   export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi
}

setup_config() {
    log_step "Setting up configuration..."

    mkdir -p "$CONFIG_DIR"

    if [[ ! -f "${CONFIG_DIR}/config.toml" ]]; then
        # Initialize default config using navr itself
        if "${INSTALL_DIR}/navr" config reset 2>/dev/null; then
            log_success "Created default configuration at ${CONFIG_DIR}"
        else
            log_warn "Could not create default configuration"
            log_info "Run 'navr config reset' manually after installation"
        fi
    else
        log_warn "Configuration already exists, skipping"
    fi
}

install_shell_integration() {
    log_step "Installing shell integration..."

    local rc_file=""
    local integration_line=""

    case $SHELL_TYPE in
        bash)
            rc_file="${HOME}/.bashrc"
            # Also check for .bash_profile on macOS
            if [[ "$(detect_os)" == "macos" ]] && [[ -f "${HOME}/.bash_profile" ]]; then
                rc_file="${HOME}/.bash_profile"
            fi
            integration_line='eval "$(navr shell init bash)"'
            ;;
        zsh)
            rc_file="${HOME}/.zshrc"
            integration_line='eval "$(navr shell init zsh)"'
            ;;
        fish)
            rc_file="${HOME}/.config/fish/config.fish"
            mkdir -p "$(dirname "$rc_file")"
            integration_line='navr shell init fish | source'
            ;;
    esac

    if [[ -z "$rc_file" ]]; then
        log_warn "Could not determine shell configuration file"
        return 0
    fi

    # Check if already installed
    if [[ -f "$rc_file" ]] && grep -q "navr shell init" "$rc_file" 2>/dev/null; then
        log_warn "Shell integration already exists in ${rc_file}"
        return 0
    fi

    # Add integration
    echo "" >> "$rc_file"
    echo "# navr - fast directory navigation" >> "$rc_file"
    echo "$integration_line" >> "$rc_file"

    log_success "Added shell integration to ${rc_file}"
}

print_next_steps() {
    local rc_file=""
    case $SHELL_TYPE in
        bash)
            rc_file="~/.bashrc"
            [[ "$(detect_os)" == "macos" ]] && [[ -f "${HOME}/.bash_profile" ]] && rc_file="~/.bash_profile"
            ;;
        zsh)
            rc_file="~/.zshrc"
            ;;
        fish)
            rc_file="~/.config/fish/config.fish"
            ;;
    esac

    echo ""
    echo -e "${GREEN}${BOLD}Installation complete!${NC}"
    echo ""
    echo -e "${CYAN}${BOLD}Next steps:${NC}"
    echo ""
    echo "1. Reload your shell configuration:"
    echo -e "   ${YELLOW}source ${rc_file}${NC}"
    echo ""
    echo "2. Add your first shortcut:"
    echo -e "   ${YELLOW}cd /path/to/project${NC}"
    echo -e "   ${YELLOW}navr jump --add myproject${NC}"
    echo ""
    echo "3. Jump to your shortcut:"
    echo -e "   ${YELLOW}j myproject${NC}"
    echo ""
    echo "4. View all shortcuts:"
    echo -e "   ${YELLOW}navr jump --list${NC}"
    echo ""
    echo "5. Get help:"
    echo -e "   ${YELLOW}navr --help${NC}"
    echo ""
    echo -e "For more information, visit: ${BLUE}${REPO_URL}${NC}"
}

# Cleanup function for trap
cleanup() {
    local exit_code=$?
    if [[ $exit_code -ne 0 ]] && [[ -n "${BUILD_DIR:-}" ]] && [[ "$BUILD_DIR" == "navr" ]] && [[ -d "$BUILD_DIR" ]]; then
        echo ""
        log_info "Build directory retained at: $(pwd)/${BUILD_DIR}"
        log_info "You can manually remove it with: rm -rf ${BUILD_DIR}"
    fi
    exit $exit_code
}

trap cleanup EXIT

# Help message
show_help() {
    cat << EOF
Navr Installation Script

Usage: $0 [OPTIONS]

Options:
    -h, --help          Show this help message
    -d, --dir DIR       Install directory (default: /usr/local/bin)
    --no-shell          Skip shell integration
    --no-config         Skip configuration setup

Environment Variables:
    INSTALL_DIR         Override default install directory

Examples:
    $0                  # Default installation
    $0 -d ~/.local/bin  # Install to ~/.local/bin
    $0 --no-shell       # Install without shell integration
EOF
}

# Parse command line arguments
SKIP_SHELL=false
SKIP_CONFIG=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -d|--dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --no-shell)
            SKIP_SHELL=true
            shift
            ;;
        --no-config)
            SKIP_CONFIG=true
            shift
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Main installation flow
main() {
    print_banner

    local os arch
    os=$(detect_os)
    arch=$(detect_arch)

    log_info "OS: ${BOLD}${os}${NC} | Architecture: ${BOLD}${arch}${NC}"
    echo ""

    detect_shell
    detect_config_dir
    check_dependencies
    get_build_dir

    echo ""
    echo "Installation summary:"
    echo "  Install directory: ${BOLD}${INSTALL_DIR}${NC}"
    echo "  Config directory:  ${BOLD}${CONFIG_DIR}${NC}"
    echo "  Shell type:        ${BOLD}${SHELL_TYPE}${NC}"
    echo ""

    read -r -p "Continue with installation? (Y/n) " -n 1
    echo ""

    if [[ "$REPLY" =~ ^[Nn]$ ]]; then
        log_info "Installation cancelled."
        exit 0
    fi

    echo ""
    build_navr
    install_binary

    if [[ "$SKIP_CONFIG" != true ]]; then
        setup_config
    fi

    if [[ "$SKIP_SHELL" != true ]]; then
        install_shell_integration
    fi

    print_next_steps
}

# Run main function
main "$@"
