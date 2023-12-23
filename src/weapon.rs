use rand::Rng;

use serde::{Deserialize, Serialize};

use crate::model::Model;

pub fn w_table(strenght: u8, tougness: u8) -> u8 {
    match (strenght, tougness) {
        (s, t) if s >= t * 2 => 2,
        (s, t) if s > t => 3,
        (s, t) if s == t => 4,
        (s, t) if s <= t => 5,
        (s, t) if s * 2 <= t => 6,
        _ => 0,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Ability {
    ANTI(String, u8),
    ASSAULT,
    HEAVY,
    PISTOL,
    NONE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weapon {
    name: String,
    n_attacks: i32,
    b_skill: i32,
    strength: i32,
    ap: i32,
    damage: i32,
    damage_die: i32,
    abilities: Vec<Ability>,
}

impl Weapon {
    pub fn new(
        name: String,
        n_attacks: i32,
        b_skill: i32,
        strength: i32,
        ap: i32,
        damage: i32,
        damage_die: i32,
        abilities: Vec<Ability>,
    ) -> Weapon {
        Weapon {
            name,
            n_attacks,
            b_skill,
            strength,
            ap,
            damage,
            damage_die,
            abilities,
        }
    }

    pub fn damage(&self) -> i32 {
        let dmg_die: i32 = match self.damage_die {
            3 => rand::thread_rng().gen_range(1..4),
            6 => rand::thread_rng().gen_range(1..7),
            _ => 0,
        };

        self.damage + dmg_die
    }

    pub fn deal_damage(&self, damage: i32, model: &mut Model) {
        model.take_damage(damage);
    }

    pub fn description(&self) -> String {
        let ability_descriptions: Vec<String> = self
            .abilities
            .iter()
            .map(|ability| match ability {
                Ability::ANTI(ref target, value) => format!("ANTI({}, {})", target, value),
                Ability::ASSAULT => "ASSAULT".to_string(),
                Ability::HEAVY => "HEAVY".to_string(),
                Ability::PISTOL => "PISTOL".to_string(),
                Ability::NONE => "NONE".to_string(),
                _ => "".to_string(),
            })
            .collect();

        let abilities_str: String = ability_descriptions.join(", ");

        if self.damage_die > 0 {
            format!(
                "{}[{}], A: {}, BS: {}+, S: {}, AP: {}, D:{}+{}",
                self.name,
                abilities_str,
                self.n_attacks,
                self.b_skill,
                self.strength,
                self.ap,
                self.damage,
                self.damage_die
            )
        } else {
            format!(
                "{}[{}], A: {}, BS: {}+, S: {}, AP: {}, Damage: {}",
                self.name, abilities_str, self.n_attacks, self.b_skill, self.strength, self.ap, self.damage
            )
        }
    }

    pub fn hit(&self) -> (bool, i32) {
        let mut num = rand::thread_rng().gen_range(1..7);

        // Iterate over each ability and apply effects
        for ability in &self.abilities {
            match ability {
                Ability::HEAVY => {
                    num += 1; // Modify num for HEAVY ability
                }
                // Add more abilities here with their respective logic
                _ => {}
            }
        }

        // Check for automatic fail or success
        if num == 1 {
            return (false, num);
        } else if num == 6 {
            return (true, num);
        }

        // Standard check against b_skill
        (num >= self.b_skill, num)
    }

    pub fn wound(&self, d_tougness: u8, keyswords: &Vec<String>) -> (bool, u8, u8) {
        let w_range = w_table(self.strength as u8, d_tougness);

        let num = rand::thread_rng().gen_range(1..7);
        let mut w_val = w_range;

        for ability in &self.abilities {
            match ability {
                Ability::ANTI(s, range) if keyswords.contains(s) => {
                    w_val = *range;
                }
                _ => {}
            }
        }

        if num == 1 {
            return (false, num, w_val);
        } else if num == 6 {
            return (true, num, w_val);
        }

        (num >= w_val, num, w_val)
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_n_attacks(&self) -> i32 {
        self.n_attacks
    }

    pub fn get_b_skill(&self) -> i32 {
        self.b_skill
    }

    pub fn get_strength(&self) -> i32 {
        self.strength
    }

    pub fn get_ap(&self) -> i32 {
        self.ap
    }
}
