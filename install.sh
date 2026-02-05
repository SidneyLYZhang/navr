#!/bin/bash
# Navr Installation Script

set -e

REPO_URL="https://github.com/sidneyzhang/navr"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR=""
SHELL_TYPE=""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_banner() {
    echo -e "${BLUE}"
    echo " _   _"
    echo "| \ | | __ ___   ___ __"
    echo "|  \| |/ _` \ \ / / '__|"
    echo "| |\  | (_| |\ V /| |"
    echo "|_| \_|\__,_| \_/ |_|"
    echo -e "${NC}"
    echo "Fast directory navigation tool"
    echo ""
}

detect_shell() {
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_TYPE="zsh"
    elif [ -n "$BASH_VERSION" ]; then
        SHELL_TYPE="bash"
    elif [ -f "$HOME/.config/fish/config.fish" ]; then
        SHELL_TYPE="fish"
    else
        SHELL_TYPE="bash"
    fi
    echo -e "${BLUE}Detected shell: ${SHELL_TYPE}${NC}"
}

detect_config_dir() {
    if [ "$(uname)" = "Darwin" ]; then
        CONFIG_DIR="$HOME/Library/Application Support/navr"
    elif [ "$(uname)" = "Linux" ]; then
        CONFIG_DIR="$HOME/.config/navr"
    else
        CONFIG_DIR="$HOME/.navr"
    fi
}

check_dependencies() {
    echo -e "${BLUE}Checking dependencies...${NC}"
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Rust/Cargo not found!${NC}"
        echo "Please install Rust first: https://rustup.rs/"
        exit 1
    fi
    
    echo -e "${GREEN}✓ Cargo found${NC}"
}

build_navr() {
    echo -e "${BLUE}Building navr...${NC}"
    
    if [ -d "navr" ]; then
        cd navr
        git pull
    else
        git clone "$REPO_URL" navr
        cd navr
    fi
    
    cargo build --release
    
    echo -e "${GREEN}✓ Build successful${NC}"
}

install_binary() {
    echo -e "${BLUE}Installing binary...${NC}"
    
    # Check if we need sudo
    if [ -w "$INSTALL_DIR" ]; then
        cp target/release/navr "$INSTALL_DIR/"
        cp target/release/navr-shell "$INSTALL_DIR/" 2>/dev/null || true
    else
        echo -e "${YELLOW}Requesting sudo access to install to $INSTALL_DIR${NC}"
        sudo cp target/release/navr "$INSTALL_DIR/"
        sudo cp target/release/navr-shell "$INSTALL_DIR/" 2>/dev/null || true
    fi
    
    echo -e "${GREEN}✓ Installed to $INSTALL_DIR/navr${NC}"
}

setup_config() {
    echo -e "${BLUE}Setting up configuration...${NC}"
    
    mkdir -p "$CONFIG_DIR"
    
    if [ ! -f "$CONFIG_DIR/config.toml" ]; then
        navr config reset
        echo -e "${GREEN}✓ Created default configuration${NC}"
    else
        echo -e "${YELLOW}Configuration already exists, skipping${NC}"
    fi
}

install_shell_integration() {
    echo -e "${BLUE}Installing shell integration...${NC}"
    
    case $SHELL_TYPE in
        bash)
            RC_FILE="$HOME/.bashrc"
            if ! grep -q "navr shell init" "$RC_FILE" 2>/dev/null; then
                echo 'eval "$(navr shell init bash)"' >> "$RC_FILE"
                echo -e "${GREEN}✓ Added to $RC_FILE${NC}"
            fi
            ;;
        zsh)
            RC_FILE="$HOME/.zshrc"
            if ! grep -q "navr shell init" "$RC_FILE" 2>/dev/null; then
                echo 'eval "$(navr shell init zsh)"' >> "$RC_FILE"
                echo -e "${GREEN}✓ Added to $RC_FILE${NC}"
            fi
            ;;
        fish)
            FISH_DIR="$HOME/.config/fish"
            FISH_CONFIG="$FISH_DIR/config.fish"
            mkdir -p "$FISH_DIR"
            if ! grep -q "navr shell init" "$FISH_CONFIG" 2>/dev/null; then
                echo 'navr shell init fish | source' >> "$FISH_CONFIG"
                echo -e "${GREEN}✓ Added to $FISH_CONFIG${NC}"
            fi
            ;;
    esac
}

print_next_steps() {
    echo ""
    echo -e "${GREEN}Installation complete!${NC}"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo ""
    echo "1. Reload your shell configuration:"
    echo -e "   ${YELLOW}source ~/$([ "$SHELL_TYPE" = "zsh" ] && echo ".zshrc" || [ "$SHELL_TYPE" = "fish" ] && echo ".config/fish/config.fish" || echo ".bashrc")${NC}"
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
    echo -e "For more information, visit: ${BLUE}$REPO_URL${NC}"
}

# Main installation flow
main() {
    print_banner
    
    detect_shell
    detect_config_dir
    check_dependencies
    
    echo ""
    read -p "Continue with installation? (Y/n) " -n 1 -r
    echo ""
    
    if [[ $REPLY =~ ^[Nn]$ ]]; then
        echo "Installation cancelled."
        exit 0
    fi
    
    build_navr
    install_binary
    setup_config
    install_shell_integration
    
    print_next_steps
}

# Run main function
main "$@"
