#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "--- Starting HackerNews Data Pipeline ---"

# --- Step 1: Run the Rust Scraper ---
echo "1. Running Rust scraper..."
# Ensure you are in the project root or adjust paths accordingly
cargo run --manifest-path ./scraper/Cargo.toml

echo "Rust scraper finished."

# --- Step 2: Run the Python Analyzer ---
echo "2. Running Python analyzer to load data into DuckDB..."
# Ensure Python is in your PATH or specify the full path to the interpreter
python analysis/main.py

echo "Python analyzer finished."

echo "--- Data Pipeline Completed Successfully ---"