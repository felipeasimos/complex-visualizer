#!/bin/bash
set -e

# Install Rust
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Verify cargo is available
cargo --version
wasm-pack --version || cargo install wasm-pack

# Build WASM
wasm-pack build --target web --out-dir ./www/pkg
