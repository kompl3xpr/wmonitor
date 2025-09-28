#!/bin/bash

# Build script for Rust project
# Steps:
# 1. Build release version using cargo
# 2. Create bin directory
# 3. Copy executable from target/release to bin
# 4. Copy database directory and config file to bin

echo "Starting build process..."

# Build release version
echo "Running cargo build --release..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

# Create bin directory
echo "Creating bin directory..."
mkdir -p bin

# Copy executable file
echo "Copying executable..."
cp "target/release/wmonitor" "bin/"

# Copy database directory
echo "Copying database directory..."
if [ -d "db" ]; then
    cp -r "db" "bin/"
else
    echo "Warning: db directory not found"
fi

# Copy config file
echo "Copying config file..."
if [ -f "cfg.toml" ]; then
    cp "cfg.toml" "bin/"
else
    echo "Warning: cfg.toml not found"
fi

echo "Build completed successfully!"

# Make the executable executable
chmod +x "bin/wmonitor"