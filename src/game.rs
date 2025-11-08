use crate::items::{Armor, Consumable, Inventory, Weapon};
use crossterm::event::{KeyCode, KeyEvent};
use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub position: Position,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub scavenged_items: u32,
    pub inventory: Inventory,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            position: Position::new(x, y),
            health: 100,
            max_health: 100,
            attack: 10,
            scavenged_items: 0,
            inventory: Inventory::new(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        let defense = self.inventory.total_defense_bonus();
        let actual_damage = (damage - defense).max(1); // Always take at least 1 damage
        self.health = (self.health - actual_damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn total_attack(&self) -> i32 {
        self.attack + self.inventory.total_damage_bonus()
    }
}

#[derive(Debug, Clone)]
pub enum EnemyType {
    Rat,
    FerralCat,
    RivalPunk,
}

impl fmt::Display for EnemyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnemyType::Rat => write!(f, "Rat"),
            EnemyType::FerralCat => write!(f, "Feral Cat"),
            EnemyType::RivalPunk => write!(f, "Rival Punk"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub position: Position,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub enemy_type: EnemyType,
    pub is_alive: bool,
}

impl Enemy {
    pub fn new_rat(x: i32, y: i32) -> Self {
        Enemy {
            position: Position::new(x, y),
            health: 20,
            max_health: 20,
            attack: 3,
            enemy_type: EnemyType::Rat,
            is_alive: true,
        }
    }

    pub fn new_feral_cat(x: i32, y: i32) -> Self {
        Enemy {
            position: Position::new(x, y),
            health: 35,
            max_health: 35,
            attack: 7,
            enemy_type: EnemyType::FerralCat,
            is_alive: true,
        }
    }

    pub fn new_rival_punk(x: i32, y: i32) -> Self {
        Enemy {
            position: Position::new(x, y),
            health: 50,
            max_health: 50,
            attack: 10,
            enemy_type: EnemyType::RivalPunk,
            is_alive: true,
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health <= 0 {
            self.health = 0;
            self.is_alive = false;
        }
    }

    pub fn get_char(&self) -> char {
        match self.enemy_type {
            EnemyType::Rat => 'r',
            EnemyType::FerralCat => 'c',
            EnemyType::RivalPunk => 'P',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(10, 15);
        assert_eq!(player.position.x, 10);
        assert_eq!(player.position.y, 15);
        assert_eq!(player.health, 100);
        assert_eq!(player.max_health, 100);
        assert!(player.is_alive());
    }

    #[test]
    fn test_player_damage() {
        let mut player = Player::new(0, 0);
        player.take_damage(30);
        assert_eq!(player.health, 70);
        assert!(player.is_alive());

        player.take_damage(80);
        assert_eq!(player.health, 0);
        assert!(!player.is_alive());
    }

    #[test]
    fn test_player_healing() {
        let mut player = Player::new(0, 0);
        player.take_damage(50);
        player.heal(20);
        assert_eq!(player.health, 70);

        // Cannot heal beyond max health
        player.heal(100);
        assert_eq!(player.health, 100);
    }

    #[test]
    fn test_enemy_creation() {
        let rat = Enemy::new_rat(5, 5);
        assert_eq!(rat.health, 20);
        assert_eq!(rat.attack, 3);
        assert!(rat.is_alive);
        assert_eq!(rat.get_char(), 'r');

        let cat = Enemy::new_feral_cat(5, 5);
        assert_eq!(cat.health, 35);
        assert_eq!(cat.attack, 7);
        assert_eq!(cat.get_char(), 'c');

        let punk = Enemy::new_rival_punk(5, 5);
        assert_eq!(punk.health, 50);
        assert_eq!(punk.attack, 10);
        assert_eq!(punk.get_char(), 'P');
    }

    #[test]
    fn test_enemy_damage() {
        let mut enemy = Enemy::new_rat(0, 0);
        enemy.take_damage(10);
        assert_eq!(enemy.health, 10);
        assert!(enemy.is_alive);

        enemy.take_damage(15);
        assert_eq!(enemy.health, 0);
        assert!(!enemy.is_alive);
    }

    #[test]
    fn test_position_distance() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(3, 4);
        assert_eq!(pos1.distance_to(&pos2), 5.0);
    }

    #[test]
    fn test_game_state_initialization() {
        let game = GameState::new(60, 20);
        assert_eq!(game.width, 60);
        assert_eq!(game.height, 20);
        assert!(game.player.is_alive());
        assert_eq!(game.player.scavenged_items, 0);
        assert_eq!(game.dumpsters.len(), 3);
        assert!(!game.enemies.is_empty());
        assert_eq!(game.mode, GameMode::Exploring);
    }

    #[test]
    fn test_dumpster_creation() {
        let dumpster = Dumpster::new(10, 10);
        assert_eq!(dumpster.position.x, 10);
        assert_eq!(dumpster.position.y, 10);
        assert!(dumpster.has_items);
    }
}

#[derive(Debug, Clone)]
pub struct Dumpster {
    pub position: Position,
    pub has_items: bool,
    pub item_weapon: Option<Weapon>,
    pub item_armor: Option<Armor>,
    pub item_consumable: Option<Consumable>,
    pub has_bolt_cutters: bool,
}

impl Dumpster {
    pub fn new(x: i32, y: i32) -> Self {
        let mut rng = rand::thread_rng();

        // Generate random items for this dumpster
        let has_weapon = rng.gen_bool(0.7);
        let has_armor = rng.gen_bool(0.7);
        let has_consumable = rng.gen_bool(0.8);

        Dumpster {
            position: Position::new(x, y),
            has_items: true,
            item_weapon: if has_weapon {
                Some(Weapon::random_generate())
            } else {
                None
            },
            item_armor: if has_armor {
                Some(Armor::random_generate())
            } else {
                None
            },
            item_consumable: if has_consumable {
                Some(Consumable::random_generate())
            } else {
                None
            },
            has_bolt_cutters: false,
        }
    }

    pub fn new_with_bolt_cutters(x: i32, y: i32) -> Self {
        let mut dumpster = Self::new(x, y);
        dumpster.has_bolt_cutters = true;
        dumpster
    }
}

#[derive(Debug, PartialEq)]
pub enum GameMode {
    Exploring,
    Combat(usize), // index of enemy in combat
    Victory,
    GameOver,
    LevelComplete, // New state when level is complete but chain not cut yet
}

pub struct GameState {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub dumpsters: Vec<Dumpster>,
    pub width: i32,
    pub height: i32,
    pub mode: GameMode,
    pub messages: Vec<String>,
    pub turn_count: u32,
    pub chain_position: Position, // Position of the locked chain
}

impl GameState {
    pub fn new(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();

        // Player starts near the bottom center
        let player = Player::new(width / 2, height - 3);

        // Create dumpsters in the top area (behind the burger place)
        let mut dumpsters = Vec::new();
        for i in 0..2 {
            dumpsters.push(Dumpster::new(width / 4 + i * (width / 3), 3));
        }
        // Last dumpster has bolt cutters
        dumpsters.push(Dumpster::new_with_bolt_cutters(width * 3 / 4, 3));

        // Create enemies scattered around
        let mut enemies = Vec::new();

        // Add some rats
        for _ in 0..3 {
            let x = rng.gen_range(5..width - 5);
            let y = rng.gen_range(5..height - 5);
            enemies.push(Enemy::new_rat(x, y));
        }

        // Add a feral cat
        let x = rng.gen_range(5..width - 5);
        let y = rng.gen_range(5..height / 2);
        enemies.push(Enemy::new_feral_cat(x, y));

        // Add a rival punk near the dumpsters
        enemies.push(Enemy::new_rival_punk(width / 2 + 5, 5));

        // Chain is at the exit (top center)
        let chain_position = Position::new(width / 2, 1);

        GameState {
            player,
            enemies,
            dumpsters,
            width,
            height,
            mode: GameMode::Exploring,
            messages: vec![
                "Welcome to RUST PUNK!".to_string(),
                "Fight rats and rivals for dumpster scavenge rights!".to_string(),
                "Use WASD to move, Space to attack, Q to quit".to_string(),
                "Find bolt cutters to unlock the exit!".to_string(),
            ],
            turn_count: 0,
            chain_position,
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
        // Keep only last 5 messages
        if self.messages.len() > 5 {
            self.messages.remove(0);
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        match self.mode {
            GameMode::Exploring => self.handle_exploring_input(key),
            GameMode::Combat(enemy_idx) => self.handle_combat_input(key, enemy_idx),
            GameMode::LevelComplete => self.handle_level_complete_input(key),
            _ => {}
        }
    }

    fn handle_exploring_input(&mut self, key: KeyEvent) {
        let old_pos = self.player.position;
        let mut new_pos = old_pos;

        match key.code {
            KeyCode::Char('w') | KeyCode::Up => new_pos.y -= 1,
            KeyCode::Char('s') | KeyCode::Down => new_pos.y += 1,
            KeyCode::Char('a') | KeyCode::Left => new_pos.x -= 1,
            KeyCode::Char('d') | KeyCode::Right => new_pos.x += 1,
            KeyCode::Char('e') => {
                // Use consumable
                self.use_consumable();
                self.turn_count += 1;
                self.player.inventory.update_turn();
                return;
            }
            _ => return,
        }

        // Boundary check
        if new_pos.x >= 1
            && new_pos.x < self.width - 1
            && new_pos.y >= 1
            && new_pos.y < self.height - 1
        {
            self.player.position = new_pos;
            self.turn_count += 1;

            // Update inventory turn effects
            self.player.inventory.update_turn();

            // Check for dumpster interaction
            self.check_dumpster_scavenge();

            // Check for chain interaction
            self.check_chain_interaction();

            // Check for enemy encounters
            self.check_enemy_encounters();

            // Move enemies towards player
            self.move_enemies();
        }
    }

    fn handle_combat_input(&mut self, key: KeyEvent, enemy_idx: usize) {
        match key.code {
            KeyCode::Char(' ') | KeyCode::Enter => {
                if enemy_idx < self.enemies.len() && self.enemies[enemy_idx].is_alive {
                    let damage = self.player.total_attack();

                    // Take damage first
                    let enemy = &mut self.enemies[enemy_idx];
                    enemy.take_damage(damage);
                    let enemy_type = enemy.enemy_type.clone();
                    let enemy_is_alive = enemy.is_alive;
                    let enemy_damage = enemy.attack;

                    // Add message about player attack
                    self.add_message(format!("You attack {} for {} damage!", enemy_type, damage));

                    if !enemy_is_alive {
                        self.add_message(format!("{} defeated!", enemy_type));
                        self.mode = GameMode::Exploring;
                    } else {
                        // Enemy counterattacks
                        let defense_bonus = self.player.inventory.total_defense_bonus();
                        self.player.take_damage(enemy_damage);

                        let actual_damage = (enemy_damage - defense_bonus).max(1);
                        self.add_message(format!(
                            "{} attacks you for {} damage! (Reduced by {})",
                            enemy_type, actual_damage, defense_bonus
                        ));

                        if !self.player.is_alive() {
                            self.mode = GameMode::GameOver;
                            self.add_message("You have been defeated!".to_string());
                        }
                    }
                }
            }
            KeyCode::Char('r') => {
                self.add_message("You retreat from combat!".to_string());
                // Move player away from enemy
                let enemy = &self.enemies[enemy_idx];
                let dx = self.player.position.x - enemy.position.x;
                let dy = self.player.position.y - enemy.position.y;

                let new_x = (self.player.position.x + dx.signum()).clamp(1, self.width - 2);
                let new_y = (self.player.position.y + dy.signum()).clamp(1, self.height - 2);

                self.player.position = Position::new(new_x, new_y);
                self.mode = GameMode::Exploring;
            }
            KeyCode::Char('e') => {
                // Use consumable during combat
                self.use_consumable();
                self.player.inventory.update_turn();
            }
            _ => {}
        }
    }

    fn try_pickup_weapon(
        dumpster_slot: &mut Option<Weapon>,
        inventory_slot: &mut Option<Weapon>,
        found_items: &mut Vec<String>,
    ) -> bool {
        if let Some(weapon) = dumpster_slot.take() {
            if inventory_slot.is_some() {
                found_items.push(format!("Found {} but weapon slot full!", weapon.name));
                *dumpster_slot = Some(weapon); // Put it back
                false
            } else {
                found_items.push(format!(
                    "Found {}! (+{} damage)",
                    weapon.name, weapon.damage_bonus
                ));
                *inventory_slot = Some(weapon);
                true
            }
        } else {
            false
        }
    }

    fn try_pickup_armor(
        dumpster_slot: &mut Option<Armor>,
        inventory_slot: &mut Option<Armor>,
        found_items: &mut Vec<String>,
    ) -> bool {
        if let Some(armor) = dumpster_slot.take() {
            if inventory_slot.is_some() {
                found_items.push(format!("Found {} but armor slot full!", armor.name));
                *dumpster_slot = Some(armor); // Put it back
                false
            } else {
                found_items.push(format!(
                    "Found {}! (+{} defense)",
                    armor.name, armor.defense_bonus
                ));
                *inventory_slot = Some(armor);
                true
            }
        } else {
            false
        }
    }

    fn try_pickup_consumable(
        dumpster_slot: &mut Option<Consumable>,
        inventory_slot: &mut Option<Consumable>,
        found_items: &mut Vec<String>,
    ) -> bool {
        if let Some(consumable) = dumpster_slot.take() {
            if inventory_slot.is_some() {
                found_items.push(format!(
                    "Found {} but consumable slot full!",
                    consumable.name
                ));
                *dumpster_slot = Some(consumable); // Put it back
                false
            } else {
                found_items.push(format!("Found {}! (Press E to use)", consumable.name));
                *inventory_slot = Some(consumable);
                true
            }
        } else {
            false
        }
    }

    fn check_dumpster_scavenge(&mut self) {
        let player_pos = self.player.position;
        let mut scavenged = false;
        let mut found_items = Vec::new();
        let mut items_picked_up = false;

        for dumpster in &mut self.dumpsters {
            if dumpster.position.x == player_pos.x
                && dumpster.position.y == player_pos.y
                && dumpster.has_items
            {
                scavenged = true;

                // Check for bolt cutters first
                if dumpster.has_bolt_cutters {
                    self.player.inventory.bolt_cutters.found = true;
                    found_items.push("Bolt Cutters! (You can now cut the chain!)".to_string());
                    dumpster.has_bolt_cutters = false;
                    items_picked_up = true;
                }

                // Pick up weapon if slot is empty
                if Self::try_pickup_weapon(
                    &mut dumpster.item_weapon,
                    &mut self.player.inventory.weapon,
                    &mut found_items,
                ) {
                    items_picked_up = true;
                }

                // Pick up armor if slot is empty
                if Self::try_pickup_armor(
                    &mut dumpster.item_armor,
                    &mut self.player.inventory.armor,
                    &mut found_items,
                ) {
                    items_picked_up = true;
                }

                // Pick up consumable if slot is empty
                if Self::try_pickup_consumable(
                    &mut dumpster.item_consumable,
                    &mut self.player.inventory.consumable,
                    &mut found_items,
                ) {
                    items_picked_up = true;
                }

                // Only mark as scavenged if at least one item was picked up or no items remain
                let has_remaining_items = dumpster.item_weapon.is_some()
                    || dumpster.item_armor.is_some()
                    || dumpster.item_consumable.is_some()
                    || dumpster.has_bolt_cutters;

                if items_picked_up || !has_remaining_items {
                    dumpster.has_items = false;
                }

                break;
            }
        }

        if scavenged && items_picked_up {
            self.player.scavenged_items += 1;
            self.add_message(format!(
                "Scavenged dumpster! ({}/3)",
                self.player.scavenged_items
            ));

            // Heal player a bit
            self.player.heal(20);
            self.add_message("Found some food! Health restored.".to_string());

            // Add all found items messages
            for item_msg in found_items {
                self.add_message(item_msg);
            }

            // Check if all dumpsters are scavenged (ready for exit)
            if self.player.scavenged_items >= 3 {
                self.mode = GameMode::LevelComplete;
                self.add_message("All dumpsters scavenged! Find the exit.".to_string());
            }
        } else if scavenged && !items_picked_up {
            // Player found a dumpster but inventory was full
            self.add_message("Your inventory is full! Drop items to pick up more.".to_string());
            for item_msg in found_items {
                self.add_message(item_msg);
            }
        }
    }

    fn check_enemy_encounters(&mut self) {
        for (idx, enemy) in self.enemies.iter().enumerate() {
            if enemy.is_alive
                && enemy.position.x == self.player.position.x
                && enemy.position.y == self.player.position.y
            {
                self.mode = GameMode::Combat(idx);
                self.add_message(format!(
                    "Encountered {}! Space to attack, R to retreat.",
                    enemy.enemy_type
                ));
                break;
            }
        }
    }

    fn move_enemies(&mut self) {
        let player_pos = self.player.position;

        for enemy in &mut self.enemies {
            if !enemy.is_alive {
                continue;
            }

            // Simple AI: move towards player if within range
            let distance = enemy.position.distance_to(&player_pos);
            if distance < 10.0 && distance > 1.0 {
                let dx = (player_pos.x - enemy.position.x).signum();
                let dy = (player_pos.y - enemy.position.y).signum();

                // 50% chance to move each turn
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.5) {
                    enemy.position.x = (enemy.position.x + dx).clamp(1, self.width - 2);
                    enemy.position.y = (enemy.position.y + dy).clamp(1, self.height - 2);
                }
            }
        }
    }

    fn check_chain_interaction(&mut self) {
        if self.player.position.x == self.chain_position.x
            && self.player.position.y == self.chain_position.y
            && self.mode == GameMode::LevelComplete
        {
            if self.player.inventory.bolt_cutters.found {
                self.mode = GameMode::Victory;
                self.add_message("You cut the chain and escape! Victory!".to_string());
            } else {
                self.add_message("The exit is locked with a chain. Need bolt cutters!".to_string());
            }
        }
    }

    fn handle_level_complete_input(&mut self, key: KeyEvent) {
        // In level complete mode, player can still move around
        let old_pos = self.player.position;
        let mut new_pos = old_pos;

        match key.code {
            KeyCode::Char('w') | KeyCode::Up => new_pos.y -= 1,
            KeyCode::Char('s') | KeyCode::Down => new_pos.y += 1,
            KeyCode::Char('a') | KeyCode::Left => new_pos.x -= 1,
            KeyCode::Char('d') | KeyCode::Right => new_pos.x += 1,
            KeyCode::Char('e') => {
                self.use_consumable();
                return;
            }
            _ => return,
        }

        // Boundary check
        if new_pos.x >= 1
            && new_pos.x < self.width - 1
            && new_pos.y >= 1
            && new_pos.y < self.height - 1
        {
            self.player.position = new_pos;
            self.check_chain_interaction();
        }
    }

    fn use_consumable(&mut self) {
        use crate::items::ConsumableEffect;

        if let Some((effect, name)) = self.player.inventory.use_consumable() {
            match effect {
                ConsumableEffect::Heal(amount) => {
                    self.player.heal(amount);
                    self.add_message(format!("Used {}! Healed {} HP", name, amount));
                }
                ConsumableEffect::DamageBoost(amount, duration) => {
                    self.add_message(format!(
                        "Used {}! +{} damage for {} turns",
                        name, amount, duration
                    ));
                }
                ConsumableEffect::DefenseBoost(amount, duration) => {
                    self.add_message(format!(
                        "Used {}! +{} defense for {} turns",
                        name, amount, duration
                    ));
                }
            }
        } else {
            self.add_message("No consumable to use!".to_string());
        }
    }
}
