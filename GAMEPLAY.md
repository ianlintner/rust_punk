# Rust Punk - Gameplay Guide

## Overview

**Rust Punk** (a play on "Crust Punk") is a terminal-based roguelike/lite game where you play as a scrappy punk fighting for dumpster scavenge rights behind a "Legally Distinct Burger Place."

## Story

You're a street punk trying to survive in a dystopian future. Your goal is to secure three dumpsters behind a burger joint for valuable scavenge. But you're not alone - rats, feral cats, and rival punks all want the same precious trash!

## How to Play

### Installation & Running

```bash
# Clone the repository
git clone https://github.com/ianlintner/rust_punk.git
cd rust_punk

# Build and run
cargo run --release
```

### Controls

- **W** / **↑** - Move up
- **S** / **↓** - Move down
- **A** / **←** - Move left
- **D** / **→** - Move right
- **Space** / **Enter** - Attack (during combat)
- **R** - Retreat from combat
- **Q** / **Esc** - Quit game

### Game Screen

```
╔════════════════════════════════════════════════════════════╗
║    RUST PUNK - Behind Legally Distinct Burger Place      ║
╠════════════════════════════════════════════════════════════╣
║############################################################║
║#                                                          #║
║#              ▓              ▓              ▓             #║  <- Dumpsters to scavenge
║#                                                          #║
║#                      P           c                       #║  <- Enemies
║#                           r                              #║
║#                  r                       r               #║
║#                                                          #║
║#                          @                               #║  <- You!
║#                                                          #║
║############################################################║
╠════════════════════════════════════════════════════════════╣
║ HP: 100/100 | Scavenged: 0/3 | Turn: 0                   ║
║ Welcome to RUST PUNK!                                     ║
║ Fight rats and rivals for dumpster scavenge rights!       ║
║ @ = You | r = Rat | c = Cat | P = Punk | ▓ = Dumpster   ║
╚════════════════════════════════════════════════════════════╝
```

### Characters & Enemies

- **@** - You (the player)
- **▓** - Dumpster (scavenge these to win!)
- **r** - Rat (weak enemy, 20 HP, 3 damage)
- **c** - Feral Cat (medium enemy, 35 HP, 7 damage)
- **P** - Rival Punk (strong enemy, 50 HP, 10 damage)

### Objective

**Win Condition:** Scavenge all 3 dumpsters

**Lose Condition:** Your HP drops to 0

### Gameplay Mechanics

#### Movement
- Move around the game world using WASD or arrow keys
- You cannot move through walls (#) or off the screen edges
- Moving takes one turn

#### Combat
- When you move into the same space as an enemy, combat begins
- During combat:
  - Press **Space** to attack the enemy
  - Press **R** to retreat (moves you away from the enemy)
- Combat is turn-based: you attack, then the enemy counterattacks
- Defeat the enemy to continue exploring

#### Scavenging
- Move onto a dumpster (▓) to scavenge it
- Scavenging restores 20 HP
- Each dumpster can only be scavenged once (turns to ▒)
- Scavenge all 3 dumpsters to win!

#### Enemy AI
- Enemies move randomly towards you if you're within range
- Different enemy types have different stats:
  - **Rats:** Fast but weak
  - **Feral Cats:** Moderate threat
  - **Rival Punks:** Dangerous, guard the dumpsters

### Strategy Tips

1. **Avoid unnecessary fights** - Retreat if your HP is low
2. **Plan your route** - Try to reach dumpsters without fighting too many enemies
3. **Use healing** - Each dumpster restores 20 HP, so time your scavenging
4. **Fight weak enemies first** - Rats are easier to defeat than punks
5. **Watch enemy positions** - They move towards you, so keep your distance

### Victory Screen

```
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║  ██╗   ██╗██╗ ██████╗████████╗ ██████╗ ██████╗ ██╗   ██╗ ║
║  ██║   ██║██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗╚██╗ ██╔╝ ║
║  ██║   ██║██║██║        ██║   ██║   ██║██████╔╝ ╚████╔╝  ║
║  ╚██╗ ██╔╝██║██║        ██║   ██║   ██║██╔══██╗  ╚██╔╝   ║
║   ╚████╔╝ ██║╚██████╗   ██║   ╚██████╔╝██║  ██║   ██║    ║
║    ╚═══╝  ╚═╝ ╚═════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝   ╚═╝    ║
║                                                            ║
║          You've secured the dumpsters!                    ║
║          The scavenge rights are yours!                   ║
╚════════════════════════════════════════════════════════════╝
```

## Technical Details

- **Language:** Rust
- **UI Library:** Crossterm (terminal/curses-style interface)
- **Random Generation:** rand crate
- **Style:** ASCII/text-based roguelike

## Credits

A dystopian adventure in scavenging and survival. Fight for your right to trash!

**Rust Punk** - Because even in the apocalypse, punks gotta eat.
