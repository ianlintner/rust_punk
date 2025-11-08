# Rust Punk - Example Gameplay Scenarios

## Starting the Game

When you launch Rust Punk, you'll see:

```
╔══════════════════════════════════════════════════════════╗
║    RUST PUNK - Behind Legally Distinct Burger Place    ║
╠══════════════════════════════════════════════════════════╣
║############################################################║
║#                                                          #║
║#                                                          #║
║#              ▓              ▓              ▓             #║  <- 3 Dumpsters at the top
║#                                                          #║
║#                                  P               r       #║  <- Rival Punk and Rat
║#                                                          #║
║#                                                          #║
║#                       c                                  #║  <- Feral Cat
║#                                                          #║
║#                                                          #║
║#                                                r         #║  <- More Rats
║#                                              r           #║
║#                                                          #║
║#                                                          #║
║#                                                          #║
║#                                                          #║
║#                             @                            #║  <- You start here!
║#                                                          #║
║############################################################║
╠══════════════════════════════════════════════════════════╣
║ HP: 100/100 | Scavenged: 0/3 | Turn: 0                   ║
║ Welcome to RUST PUNK!                                    ║
║ Fight rats and rivals for dumpster scavenge rights!      ║
║ Use WASD to move, Space to attack, Q to quit             ║
║ Scavenge 3 dumpsters to win!                             ║
║ @ = You | r = Rat | c = Cat | P = Punk | ▓ = Dumpster  ║
╚══════════════════════════════════════════════════════════╝
```

## Combat Encounter

When you encounter an enemy:

```
╔══════════════════════════════════════════════════════════╗
║    RUST PUNK - Behind Legally Distinct Burger Place    ║
╠══════════════════════════════════════════════════════════╣
║ HP: 90/100 | Scavenged: 1/3 | Turn: 15                   ║
║ COMBAT! Rat HP: 10/20                                    ║  <- Combat Status
║ Encountered Rat! Space to attack, R to retreat.          ║
║ You attack Rat for 10 damage!                            ║
║ Rat attacks you for 3 damage!                            ║
║ @ = You | r = Rat | c = Cat | P = Punk | ▓ = Dumpster  ║
╚══════════════════════════════════════════════════════════╝
```

## Scavenging Success

After scavenging a dumpster:

```
║ HP: 100/100 | Scavenged: 2/3 | Turn: 23                  ║
║ Scavenged dumpster! (2/3)                                ║
║ Found some food! Health restored.                        ║
║ Moving towards next dumpster...                          ║
```

## Victory Screen

When you scavenge all 3 dumpsters:

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
║                                                            ║
║          Final Stats: 85 HP | 42 turns                    ║
║                                                            ║
║          Press Q to quit                                   ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
```

## Game Over Screen

If you're defeated:

```
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║   ██████╗  █████╗ ███╗   ███╗███████╗                     ║
║  ██╔════╝ ██╔══██╗████╗ ████║██╔════╝                     ║
║  ██║  ███╗███████║██╔████╔██║█████╗                       ║
║  ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝                       ║
║  ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗                     ║
║   ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝                     ║
║   ██████╗ ██╗   ██╗███████╗██████╗                        ║
║  ██╔═══██╗██║   ██║██╔════╝██╔══██╗                       ║
║  ██║   ██║██║   ██║█████╗  ██████╔╝                       ║
║  ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗                       ║
║  ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║                       ║
║   ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝                       ║
║                                                            ║
║          You've been defeated!                            ║
║          The dumpsters are lost...                        ║
║                                                            ║
║          Survived 35 turns | Scavenged: 1/3               ║
║                                                            ║
║          Press Q to quit                                   ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
```

## Typical Gameplay Flow

1. **Start**: You begin at the bottom center of the screen
2. **Explore**: Use WASD to navigate towards dumpsters
3. **Avoid/Fight**: Enemies move towards you - fight or flee
4. **Scavenge**: Reach dumpsters to collect items and heal
5. **Win**: Collect all 3 dumpsters before being defeated
6. **Lose**: If your HP reaches 0, game over!

## Strategy Tips

- **Rats (r)**: Weakest enemies, easiest to defeat
- **Feral Cats (c)**: Moderate threat, be cautious
- **Rival Punks (P)**: Strongest enemies, guard the dumpsters
- Each dumpster heals you 20 HP, so plan your route!
- You can retreat from combat if things get dangerous
- Enemies move randomly towards you, so keep moving!
