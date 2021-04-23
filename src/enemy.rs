use core::fmt;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Enemy {
    health: u8,
    attack: u8,
    defense: u8,
}

impl Enemy {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            health: rng.gen_range(5..=15),
            attack: rng.gen_range(1..=5),
            defense: rng.gen_range(0..=5),
        }
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn attack(&self) -> u8 {
        self.attack
    }

    pub fn defense(&self) -> u8 {
        self.defense
    }

    pub fn hit_by(&mut self, attack: u8) {
        if self.defense >= attack {
            return;
        }

        let damage = attack - self.defense;
        if damage > self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "H: ({}), A: ({}), D: ({})",
            self.health(),
            self.attack(),
            self.defense()
        )
    }
}
