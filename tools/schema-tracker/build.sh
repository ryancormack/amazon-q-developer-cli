#!/bin/bash

# Build script for schema-tracker tool
set -e

echo "🔨 Building schema-tracker..."

# Build the tool
cargo build --release

echo "✅ Build complete!"
echo "📍 Binary location: target/release/schema-tracker"

# Make it executable
chmod +x target/release/schema-tracker

# Optionally create a symlink in the repo root for easy access
if [ ! -L "../../schema-tracker" ]; then
    echo "🔗 Creating symlink in repo root..."
    ln -s tools/schema-tracker/target/release/schema-tracker ../../schema-tracker
fi

echo ""
echo "🚀 Usage examples:"
echo "  ./schema-tracker capture -n 'Initial baseline'"
echo "  ./schema-tracker list"
echo "  ./schema-tracker diff schema1.json schema2.json"
echo "  ./schema-tracker analyze -p '**/*.json'"
echo ""
echo "📖 See tools/schema-tracker/README.md for full documentation"
