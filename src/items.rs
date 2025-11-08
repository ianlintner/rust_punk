use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
}

impl fmt::Display for ItemRarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemRarity::Common => write!(f, "Common"),
            ItemRarity::Uncommon => write!(f, "Uncommon"),
            ItemRarity::Rare => write!(f, "Rare"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub damage_bonus: i32,
    pub rarity: ItemRarity,
    pub description: String,
}

impl Weapon {
    pub fn random_generate() -> Self {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..100);
        
        let (rarity, multiplier) = if roll < 60 {
            (ItemRarity::Common, 1)
        } else if roll < 90 {
            (ItemRarity::Uncommon, 2)
        } else {
            (ItemRarity::Rare, 3)
        };
        
        let weapon_types = [
            ("Broken Bottle", 2, "Sharp glass from the dumpster"),
            ("Rusty Chain", 3, "Heavy and intimidating"),
            ("Spiked Bat", 4, "With nails driven through it"),
            ("Bike Lock", 3, "U-lock makes a good weapon"),
            ("Crowbar", 5, "Classic street tool"),
            ("Switchblade", 3, "Flicks open with a satisfying click"),
            ("Brass Knuckles", 2, "Adds weight to your punch"),
            ("Metal Pipe", 4, "Found behind the dumpster"),
        ];
        
        let idx = rng.gen_range(0..weapon_types.len());
        let (name, base_damage, desc) = weapon_types[idx];
        let damage_bonus = base_damage * multiplier;
        
        Weapon {
            name: name.to_string(),
            damage_bonus,
            rarity,
            description: desc.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Armor {
    pub name: String,
    pub defense_bonus: i32,
    pub rarity: ItemRarity,
    pub description: String,
}

impl Armor {
    pub fn random_generate() -> Self {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..100);
        
        let (rarity, multiplier) = if roll < 60 {
            (ItemRarity::Common, 1)
        } else if roll < 90 {
            (ItemRarity::Uncommon, 2)
        } else {
            (ItemRarity::Rare, 3)
        };
        
        let armor_types = [
            ("Leather Jacket", 2, "Worn but protective"),
            ("Studded Vest", 3, "Metal studs add defense"),
            ("Chain Mail Shirt", 4, "Surprisingly effective"),
            ("Motorcycle Helmet", 2, "Protects your head"),
            ("Kevlar Vest", 5, "Military surplus find"),
            ("Padded Hoodie", 2, "Extra layers help"),
            ("Steel-Toe Boots", 1, "Good for kicking and protection"),
            ("Riot Shield", 4, "Liberated from authorities"),
        ];
        
        let idx = rng.gen_range(0..armor_types.len());
        let (name, base_defense, desc) = armor_types[idx];
        let defense_bonus = base_defense * multiplier;
        
        Armor {
            name: name.to_string(),
            defense_bonus,
            rarity,
            description: desc.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConsumableEffect {
    Heal(i32),
    DamageBoost(i32, u32), // amount, duration in turns
    DefenseBoost(i32, u32), // amount, duration in turns
}

#[derive(Debug, Clone)]
pub struct Consumable {
    pub name: String,
    pub effect: ConsumableEffect,
    pub description: String,
}

impl Consumable {
    pub fn random_generate() -> Self {
        let mut rng = rand::thread_rng();
        let consumable_types = [
            ("Burger Leftovers", ConsumableEffect::Heal(15), "Still edible... probably"),
            ("Energy Drink", ConsumableEffect::DamageBoost(5, 3), "Gives you wings (and jitters)"),
            ("Painkillers", ConsumableEffect::DefenseBoost(3, 5), "Takes the edge off"),
            ("Mystery Meat", ConsumableEffect::Heal(25), "Don't ask what it is"),
            ("Expired Soda", ConsumableEffect::Heal(10), "Flat but refreshing"),
            ("Protein Bar", ConsumableEffect::Heal(20), "Found in someone's backpack"),
            ("Adrenaline Shot", ConsumableEffect::DamageBoost(8, 2), "Fight or flight activated"),
            ("Bandages", ConsumableEffect::Heal(30), "Clean-ish medical supplies"),
        ];
        
        let idx = rng.gen_range(0..consumable_types.len());
        let (name, effect, desc) = &consumable_types[idx];
        
        Consumable {
            name: name.to_string(),
            effect: effect.clone(),
            description: desc.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoltCutters {
    pub found: bool,
}

impl BoltCutters {
    pub fn new() -> Self {
        BoltCutters { found: false }
    }
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
    pub consumable: Option<Consumable>,
    pub bolt_cutters: BoltCutters,
    pub temp_damage_boost: i32,
    pub temp_defense_boost: i32,
    pub boost_turns_remaining: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            weapon: None,
            armor: None,
            consumable: None,
            bolt_cutters: BoltCutters::new(),
            temp_damage_boost: 0,
            temp_defense_boost: 0,
            boost_turns_remaining: 0,
        }
    }
    
    pub fn total_damage_bonus(&self) -> i32 {
        let weapon_bonus = self.weapon.as_ref().map_or(0, |w| w.damage_bonus);
        weapon_bonus + self.temp_damage_boost
    }
    
    pub fn total_defense_bonus(&self) -> i32 {
        let armor_bonus = self.armor.as_ref().map_or(0, |a| a.defense_bonus);
        armor_bonus + self.temp_defense_boost
    }
    
    pub fn update_turn(&mut self) {
        if self.boost_turns_remaining > 0 {
            self.boost_turns_remaining -= 1;
            if self.boost_turns_remaining == 0 {
                self.temp_damage_boost = 0;
                self.temp_defense_boost = 0;
            }
        }
    }
    
    pub fn use_consumable(&mut self) -> Option<(ConsumableEffect, String)> {
        if let Some(consumable) = self.consumable.take() {
            let effect = consumable.effect.clone();
            let name = consumable.name.clone();
            
            match &effect {
                ConsumableEffect::DamageBoost(amount, duration) => {
                    // Add to existing boost instead of overwriting
                    self.temp_damage_boost += *amount;
                    self.boost_turns_remaining = self.boost_turns_remaining.max(*duration);
                }
                ConsumableEffect::DefenseBoost(amount, duration) => {
                    // Add to existing boost instead of overwriting
                    self.temp_defense_boost += *amount;
                    self.boost_turns_remaining = self.boost_turns_remaining.max(*duration);
                }
                _ => {}
            }
            
            Some((effect, name))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weapon_generation() {
        let weapon = Weapon::random_generate();
        assert!(!weapon.name.is_empty());
        assert!(weapon.damage_bonus > 0);
        assert!(!weapon.description.is_empty());
    }

    #[test]
    fn test_armor_generation() {
        let armor = Armor::random_generate();
        assert!(!armor.name.is_empty());
        assert!(armor.defense_bonus > 0);
        assert!(!armor.description.is_empty());
    }

    #[test]
    fn test_consumable_generation() {
        let consumable = Consumable::random_generate();
        assert!(!consumable.name.is_empty());
        assert!(!consumable.description.is_empty());
    }

    #[test]
    fn test_inventory_creation() {
        let inventory = Inventory::new();
        assert!(inventory.weapon.is_none());
        assert!(inventory.armor.is_none());
        assert!(inventory.consumable.is_none());
        assert!(!inventory.bolt_cutters.found);
    }

    #[test]
    fn test_inventory_bonuses() {
        let mut inventory = Inventory::new();
        assert_eq!(inventory.total_damage_bonus(), 0);
        assert_eq!(inventory.total_defense_bonus(), 0);
        
        inventory.weapon = Some(Weapon {
            name: "Test Weapon".to_string(),
            damage_bonus: 5,
            rarity: ItemRarity::Common,
            description: "Test".to_string(),
        });
        
        inventory.armor = Some(Armor {
            name: "Test Armor".to_string(),
            defense_bonus: 3,
            rarity: ItemRarity::Common,
            description: "Test".to_string(),
        });
        
        assert_eq!(inventory.total_damage_bonus(), 5);
        assert_eq!(inventory.total_defense_bonus(), 3);
    }

    #[test]
    fn test_consumable_use() {
        let mut inventory = Inventory::new();
        inventory.consumable = Some(Consumable {
            name: "Test Consumable".to_string(),
            effect: ConsumableEffect::DamageBoost(5, 3),
            description: "Test".to_string(),
        });
        
        let result = inventory.use_consumable();
        assert!(result.is_some());
        assert!(inventory.consumable.is_none());
        assert_eq!(inventory.temp_damage_boost, 5);
        assert_eq!(inventory.boost_turns_remaining, 3);
    }

    #[test]
    fn test_boost_duration() {
        let mut inventory = Inventory::new();
        inventory.temp_damage_boost = 5;
        inventory.boost_turns_remaining = 2;
        
        inventory.update_turn();
        assert_eq!(inventory.boost_turns_remaining, 1);
        assert_eq!(inventory.temp_damage_boost, 5);
        
        inventory.update_turn();
        assert_eq!(inventory.boost_turns_remaining, 0);
        assert_eq!(inventory.temp_damage_boost, 0);
    }
}
