use core::fmt;
use rand::Rng;

#[derive(Debug, Clone)]
pub enum Location {
    Head,
    Chest,
    Leg,
    Arm,
    Foot,
    Hand,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub location: Location,
    pub health_mod: i8,
    pub mana_mod: i8,
    pub attack_mod: i8,
    pub defense_mod: i8,
    pub strength_mod: i8,
    pub dexterity_mod: i8,
    pub intelligence_mod: i8,
    pub speed_mod: i8,
}

impl Item {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut parameters = rng.gen_range(1..=3);
        let location = Location::Head;

        let health_mod = Self::get_random_parmeter(&mut rng, &mut parameters);
        let mana_mod = Self::get_random_parmeter(&mut rng, &mut parameters);
        let attack_mod = Self::get_random_parmeter(&mut rng, &mut parameters);
        let defense_mod = Self::get_random_parmeter(&mut rng, &mut parameters);
        let strength_mod = Self::get_random_parmeter(&mut rng, &mut parameters);
        let dexterity_mod = if parameters == 3 {
            parameters -= 1;
            rng.gen_range(1..=3)
        } else {
            Self::get_random_parmeter(&mut rng, &mut parameters)
        };
        let intelligence_mod = if parameters == 2 {
            parameters -= 1;
            rng.gen_range(1..=3)
        } else {
            Self::get_random_parmeter(&mut rng, &mut parameters)
        };
        let speed_mod = if parameters == 1 {
            rng.gen_range(1..=3)
        } else {
            Self::get_random_parmeter(&mut rng, &mut parameters)
        };

        Self {
            location,
            health_mod,
            mana_mod,
            attack_mod,
            defense_mod,
            strength_mod,
            dexterity_mod,
            intelligence_mod,
            speed_mod,
        }
    }

    fn get_random_parmeter(rng: &mut impl Rng, parameters: &mut u8) -> i8 {
        if *parameters > 0 && rng.gen() {
            let value = rng.gen_range(-3..=3);
            if value > 0 {
                *parameters -= 1;
            }
            value
        } else {
            0
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.location {
            Location::Leg => "Pants",
            Location::Arm => "???",
            Location::Head => "Helmet",
            Location::Foot => "Boots",
            Location::Chest => "Armor",
            Location::Hand => "Sword",
        };

        let mut params = String::new();
        if self.health_mod != 0 {
            params.push_str(&format!(" h({}),", self.health_mod));
        }
        if self.mana_mod != 0 {
            params.push_str(&format!(" m({}),", self.mana_mod));
        }
        if self.attack_mod != 0 {
            params.push_str(&format!(" a({}),", self.attack_mod));
        }
        if self.defense_mod != 0 {
            params.push_str(&format!(" d({}),", self.defense_mod));
        }
        if self.strength_mod != 0 {
            params.push_str(&format!(" s({}),", self.strength_mod));
        }
        if self.dexterity_mod != 0 {
            params.push_str(&format!(" x({}),", self.dexterity_mod));
        }
        if self.intelligence_mod != 0 {
            params.push_str(&format!(" i({}),", self.intelligence_mod));
        }
        if self.speed_mod != 0 {
            params.push_str(&format!(" v({}),", self.speed_mod));
        }
        write!(f, "{}{}", name, params)
    }
}
