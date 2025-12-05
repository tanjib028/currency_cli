# Currency CLI

A fast, privacy-friendly currency converter for your terminal. Fetch live or historical exchange rates, cache locally, and convert across currencies without ads or bloat.

## Features
- Convert between any two currencies
- Multi-target conversions
- Historical date support (YYYY-MM-DD)
- Local caching for offline use
- Output formats: plain, JSON, CSV
- Cross-platform binaries (macOS Apple Silicon + Intel)

## Install
Download binaries from Releases, or build from source:
```bash
git clone https://github.com/youruser/currency-cli
cd currency-cli
cargo build --release
./target/release/currency-cli --help