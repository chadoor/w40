use std::collections::HashMap;

use rand::Rng;

pub enum Ability {
    ANTI(String, u8),
    ASSAULT,
    HEAVY,
    PISTOL,
    NONE,
}

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
        let mut num: i32 = 0;
        for ability in &self.abilities {
            let result: bool = match ability {
                Ability::HEAVY => {
                    num = rand::thread_rng().gen_range(1..7) + 1;
                    num >= self.b_skill
                }
                _ => {
                    num = rand::thread_rng().gen_range(1..7);
                    num >= self.b_skill
                }
            };

            if result {
                return (true, num);
            }
        }
        (false, num)
    }

    pub fn wound(&self, w_range: u8, keyswords: &Vec<String>) -> (bool, u8) {
        let mut num: u8 = 0;
        // let ability_hmap = HashMap::new();
        let infantry: String = "Infantry".to_string();

        for ability in &self.abilities {
            let result: bool = match ability {
                Ability::ANTI(s, range) if keyswords.contains(s) => {
                    num = rand::thread_rng().gen_range(1..7);
                    num >= *range
                }
                _ => {
                    num = rand::thread_rng().gen_range(1..7);
                    num >= w_range
                }
            };

            if result {
                return (true, num);
            }
        }
        (false, num)
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
