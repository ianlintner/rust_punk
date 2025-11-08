#!/bin/bash
# Script to generate a screenshot of the game for CI/documentation

set -e

echo "Building rust_punk..."
cargo build --release

echo "Attempting to capture game screenshot..."

# Use script command to capture terminal output
# The game will run for 1 second then timeout, capturing initial state
(
  timeout 1s cargo run --release 2>&1 || true
) > game_output.txt

echo "Screenshot capture completed."
echo "Output saved to game_output.txt"

# Display captured content
if [ -f game_output.txt ]; then
  echo "=== Game Output ==="
  cat game_output.txt
  echo "==================="
fi
