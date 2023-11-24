use crate::weapon::Weapon;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Model {
    name: String,
    toughness: i32,
    a_save: i32,
    i_save: i32,
    wounds: i32,
    weapons: Vec<Weapon>,
    keywords: Vec<String>,
    count: u8,
}

impl Model {
    pub fn new(name: String, toughness: i32, a_save: i32, wounds: i32, count: u8) -> Model {
        Model {
            name,
            toughness,
            a_save,
            wounds,
            weapons: vec![],
            i_save: 10,
            keywords: vec![],
            count,
        }
    }

    pub fn description(&self) -> String {
        format!(
            "{}, T: {}, SV: {}+, W: {}",
            self.name, self.toughness, self.a_save, self.wounds
        )
    }

    pub fn add_keyword(&mut self, keyword: String) {
        self.keywords.push(keyword);
    }

    pub fn add_keyword_v(&mut self, keywords: Vec<String>) {
        self.keywords = keywords;
    }

    pub fn add_keyword_vr(&mut self, keywords: &Vec<String>) {
        self.keywords.extend(keywords.iter().cloned());
    }

    pub fn add_weapon(&mut self, weapon: Weapon) {
        self.weapons.push(weapon);
    }

    pub fn add_weapon_v(&mut self, weapons: &mut Vec<Weapon>) {
        self.weapons.append(weapons);
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.wounds -= damage;
        if self.wounds <= 0 {
            println!("{} has died", self.name);
        }
    }

    pub fn set_i_save(&mut self, i_save: i32) {
        self.i_save = i_save;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_toughness(&self) -> i32 {
        self.toughness
    }

    pub fn get_a_save(&self) -> i32 {
        self.a_save
    }

    pub fn get_i_save(&self) -> i32 {
        self.i_save
    }

    pub fn get_wounds(&self) -> i32 {
        self.wounds
    }

    pub fn get_weapon(&self) -> &Vec<Weapon> {
        &self.weapons
    }

    pub fn get_weapon_names(&self) -> String {
        let weapon_descriptions: Vec<String> = self.weapons.iter().map(|m: &Weapon| m.get_name().to_string()).collect();

        let weapon_names: String = weapon_descriptions.join(", ");
        weapon_names
    }

    pub fn get_keywords(&self) -> &Vec<String> {
        &self.keywords
    }

    pub fn get_model_dummy(&self) {}

    pub fn get_count(&self) -> u8 {
        self.count
    }
}
