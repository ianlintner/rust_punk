# rust_punk

**Rust Punk** - A CLI/Curses Roguelike Game

Fight rats and rival punks for dumpster scavenge rights behind a "Legally Distinct Burger Place" in this dystopian terminal adventure!

## Quick Start

```bash
cargo run --release
```

Use WASD to move, Space to attack, E to use items, Q to quit. Scavenge all 3 dumpsters and find bolt cutters to escape!

## Game Features

- 🎮 Terminal-based roguelike/lite gameplay
- ⌨️ Keyboard controls (WASD + Space/Enter)
- 🗑️ Scavenge dumpsters for survival
- ⚔️ Turn-based combat system
- 🐀 Multiple enemy types (Rats, Feral Cats, Rival Punks)
- 🎒 Inventory system with weapons, armor, and consumables
- 🎲 Roguelike random item generation with rarity system
- 🔧 Find bolt cutters to unlock the exit
- 💊 Consumable items with healing and buff effects
- 🎨 ASCII/Character-based graphics
- 🏆 Victory and defeat conditions

## About

*Rust Punk* is a play on "Crust Punk" - a terminal game where you fight over scavenge rights in a dystopian future. Navigate the dangerous area behind a burger joint, battle hostile creatures and rival scavengers, collect weapons and armor, and secure your prize: three precious dumpsters full of valuable trash! Find the bolt cutters to unlock the exit and escape with your haul.

See [GAMEPLAY.md](GAMEPLAY.md) for detailed instructions and strategy tips.

## Build & Run

```bash
# Build
cargo build --release

# Run
cargo run --release

# Check
cargo check
```

## Development

### Testing & Quality Checks

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy --all-targets --all-features -- -D warnings
```

### Continuous Integration

This project uses GitHub Actions for CI. All pull requests are automatically tested for:
- Code formatting (`cargo fmt`)
- Successful build (`cargo build`)
- Passing tests (`cargo test`)
- Linting (`cargo clippy`)
- Screenshot generation

See [.github/README.md](.github/README.md) for more details.

## Dependencies

- `crossterm` - Cross-platform terminal manipulation
- `rand` - Random number generation

## License

MIT (or your preferred license)
