#!/bin/bash

# Setup script for git hooks configuration
# Sets the git hooks path and ensures execute permissions

echo "Setting up git hooks..."

# Set git hooks path to .githooks
echo "Setting git hooks path to .githooks..."
git config core.hooksPath .githooks

# Check if .githooks directory exists
if [ -d ".githooks" ]; then
    echo "Adding execute permissions to git hooks..."
    
    # Add execute permissions to all files in .githooks
    find .githooks -type f -exec chmod +x {} \;
    
    # List the hooks for verification
    echo "Git hooks in .githooks/:"
    ls -la .githooks/
else
    echo "Warning: .githooks directory not found"
    echo "You can create it with: mkdir .githooks"
fi

echo "Setup completed!"
echo "Git hooks path is now set to: $(git config core.hooksPath)"