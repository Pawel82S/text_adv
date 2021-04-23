use crate::{item::Item, vector::Vec2};

pub struct Player {
    name: String,
    position: Vec2,
    level: u8,
    current_xp: u8,
    max_health: u8,
    current_health: u8,
    max_mana: u8,
    current_mana: u8,
    strenght: u8,
    dexterity: u8,
    intelligence: u8,
    speed: u8,
    remaining_moves: u8,
    head: Option<Item>,
    chest: Option<Item>,
    legs: Option<Item>,
    arms: Option<Item>,
    foots: Option<Item>,
    left_hand: Option<Item>,
    right_hand: Option<Item>,
    inventory: Vec<Item>,
}

impl Player {
    pub const SYMBOL: char = '☻';

    pub fn new(
        name: String,
        max_health: u8,
        max_mana: u8,
        strenght: u8,
        dexterity: u8,
        intelligence: u8,
        speed: u8,
    ) -> Self {
        Self {
            name,
            position: Vec2::ZERO,
            level: 0,
            current_xp: 0,
            max_health,
            current_health: max_health,
            max_mana,
            current_mana: max_mana,
            strenght,
            dexterity,
            intelligence,
            speed,
            remaining_moves: speed,
            head: None,
            chest: None,
            legs: None,
            arms: None,
            foots: None,
            left_hand: None,
            right_hand: None,
            inventory: vec![],
        }
    }

    pub fn turn(&mut self) {
        self.remaining_moves = self.max_moves();
    }

    pub fn hit_by(&mut self, attack: u8) {
        let player_def = self.defense();
        if player_def >= attack {
            return;
        }

        let remaining_damage = attack - player_def;
        if remaining_damage < self.current_health {
            self.current_health -= remaining_damage;
        } else {
            self.current_health = 0;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current_health == 0
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn move_vec(&mut self, vec: Vec2) {
        self.position += vec;
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn current_xp(&self) -> u8 {
        self.current_xp
    }

    pub fn next_level_xp(&self) -> u8 {
        0
    }

    pub fn max_health(&self) -> u8 {
        let mut health = 0;
        let static_health = (self.max_health + self.level) as i8;

        if let Some(item) = &self.head {
            health += item.health_mod;
        }

        if let Some(item) = &self.chest {
            health += item.health_mod;
        }

        if let Some(item) = &self.legs {
            health += item.health_mod;
        }

        if let Some(item) = &self.arms {
            health += item.health_mod;
        }

        if let Some(item) = &self.foots {
            health += item.health_mod;
        }

        if let Some(item) = &self.left_hand {
            health += item.health_mod;
        }

        if let Some(item) = &self.right_hand {
            health += item.health_mod;
        }

        if static_health + health < 0 {
            0
        } else {
            (static_health + health) as u8
        }
    }

    pub fn current_health(&self) -> u8 {
        self.current_health
    }

    pub fn max_mana(&self) -> u8 {
        let mut mana = 0;
        let static_mana = (self.max_mana + self.level) as i8;

        if let Some(item) = &self.head {
            mana += item.mana_mod;
        }

        if let Some(item) = &self.chest {
            mana += item.mana_mod;
        }

        if let Some(item) = &self.legs {
            mana += item.mana_mod;
        }

        if let Some(item) = &self.arms {
            mana += item.mana_mod;
        }

        if let Some(item) = &self.foots {
            mana += item.mana_mod;
        }

        if let Some(item) = &self.left_hand {
            mana += item.mana_mod;
        }

        if let Some(item) = &self.right_hand {
            mana += item.mana_mod;
        }

        if static_mana + mana < 0 {
            0
        } else {
            (static_mana + mana) as u8
        }
    }

    pub fn current_mana(&self) -> u8 {
        self.current_mana
    }

    pub fn attack(&self) -> u8 {
        let mut attack = 0;
        let static_attack = (self.strenght + self.level) as i8;

        if let Some(item) = &self.head {
            attack += item.attack_mod;
        }

        if let Some(item) = &self.chest {
            attack += item.attack_mod;
        }

        if let Some(item) = &self.legs {
            attack += item.attack_mod;
        }

        if let Some(item) = &self.arms {
            attack += item.attack_mod;
        }

        if let Some(item) = &self.foots {
            attack += item.attack_mod;
        }

        if let Some(item) = &self.left_hand {
            attack += item.attack_mod;
        }

        if let Some(item) = &self.right_hand {
            attack += item.attack_mod;
        }

        if static_attack + attack < 0 {
            0
        } else {
            (static_attack + attack) as u8
        }
    }

    pub fn defense(&self) -> u8 {
        let mut defense = 0;
        let static_defense = (self.dexterity + self.level) as i8;

        if let Some(item) = &self.head {
            defense += item.defense_mod;
        }

        if let Some(item) = &self.chest {
            defense += item.defense_mod;
        }

        if let Some(item) = &self.legs {
            defense += item.defense_mod;
        }

        if let Some(item) = &self.arms {
            defense += item.defense_mod;
        }

        if let Some(item) = &self.foots {
            defense += item.defense_mod;
        }

        if let Some(item) = &self.left_hand {
            defense += item.defense_mod;
        }

        if let Some(item) = &self.right_hand {
            defense += item.defense_mod;
        }

        if static_defense + defense < 0 {
            0
        } else {
            (static_defense + defense) as u8
        }
    }

    pub fn strenght(&self) -> u8 {
        let mut strength = 0;
        let static_strength = (self.strenght + self.level) as i8;

        if let Some(item) = &self.head {
            strength += item.strength_mod;
        }

        if let Some(item) = &self.chest {
            strength += item.strength_mod;
        }

        if let Some(item) = &self.legs {
            strength += item.strength_mod;
        }

        if let Some(item) = &self.arms {
            strength += item.strength_mod;
        }

        if let Some(item) = &self.foots {
            strength += item.strength_mod;
        }

        if let Some(item) = &self.left_hand {
            strength += item.strength_mod;
        }

        if let Some(item) = &self.right_hand {
            strength += item.strength_mod;
        }

        if static_strength + strength < 0 {
            0
        } else {
            (static_strength + strength) as u8
        }
    }

    pub fn dexterity(&self) -> u8 {
        let mut dexterity = 0;
        let static_dexterity = (self.dexterity + self.level) as i8;

        if let Some(item) = &self.head {
            dexterity += item.dexterity_mod;
        }

        if let Some(item) = &self.chest {
            dexterity += item.dexterity_mod;
        }

        if let Some(item) = &self.legs {
            dexterity += item.dexterity_mod;
        }

        if let Some(item) = &self.arms {
            dexterity += item.dexterity_mod;
        }

        if let Some(item) = &self.foots {
            dexterity += item.dexterity_mod;
        }

        if let Some(item) = &self.left_hand {
            dexterity += item.dexterity_mod;
        }

        if let Some(item) = &self.right_hand {
            dexterity += item.dexterity_mod;
        }

        if static_dexterity + dexterity < 0 {
            0
        } else {
            (static_dexterity + dexterity) as u8
        }
    }

    pub fn intelligence(&self) -> u8 {
        let mut intelligence = 0;
        let static_intelligence = (self.intelligence + self.level) as i8;

        if let Some(item) = &self.head {
            intelligence += item.intelligence_mod;
        }

        if let Some(item) = &self.chest {
            intelligence += item.intelligence_mod;
        }

        if let Some(item) = &self.legs {
            intelligence += item.intelligence_mod;
        }

        if let Some(item) = &self.arms {
            intelligence += item.intelligence_mod;
        }

        if let Some(item) = &self.foots {
            intelligence += item.intelligence_mod;
        }

        if let Some(item) = &self.left_hand {
            intelligence += item.intelligence_mod;
        }

        if let Some(item) = &self.right_hand {
            intelligence += item.intelligence_mod;
        }

        if static_intelligence + intelligence < 0 {
            0
        } else {
            (static_intelligence + intelligence) as u8
        }
    }

    pub fn speed(&self) -> u8 {
        self.speed
    }

    pub fn max_moves(&self) -> u8 {
        let mut speed = 0;
        let static_speed = (self.speed + self.level) as i8;

        if let Some(item) = &self.head {
            speed += item.speed_mod;
        }

        if let Some(item) = &self.chest {
            speed += item.speed_mod;
        }

        if let Some(item) = &self.legs {
            speed += item.speed_mod;
        }

        if let Some(item) = &self.arms {
            speed += item.speed_mod;
        }

        if let Some(item) = &self.foots {
            speed += item.speed_mod;
        }

        if let Some(item) = &self.left_hand {
            speed += item.speed_mod;
        }

        if let Some(item) = &self.right_hand {
            speed += item.speed_mod;
        }

        if static_speed + speed < 0 {
            0
        } else {
            (static_speed + speed) as u8
        }
    }

    pub fn remaining_moves(&self) -> u8 {
        self.remaining_moves
    }

    pub fn head(&self) -> &Option<Item> {
        &self.head
    }

    pub fn chest(&self) -> &Option<Item> {
        &self.chest
    }

    pub fn legs(&self) -> &Option<Item> {
        &self.legs
    }

    pub fn arms(&self) -> &Option<Item> {
        &self.arms
    }

    pub fn foots(&self) -> &Option<Item> {
        &self.foots
    }
    pub fn left_hand(&self) -> &Option<Item> {
        &self.left_hand
    }
    pub fn right_hand(&self) -> &Option<Item> {
        &self.right_hand
    }

    pub fn inventory(&self) -> &Vec<Item> {
        &self.inventory
    }

    pub fn add_to_inventory(&mut self, item: Item) {
        self.inventory.push(item);
    }

    pub fn remove_from_inventory(&mut self, index: usize) {
        if index < self.inventory.len() {
            self.inventory.remove(index);
        }
    }
}
