# Rust Punk - Gameplay Guide

## Overview

**Rust Punk** (a play on "Crust Punk") is a terminal-based roguelike/lite game where you play as a scrappy punk fighting for dumpster scavenge rights behind a "Legally Distinct Burger Place."

## Story

You're a street punk trying to survive in a dystopian future. Your goal is to secure three dumpsters behind a burger joint for valuable scavenge. But you're not alone - rats, feral cats, and rival punks all want the same precious trash! Find bolt cutters to unlock the chained exit and escape with your loot!

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
- **E** - Use consumable item
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
- **⛓** - Chained Exit (need bolt cutters to unlock!)
- **▓** - Dumpster (scavenge these to win!)
- **r** - Rat (weak enemy, 20 HP, 3 damage)
- **c** - Feral Cat (medium enemy, 35 HP, 7 damage)
- **P** - Rival Punk (strong enemy, 50 HP, 10 damage)

### Objective

**Win Condition:** Scavenge all 3 dumpsters, find bolt cutters, and cut the chain at the exit

**Lose Condition:** Your HP drops to 0

### Gameplay Mechanics

#### Inventory System
You have a limited inventory with three slots:
- **Weapon Slot:** Increases your attack damage
- **Armor Slot:** Reduces damage taken from enemies
- **Consumable Slot:** Items you can use with 'E' key

Common items you'll find:
- **Weapons:** Broken Bottle, Rusty Chain, Spiked Bat, Crowbar, Switchblade, etc.
- **Armor:** Leather Jacket, Studded Vest, Motorcycle Helmet, Kevlar Vest, etc.
- **Consumables:** Burger Leftovers (heal), Energy Drink (damage boost), Painkillers (defense boost)

Items come in three rarities: Common, Uncommon, and Rare (with better bonuses)

#### Movement
- Move around the game world using WASD or arrow keys
- You cannot move through walls (#) or off the screen edges
- Moving takes one turn

#### Combat
- When you move into the same space as an enemy, combat begins
- During combat:
  - Press **Space** to attack the enemy
  - Press **E** to use a consumable item
  - Press **R** to retreat (moves you away from the enemy)
- Combat is turn-based: you attack, then the enemy counterattacks
- Your equipped weapon increases your attack damage
- Your equipped armor reduces incoming damage (minimum 1 damage)
- Defeat the enemy to continue exploring

#### Scavenging
- Move onto a dumpster (▓) to scavenge it
- Scavenging restores 20 HP
- Each dumpster contains random items:
  - Weapons (70% chance)
  - Armor (70% chance)
  - Consumables (80% chance)
  - One dumpster contains the Bolt Cutters you need!
- Items automatically fill empty inventory slots
- If your inventory is full, you'll see the item but can't pick it up
- Each dumpster can only be scavenged once (turns to ▒)
- Scavenge all 3 dumpsters to unlock the exit

#### Level Completion
- After scavenging all 3 dumpsters, the exit becomes accessible
- Navigate to the chained exit (⛓) at the top of the map
- You MUST have found the Bolt Cutters to unlock the chain
- Cut the chain to win the game!

#### Using Consumables
- Press **E** at any time to use your equipped consumable
- Healing items restore HP immediately
- Buff items provide temporary bonuses for several turns:
  - Energy Drink: +5 damage for 3 turns
  - Painkillers: +3 defense for 5 turns
  - Adrenaline Shot: +8 damage for 2 turns

#### Enemy AI
- Enemies move randomly towards you if you're within range
- Different enemy types have different stats:
  - **Rats:** Fast but weak
  - **Feral Cats:** Moderate threat
  - **Rival Punks:** Dangerous, guard the dumpsters

### Strategy Tips

1. **Scavenge first** - Hit dumpsters early to gear up before fighting tougher enemies
2. **Equip before fighting** - Weapons and armor make a huge difference in combat
3. **Save consumables** - Use healing items when HP is low, save buffs for tough fights
4. **Find bolt cutters** - You can't win without them, so prioritize finding all dumpsters
5. **Avoid unnecessary fights** - Retreat if your HP is low and you haven't found healing items
6. **Plan your route** - Try to reach dumpsters without fighting too many enemies
7. **Use buffs strategically** - Energy drinks are great against Rival Punks
8. **Fight weak enemies first** - Rats are easier to defeat than punks
9. **Watch enemy positions** - They move towards you, so keep your distance when low on HP
10. **Check inventory often** - Know what items you have available

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
