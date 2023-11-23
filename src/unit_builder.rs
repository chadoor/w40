use std::collections::HashMap;
use std::vec;

use crate::model::*;
use crate::weapon::{Ability, Weapon};
use serde_json::{from_str, to_string};
use std::fs;

pub fn build_arsenal_from_json() {
    let file = "weapons.json";
    let json_string = fs::read_to_string(file).expect("Failed to read file");

    let weapons: Vec<Weapon> = serde_json::from_str(&json_string).expect("Failed to parse json");

    println!("{:#?}", weapons);
}

pub fn drukhari_kabalite_warriors_fjson() -> Vec<Model> {
    let file = "models.json";
    let json_string: String = fs::read_to_string(file).expect("Failed to read file");

    let data: HashMap<String, Vec<Model>> = serde_json::from_str(&json_string).unwrap();

    let mut unit: Vec<Model> = vec![];

    if let Some(models) = data.get("Kaballite_Warriors") {
        println!("In if");
        for model in models {
            let count = if model.get_name() == "Sybarite" { 1 } else { 9 };
            for _ in 0..count {
                unit.push(model.clone());
            }
        }
    }

    unit
}

pub fn arsenal_kabalite_warrior() -> Vec<Weapon> {
    let mut weapons = vec![
        Weapon::new("Blast Pistol".to_string(), 1, 3, 8, -3, 0, 3, vec![Ability::PISTOL]),
        Weapon::new("Blaster".to_string(), 1, 3, 8, -4, 1, 6, vec![Ability::ASSAULT]),
        Weapon::new("Dark Lance".to_string(), 1, 4, 12, -3, 2, 6, vec![Ability::HEAVY]),
        Weapon::new(
            "Splinter pistol".to_string(),
            1,
            3,
            2,
            0,
            1,
            0,
            vec![
                Ability::ANTI("Infantry".to_string(), 3),
                Ability::ASSAULT,
                Ability::PISTOL,
            ],
        ),
    ];

    weapons
}

pub fn drukhari_kabalite_warriors() -> Vec<Model> {
    let mut models: Vec<Model> = vec![create_kabalite_warrior("Sybarite".to_string())];

    for _ in 0..9 {
        models.push(create_kabalite_warrior("Kaballite Warior".to_string()));
    }

    models
}

fn create_kabalite_warrior(name: String) -> Model {
    let mut model = Model::new(name, 3, 4, 1);
    if model.get_name() == "Kaballite Warior" || model.get_name() == "Sybarite" {
        model.set_i_save(6);

        let keywords: Vec<String> = vec![
            "Infantry".to_string(),
            "Kabal".to_string(),
            "Battleline".to_string(),
            "Aeldari".to_string(),
            "Kabalite Warriors".to_string(),
        ];
        model.add_keyword_v(keywords);

        let mut weapons: Vec<Weapon> = vec![Weapon::new(
            "Splinter rifle".to_string(),
            2,
            3,
            2,
            0,
            1,
            0,
            vec![Ability::ANTI("Infantry".to_string(), 3), Ability::ASSAULT],
        )];
        model.add_weapon_v(&mut weapons);
    }
    model
}

/* Space Marines */

fn crate_intercessor(name: String, weapon: Weapon) -> Model {
    let mut model: Model = Model::new(name, 4, 3, 2);
    if model.get_name() == "Intercessor" || model.get_name() == "Intercessor Sergeant" {
        let keywords: Vec<String> = vec![
            "Infantry".to_string(),
            "Battleline".to_string(),
            "Grenades".to_string(),
            "Imperium".to_string(),
            "Tacticus".to_string(),
            "Intercessor Squad".to_string(),
        ];
        model.add_keyword_v(keywords);

        let mut weapons: Vec<Weapon> = vec![
            Weapon::new("Bolt Pistol".to_string(), 1, 3, 4, 0, 1, 0, vec![Ability::PISTOL]),
            Weapon::new(
                "Bolt Rifle".to_string(),
                2,
                3,
                4,
                -1,
                1,
                0,
                vec![Ability::ASSAULT, Ability::HEAVY],
            ),
        ];
        model.add_weapon_v(&mut weapons);
    }
    model
}

pub fn sm_intercessor_squad(num: u8) -> Vec<Model> {
    let mut models: Vec<Model> = vec![crate_intercessor(
        "Intercessor Sergeant".to_string(),
        Weapon::new(
            "Bolt Rifle".to_string(),
            2,
            3,
            4,
            -1,
            1,
            0,
            vec![Ability::ASSAULT, Ability::HEAVY],
        ),
    )];

    for _ in 0..num {
        models.push(crate_intercessor(
            "Intercessor".to_string(),
            Weapon::new(
                "Bolt Rifle".to_string(),
                2,
                3,
                4,
                -1,
                1,
                0,
                vec![Ability::ASSAULT, Ability::HEAVY],
            ),
        ));
    }

    models
}
