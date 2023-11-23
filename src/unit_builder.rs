use std::collections::HashMap;
use std::vec;

use crate::model::*;
use crate::weapon::{Ability, Weapon};
use serde_json::{from_str, to_string};
use std::fs;

pub fn build_arsenal_from_json() -> Vec<Weapon> {
    let file = "weapons.json";
    let json_string = fs::read_to_string(file).expect("Failed to read file");

    let weapons: Vec<Weapon> = serde_json::from_str(&json_string).expect("Failed to parse json");

    println!("{:#?}", weapons);
    weapons
}

pub fn drukhari_kabalite_warriors_json() -> Vec<Model> {
    let file = "models.json";
    let json_string: String = fs::read_to_string(file).expect("Failed to read file");

    let data: HashMap<String, Vec<Model>> = serde_json::from_str(&json_string).unwrap();

    let mut unit: Vec<Model> = vec![];

    if let Some(models) = data.get("Kaballite_Warriors") {
        for model in models {
            let count = if model.get_name() == "Sybarite" { 1 } else { 9 };
            for _ in 0..count {
                unit.push(model.clone());
            }
        }
    }

    unit
}

/* Space Marines */

pub fn intercessor_squad_json(num: u8) -> Vec<Model> {
    let file = "models.json";
    let json_string = fs::read_to_string(file).expect("Failed to read file");
    let data: HashMap<String, Vec<Model>> = serde_json::from_str(&json_string).unwrap();

    let mut unit: Vec<Model> = vec![];

    if let Some(models) = data.get("Intercessor_Squad") {
        for model in models {
            let count = if model.get_name() == "Intercessor Sergeant" {
                1
            } else {
                num
            };
            for _ in 0..count {
                unit.push(model.clone());
            }
        }
    }
    unit
}
