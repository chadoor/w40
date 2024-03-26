use crate::weapon::*;

struct Stats{
    movement: u8,
    toughness: u8,
    armor_save: u8,
    wounds: u8,
    leadership: u8,
    objective_control: u8,
    invulnerable_save: u8
}

struct meta_unit{
    unit_stats: Stats,
    ranged_weapons: Vec<String>,
    melee_weapons: Vec<String>,
    keywords: Vec<String>,
    faction_keywords: String,
    abilities: Vec<String>,
    faction_abilities: String,
    core_abilities: Vec<String>,
    unit_composition: (String,u8),
    weapon_composition: Vec<String>
}

pub struct n_unit{
    name: String,
    unit_stats: Stats,
    weapons: Vec<Weapon>,
    keywords: Vec<String>,
    abilities: Vec<String>,
    models: u8
}


/*
 "kaballite_warriors": {
        "unit_stats": [
            {
                "movement": 8,
                "toughness": 3,
                "armor_save": 4,
                "wounds": 1,
                "leadership": 6,
                "objective_control": 2,
                "invulnerable_save": 6
            }
        ],
        "ranged_weapons": [
            "Blast Pistol",
            "Blaster",
            "Dark Lance",
            "Splinter rifle"
        ],
        "melee_weapons": [
            "ccw"
        ],
        "keywords": ["infantry,kabal,battleline,aeldari,kaballite_warriors"],
        "faction_keywords": "drukhari",
        "abilities": ["SADISTIC_RAIDERS"],
        "faction_abilities": "POWER_FROM_PAIN",
        "core_abilities": [],
        "unit_composition": [{"Sybarite":1},{"Kaballite_Warrior":9}],
        "weapon_composition": ["splinter_rifle","ccw"]
    }

*/

