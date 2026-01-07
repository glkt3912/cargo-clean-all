#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🚀 Installing cargo-clean-all..."

# Check if running on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}Error: This tool is designed for macOS only.${NC}"
    exit 1
fi

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed. Please install Rust first.${NC}"
    echo "Visit: https://rustup.rs/"
    exit 1
fi

# Install the binary
echo "📦 Building and installing cargo-clean-all..."
cargo install --path .

# Create config directory
echo "📁 Creating config directory..."
mkdir -p ~/.config/cargo-clean-all

# Copy example config if config doesn't exist
if [ ! -f ~/.config/cargo-clean-all/config.toml ]; then
    echo "📝 Creating default configuration..."
    cp config.example.toml ~/.config/cargo-clean-all/config.toml
    echo -e "${YELLOW}⚠️  Please edit ~/.config/cargo-clean-all/config.toml to set your scan paths.${NC}"
else
    echo -e "${GREEN}✓${NC} Configuration file already exists."
fi

# Create log directory
mkdir -p ~/.local/share/cargo-clean-all

# Ask if user wants to install LaunchAgent
echo ""
read -p "Install LaunchAgent for automatic weekly cleanup? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🤖 Installing LaunchAgent..."

    # Create LaunchAgents directory if it doesn't exist
    mkdir -p ~/Library/LaunchAgents

    # Generate plist from template
    sed "s|{{HOME}}|$HOME|g" com.user.cargo-clean-all.plist.template > ~/Library/LaunchAgents/com.user.cargo-clean-all.plist

    # Load the agent
    launchctl unload ~/Library/LaunchAgents/com.user.cargo-clean-all.plist 2>/dev/null || true
    launchctl load ~/Library/LaunchAgents/com.user.cargo-clean-all.plist

    echo -e "${GREEN}✓${NC} LaunchAgent installed and loaded."
    echo "   The cleaner will run every Sunday at 2 AM."
else
    echo "Skipped LaunchAgent installation."
fi

echo ""
echo -e "${GREEN}✅ Installation complete!${NC}"
echo ""
echo "Usage:"
echo "  cargo-clean-all --dry-run    # Preview what will be cleaned"
echo "  cargo-clean-all              # Run cleanup"
echo "  cargo-clean-all --verbose    # Verbose output"
echo ""
echo "Configuration: ~/.config/cargo-clean-all/config.toml"
echo "Logs: ~/.local/share/cargo-clean-all/clean.log"
