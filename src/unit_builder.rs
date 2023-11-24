use std::collections::HashMap;
use std::vec;

use crate::model::*;
use crate::weapon::Weapon;
use std::fs;

fn read_json_model_file(file_path: &str) -> HashMap<String, Vec<Model>> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

pub struct UnitBuilder {
    model_map: HashMap<String, Vec<Model>>,
}

impl UnitBuilder {
    pub fn new(file_path: &str) -> UnitBuilder {
        let model_map: HashMap<String, Vec<Model>> = read_json_model_file(file_path);

        UnitBuilder { model_map: model_map }
    }

    pub fn list_units(&self) {
        for (key, _) in &self.model_map {
            println!("{}", key);
        }
    }

    pub fn get_unit(&self, unit_name: String) -> Vec<Model> {
        let mut unit: Vec<Model> = vec![];

        if let Some(models) = self.model_map.get(&unit_name) {
            for model in models {
                let count = model.get_count();
                for _ in 0..count {
                    unit.push(model.clone());
                }
            }
        }

        unit
    }
}

pub fn build_arsenal_from_json() -> Vec<Weapon> {
    let file = "weapons.json";
    let json_string = fs::read_to_string(file).expect("Failed to read file");

    let weapons: Vec<Weapon> = serde_json::from_str(&json_string).expect("Failed to parse json");

    // println!("{:#?}", weapons);
    weapons
}
