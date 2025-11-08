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
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            position: Position::new(x, y),
            health: 100,
            max_health: 100,
            attack: 10,
            scavenged_items: 0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
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
        assert!(game.enemies.len() > 0);
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
}

impl Dumpster {
    pub fn new(x: i32, y: i32) -> Self {
        Dumpster {
            position: Position::new(x, y),
            has_items: true,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameMode {
    Exploring,
    Combat(usize), // index of enemy in combat
    Victory,
    GameOver,
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
}

impl GameState {
    pub fn new(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();

        // Player starts near the bottom center
        let player = Player::new(width / 2, height - 3);

        // Create dumpsters in the top area (behind the burger place)
        let mut dumpsters = Vec::new();
        for i in 0..3 {
            dumpsters.push(Dumpster::new(width / 4 + i * (width / 4), 3));
        }

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
                "Scavenge 3 dumpsters to win!".to_string(),
            ],
            turn_count: 0,
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

            // Check for dumpster interaction
            self.check_dumpster_scavenge();

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
                    let damage = self.player.attack;

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
                        self.player.take_damage(enemy_damage);
                        self.add_message(format!(
                            "{} attacks you for {} damage!",
                            enemy_type, enemy_damage
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
            _ => {}
        }
    }

    fn check_dumpster_scavenge(&mut self) {
        let player_pos = self.player.position;
        let mut scavenged = false;

        for dumpster in &mut self.dumpsters {
            if dumpster.position.x == player_pos.x
                && dumpster.position.y == player_pos.y
                && dumpster.has_items
            {
                dumpster.has_items = false;
                scavenged = true;
                break;
            }
        }

        if scavenged {
            self.player.scavenged_items += 1;
            self.add_message(format!(
                "Scavenged dumpster! ({}/3)",
                self.player.scavenged_items
            ));

            // Heal player a bit
            self.player.heal(20);
            self.add_message("Found some food! Health restored.".to_string());

            // Check for victory
            if self.player.scavenged_items >= 3 {
                self.mode = GameMode::Victory;
                self.add_message("Victory! You've secured the dumpsters!".to_string());
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
}
