# Project Status: HackerNews Data Pipeline

## Overall Goal
The project is a data pipeline that scrapes data from a news website, saves it to a JSONL file, and then uses Python with DuckDB for analysis.

## Technology Stack
- **Environment:** NixOS with a Flake (`flake.nix`) for reproducible builds.
- **Scraper:** Rust
- **Analysis:** Python 3
- **Database:** DuckDB

## Current Status
- **Rust Scraper (`scraper/src/main.rs`):** The scraper has been updated to fetch HTML directly from `https://news.ycombinator.com`, save it to `data/hackernews.html`, and then extract story data. It now appends only *unique* stories to `data/hackernews.jsonl`, preventing duplicates across runs.
- **Python Analyzer (`analysis/main.py`):** The Python script has been updated to connect to a DuckDB database (`data/hackernews.db`) and efficiently load the `data/hackernews.jsonl` file into a table named `stories`.
- **Development Environment:** The `nix develop` shell is working correctly.
- **Core Pipeline:** The full data pipeline (scraping, deduplication, and loading into DuckDB) is functional.

## Immediate Next Step
The core data pipeline is complete. The next step is to build a user interface to display and interact with the scraped data. This will involve:
1.  Creating a simple one-page web application (or TUI).
2.  Setting up a backend (e.g., Python/Flask) to serve data from the DuckDB database.
3.  Implementing a frontend (HTML/CSS/JavaScript) to fetch and display the stories.
4.  Adding features like fuzzy search and basic trend visualization.