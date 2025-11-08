use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{Result, Write, stdout};

use crate::game::{GameMode, GameState};

pub struct Renderer {
    pub width: u16,
    #[allow(dead_code)]
    pub height: u16,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Renderer { width, height }
    }

    pub fn clear_screen(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))
    }

    pub fn render(&self, game: &GameState) -> Result<()> {
        self.clear_screen()?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        match game.mode {
            GameMode::Victory => self.render_victory(game),
            GameMode::GameOver => self.render_game_over(game),
            GameMode::LevelComplete => self.render_game_world(game),
            _ => self.render_game_world(game),
        }
    }

    fn render_game_world(&self, game: &GameState) -> Result<()> {
        // Draw top border and title
        execute!(stdout(), SetForegroundColor(Color::Red), Print("╔"),)?;
        for _ in 0..self.width - 2 {
            execute!(stdout(), Print("═"))?;
        }
        execute!(stdout(), Print("╗"), ResetColor, Print("\r\n"))?;

        // Draw title
        let title = "    RUST PUNK - Behind Legally Distinct Burger Place    ";
        execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print("║"),
            ResetColor,
            SetForegroundColor(Color::Yellow),
            Print(title),
            ResetColor,
            SetForegroundColor(Color::Red),
            Print("║"),
            ResetColor,
            Print("\r\n")
        )?;

        // Draw game area border top
        execute!(stdout(), SetForegroundColor(Color::DarkGrey), Print("╠"),)?;
        for _ in 0..self.width - 2 {
            execute!(stdout(), Print("═"))?;
        }
        execute!(stdout(), Print("╣"), ResetColor, Print("\r\n"))?;

        // Draw game world
        for y in 0..game.height {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkGrey),
                Print("║"),
                ResetColor
            )?;

            for x in 0..game.width {
                let mut rendered = false;

                // Draw player
                if game.player.position.x == x && game.player.position.y == y {
                    execute!(
                        stdout(),
                        SetForegroundColor(Color::Green),
                        Print("@"),
                        ResetColor
                    )?;
                    rendered = true;
                }

                // Draw enemies
                if !rendered {
                    for enemy in &game.enemies {
                        if enemy.is_alive && enemy.position.x == x && enemy.position.y == y {
                            let color = match enemy.enemy_type {
                                crate::game::EnemyType::Rat => Color::DarkYellow,
                                crate::game::EnemyType::FerralCat => Color::Magenta,
                                crate::game::EnemyType::RivalPunk => Color::Red,
                            };
                            execute!(
                                stdout(),
                                SetForegroundColor(color),
                                Print(enemy.get_char()),
                                ResetColor
                            )?;
                            rendered = true;
                            break;
                        }
                    }
                }

                // Draw dumpsters
                if !rendered {
                    for dumpster in &game.dumpsters {
                        if dumpster.position.x == x && dumpster.position.y == y {
                            if dumpster.has_items {
                                execute!(
                                    stdout(),
                                    SetForegroundColor(Color::Cyan),
                                    Print("▓"),
                                    ResetColor
                                )?;
                            } else {
                                execute!(
                                    stdout(),
                                    SetForegroundColor(Color::DarkGrey),
                                    Print("▒"),
                                    ResetColor
                                )?;
                            }
                            rendered = true;
                            break;
                        }
                    }
                }

                // Draw chain (exit)
                if !rendered && game.chain_position.x == x && game.chain_position.y == y {
                    let chain_color = if game.player.inventory.bolt_cutters.found {
                        Color::Green
                    } else {
                        Color::Yellow
                    };
                    execute!(
                        stdout(),
                        SetForegroundColor(chain_color),
                        Print("⛓"),
                        ResetColor
                    )?;
                    rendered = true;
                }

                // Draw borders
                if !rendered && (x == 0 || x == game.width - 1 || y == 0 || y == game.height - 1) {
                    execute!(
                        stdout(),
                        SetForegroundColor(Color::DarkGrey),
                        Print("#"),
                        ResetColor
                    )?;
                    rendered = true;
                }

                // Draw ground
                if !rendered {
                    execute!(stdout(), Print(" "))?;
                }
            }

            execute!(
                stdout(),
                SetForegroundColor(Color::DarkGrey),
                Print("║"),
                ResetColor,
                Print("\r\n")
            )?;
        }

        // Draw bottom border of game area
        execute!(stdout(), SetForegroundColor(Color::DarkGrey), Print("╠"),)?;
        for _ in 0..self.width - 2 {
            execute!(stdout(), Print("═"))?;
        }
        execute!(stdout(), Print("╣"), ResetColor, Print("\r\n"))?;

        // Draw stats
        self.render_stats(game)?;

        // Draw bottom border
        execute!(stdout(), SetForegroundColor(Color::Red), Print("╚"),)?;
        for _ in 0..self.width - 2 {
            execute!(stdout(), Print("═"))?;
        }
        execute!(stdout(), Print("╝"), ResetColor, Print("\r\n"))?;

        stdout().flush()
    }

    fn render_stats(&self, game: &GameState) -> Result<()> {
        // Player stats
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print(" HP: "),
            SetForegroundColor(Color::Green),
            Print(format!("{}/{}", game.player.health, game.player.max_health)),
            ResetColor,
            Print(" | Scavenged: "),
            SetForegroundColor(Color::Cyan),
            Print(format!("{}/3", game.player.scavenged_items)),
            ResetColor,
            Print(" | Turn: "),
            Print(format!("{}", game.turn_count)),
        )?;

        // Pad the rest of the line
        let stats_text_len = format!(
            " HP: {}/{} | Scavenged: {}/3 | Turn: {}",
            game.player.health,
            game.player.max_health,
            game.player.scavenged_items,
            game.turn_count
        )
        .len();
        for _ in stats_text_len..(self.width as usize - 2) {
            execute!(stdout(), Print(" "))?;
        }

        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print("\r\n")
        )?;

        // Combat mode indicator
        if let GameMode::Combat(idx) = game.mode
            && idx < game.enemies.len()
        {
            let enemy = &game.enemies[idx];
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkGrey),
                Print("║"),
                ResetColor,
                Print(" COMBAT! "),
                SetForegroundColor(Color::Red),
                Print(format!("{}", enemy.enemy_type)),
                ResetColor,
                Print(format!(" HP: {}/{}", enemy.health, enemy.max_health)),
            )?;

            let combat_text_len = format!(
                " COMBAT! {} HP: {}/{}",
                enemy.enemy_type, enemy.health, enemy.max_health
            )
            .len();
            for _ in combat_text_len..(self.width as usize - 2) {
                execute!(stdout(), Print(" "))?;
            }

            execute!(
                stdout(),
                SetForegroundColor(Color::DarkGrey),
                Print("║"),
                ResetColor,
                Print("\r\n")
            )?;
        }
        
        // Inventory display
        self.render_inventory(game)?;

        // Messages
        for msg in &game.messages {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkGrey),
                Print("║"),
                ResetColor,
                Print(" "),
                Print(msg),
            )?;

            let msg_len = msg.len() + 1;
            for _ in msg_len..(self.width as usize - 2) {
                execute!(stdout(), Print(" "))?;
            }

            execute!(
                stdout(),
                SetForegroundColor(Color::DarkGrey),
                Print("║"),
                ResetColor,
                Print("\r\n")
            )?;
        }

        // Legend
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print(" @ = You | r = Rat | c = Cat | P = Punk | "),
            SetForegroundColor(Color::Cyan),
            Print("▓"),
            ResetColor,
            Print(" = Dump | ⛓ = Exit | E = Use Item"),
        )?;

        let legend_len = " @ = You | r = Rat | c = Cat | P = Punk | ▓ = Dump | ⛓ = Exit | E = Use Item".len();
        for _ in legend_len..(self.width as usize - 2) {
            execute!(stdout(), Print(" "))?;
        }

        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print("\r\n")
        )?;

        Ok(())
    }
    
    fn render_inventory(&self, game: &GameState) -> Result<()> {
        let inv = &game.player.inventory;
        
        // Weapon slot
        let weapon_text = if let Some(weapon) = &inv.weapon {
            format!("Weapon: {} (+{})", weapon.name, weapon.damage_bonus)
        } else {
            "Weapon: [Empty]".to_string()
        };
        
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print(" "),
            Print(&weapon_text),
        )?;
        
        let text_len = weapon_text.len() + 1;
        for _ in text_len..(self.width as usize - 2) {
            execute!(stdout(), Print(" "))?;
        }
        
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print("\r\n")
        )?;
        
        // Armor slot
        let armor_text = if let Some(armor) = &inv.armor {
            format!("Armor: {} (+{})", armor.name, armor.defense_bonus)
        } else {
            "Armor: [Empty]".to_string()
        };
        
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print(" "),
            Print(&armor_text),
        )?;
        
        let text_len = armor_text.len() + 1;
        for _ in text_len..(self.width as usize - 2) {
            execute!(stdout(), Print(" "))?;
        }
        
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print("\r\n")
        )?;
        
        // Consumable slot
        let consumable_text = if let Some(consumable) = &inv.consumable {
            format!("Consumable: {} (E to use)", consumable.name)
        } else {
            "Consumable: [Empty]".to_string()
        };
        
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print(" "),
            Print(&consumable_text),
        )?;
        
        let text_len = consumable_text.len() + 1;
        for _ in text_len..(self.width as usize - 2) {
            execute!(stdout(), Print(" "))?;
        }
        
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print("\r\n")
        )?;
        
        // Bolt cutters status
        let cutters_text = if inv.bolt_cutters.found {
            "Bolt Cutters: ✓ Found!".to_string()
        } else {
            "Bolt Cutters: ✗ Not found".to_string()
        };
        
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print(" "),
            Print(&cutters_text),
        )?;
        
        let text_len = cutters_text.len() + 1;
        for _ in text_len..(self.width as usize - 2) {
            execute!(stdout(), Print(" "))?;
        }
        
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("║"),
            ResetColor,
            Print("\r\n")
        )?;
        
        Ok(())
    }

    fn render_victory(&self, game: &GameState) -> Result<()> {
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        let stats_line = format!(
            "║          Final Stats: {} HP | {} turns             ║",
            game.player.health, game.turn_count
        );

        let victory_art = vec![
            "╔════════════════════════════════════════════════════════════╗",
            "║                                                            ║",
            "║  ██╗   ██╗██╗ ██████╗████████╗ ██████╗ ██████╗ ██╗   ██╗ ║",
            "║  ██║   ██║██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗╚██╗ ██╔╝ ║",
            "║  ██║   ██║██║██║        ██║   ██║   ██║██████╔╝ ╚████╔╝  ║",
            "║  ╚██╗ ██╔╝██║██║        ██║   ██║   ██║██╔══██╗  ╚██╔╝   ║",
            "║   ╚████╔╝ ██║╚██████╗   ██║   ╚██████╔╝██║  ██║   ██║    ║",
            "║    ╚═══╝  ╚═╝ ╚═════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝   ╚═╝    ║",
            "║                                                            ║",
            "║          You've secured the dumpsters!                    ║",
            "║          The scavenge rights are yours!                   ║",
            "║                                                            ║",
        ];

        for line in &victory_art {
            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(line),
                ResetColor,
                Print("\r\n")
            )?;
        }

        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print(&stats_line),
            ResetColor,
            Print("\r\n")
        )?;

        let end_art = vec![
            "║                                                            ║",
            "║          Press Q to quit                                   ║",
            "║                                                            ║",
            "╚════════════════════════════════════════════════════════════╝",
        ];

        for line in &end_art {
            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(line),
                ResetColor,
                Print("\r\n")
            )?;
        }

        stdout().flush()
    }

    fn render_game_over(&self, game: &GameState) -> Result<()> {
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        let stats_line = format!(
            "║          Survived {} turns | Scavenged: {}/3          ║",
            game.turn_count, game.player.scavenged_items
        );

        let game_over_art = vec![
            "╔════════════════════════════════════════════════════════════╗",
            "║                                                            ║",
            "║   ██████╗  █████╗ ███╗   ███╗███████╗                     ║",
            "║  ██╔════╝ ██╔══██╗████╗ ████║██╔════╝                     ║",
            "║  ██║  ███╗███████║██╔████╔██║█████╗                       ║",
            "║  ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝                       ║",
            "║  ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗                     ║",
            "║   ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝                     ║",
            "║   ██████╗ ██╗   ██╗███████╗██████╗                        ║",
            "║  ██╔═══██╗██║   ██║██╔════╝██╔══██╗                       ║",
            "║  ██║   ██║██║   ██║█████╗  ██████╔╝                       ║",
            "║  ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗                       ║",
            "║  ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║                       ║",
            "║   ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝                       ║",
            "║                                                            ║",
            "║          You've been defeated!                            ║",
            "║          The dumpsters are lost...                        ║",
            "║                                                            ║",
        ];

        for line in &game_over_art {
            execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(line),
                ResetColor,
                Print("\r\n")
            )?;
        }

        execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print(&stats_line),
            ResetColor,
            Print("\r\n")
        )?;

        let end_art = vec![
            "║                                                            ║",
            "║          Press Q to quit                                   ║",
            "║                                                            ║",
            "╚════════════════════════════════════════════════════════════╝",
        ];

        for line in &end_art {
            execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(line),
                ResetColor,
                Print("\r\n")
            )?;
        }

        stdout().flush()
    }
}

pub fn setup_terminal() -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
    Ok(())
}

pub fn cleanup_terminal() -> Result<()> {
    execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
