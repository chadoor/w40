use std::collections::HashMap;
use std::vec;

use crate::model::*;
use crate::weapon::Weapon;
use std::fs;

// Helper function to check the file path
fn check_file_path<'a>(file_path: &'a str, expected_suffix: &str) -> Option<&'a str> {
    if file_path.ends_with(format!("{}.json", expected_suffix).as_str()) {
        Some(file_path)
    } else {
        None
    }
}

fn read_json_model_file(file_path: &str) -> HashMap<String, Vec<Model>> {
    let _check_file_path = check_file_path(file_path, "models").expect("File path does not end with '_models.json'");

    let data = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

pub fn build_arsenal_from_json(file_path: &str) -> HashMap<String, Vec<Weapon>> {
    let _check_file_path = check_file_path(file_path, "weapons").expect("File path does not end with '_weapons.json'");

    let data = fs::read_to_string(file_path).expect("Failed to read file");
    serde_json::from_str(&data).expect("Failed to parse json")
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
