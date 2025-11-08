# GitHub Actions CI/CD

This directory contains GitHub Actions workflows for continuous integration and continuous deployment.

## Workflows

### CI Workflow (`ci.yml`)

Runs on every push to `main` and on all pull requests targeting `main`.

**Jobs:**

1. **Formatting** - Checks Rust code formatting with `cargo fmt --check`
2. **Build** - Builds the project in release mode with `cargo build --release`
3. **Test** - Runs all tests with `cargo test`
4. **Clippy** - Runs Rust linter with `cargo clippy` (treats warnings as errors)
5. **Screenshot** - Generates a game screenshot artifact for documentation

All jobs must pass for a PR to be merged.

## Local Testing

Before pushing changes, run these commands locally to ensure CI will pass:

```bash
# Check formatting
cargo fmt --check

# Build
cargo build --release

# Run tests
cargo test

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Generate screenshot
./scripts/screenshot.sh
```

## Screenshot Generation

The screenshot job captures initial game output for documentation purposes. The output is saved as an artifact that can be downloaded from the GitHub Actions run.
