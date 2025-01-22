#!/usr/bin/env sh

set -e

while true; do
  echo
  echo '┌───────────────────────────────────────────────────────┐'
  echo "│ Press Enter to run 'cargo run'. Press Ctrl+C to exit. │"
  echo '└───────────────────────────────────────────────────────┘'
  read -r _   # Wait for Enter key
  clear       # Optional: Clear the screen before running
  cargo run   # Run the desired cargo command
done
