#!/bin/bash

# Function to display messages
function echo_msg {
    echo -e "\e[32m$1\e[0m"
}

# Function to display error messages
function echo_err {
    echo -e "\e[31m$1\e[0m"
}

# Step 1: Update package index
echo_msg "Updating package index..."
if sudo apt update; then
    echo_msg "Package index updated successfully."
else
    echo_err "Failed to update package index."
    exit 1
fi

# Step 2: Install necessary dependencies
echo_msg "Installing necessary dependencies..."
if sudo apt install -y curl build-essential; then
    echo_msg "Dependencies installed successfully."
else
    echo_err "Failed to install dependencies."
    exit 1
fi

# Step 3: Download and install Rustup (Rust toolchain installer)
echo_msg "Downloading and installing Rustup..."
if curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; then
    echo_msg "Rustup installed successfully."
else
    echo_err "Failed to install Rustup."
    exit 1
fi

# Step 4: Configure the current shell to use Rust environment
echo_msg "Configuring shell to use Rust environment..."
source $HOME/.cargo/env

# Step 5: Verify the installation
echo_msg "Verifying Rust installation..."
if rustc --version && cargo --version; then
    echo_msg "Rust installed successfully. Versions:"
    rustc --version
    cargo --version
else
    echo_err "Failed to verify Rust installation."
    exit 1
fi

# Step 6: Configure Rust toolchain
echo_msg "Configuring Rust toolchain..."
rustup default stable
rustup update

# Step 7: Add useful Rust components
echo_msg "Adding useful Rust components..."
rustup component add clippy
rustup component add rustfmt

# Step 8: Display final message
echo_msg "Rust installation and configuration complete!"


