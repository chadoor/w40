use crate::{model::Model, unit::Unit, weapon::Weapon};
use rand::Rng;

pub fn w_table(strenght: i32, tougness: i32) -> u8 {
    match (strenght, tougness) {
        (s, t) if s >= t * 2 => 2,
        (s, t) if s > t => 3,
        (s, t) if s == t => 4,
        (s, t) if s <= t => 5,
        (s, t) if s * 2 <= t => 6,
        _ => 0,
    }
}

pub fn t_wound(strenght: i32, tougness: i32, weapon: &Weapon, keywords: &Vec<String>) -> (bool, u8) {
    let w_range: u8 = w_table(strenght, tougness);
    weapon.wound(w_range, keywords)
}

pub fn t_a_save(a_save: i32, ap: i32, i_save: i32) -> bool {
    if i_save <= a_save {
        rand::thread_rng().gen_range(1..7) >= i_save
    } else {
        rand::thread_rng().gen_range(1..7) + ap >= a_save
    }
}

pub fn perform_attack(
    a_weapon: &Weapon,
    attacker_strength: i32,
    toughness: i32,
    a_save: i32,
    ap: i32,
    i_save: i32,
    a_name: &str,
    d_name: &str,
    a_w_name: &str,
    defender_keywords: &Vec<String>,
) -> bool {
    let a_hit: (bool, i32) = a_weapon.hit();
    if a_hit.0 {
        println!(
            "{} has hit {} with {} (roll/bs) ({}/{})!",
            a_name,
            d_name,
            a_w_name,
            a_hit.1,
            a_weapon.get_b_skill()
        );
        let a_wound: (bool, u8) = t_wound(attacker_strength, toughness, a_weapon, defender_keywords);
        if a_wound.0 {
            println!("{} has wounded {} with (w_range) {} !", a_name, d_name, a_wound.1);
            if t_a_save(a_save, ap, i_save) {
                println!("{} failes to save", d_name);
                return true;
            }
            println!("{} saves", d_name);
        }
        println!("{} failed to wound {}!", a_name, d_name);
    } else {
        println!(
            "{} has missed {} with {} (roll/bs) ({}/{})! ",
            a_name,
            d_name,
            a_w_name,
            a_hit.1,
            a_weapon.get_b_skill()
        );
    }

    false
}

pub fn combat(attacker: &Model, defender: &mut Model) {
    let attacker_weapons: &Vec<Weapon> = attacker.get_weapon();
    for attacker_weapon in attacker_weapons {
        let attacker_strength: i32 = attacker_weapon.get_strength();
        let attacker_ap: i32 = attacker_weapon.get_ap();
        let attacker_name: &str = attacker.get_name();
        let attacker_weapon_name: &str = attacker_weapon.get_name();
        let defender_toughness: i32 = defender.get_toughness();
        let defender_a_save: i32 = defender.get_a_save();
        let defender_i_save: i32 = defender.get_i_save();
        let defender_name: &str = defender.get_name();
        let defender_keywords: &Vec<String> = defender.get_keywords();

        if perform_attack(
            &attacker_weapon,
            attacker_strength,
            defender_toughness,
            defender_a_save,
            attacker_ap,
            defender_i_save,
            attacker_name,
            defender_name,
            attacker_weapon_name,
            defender_keywords,
        ) {
            let damage: i32 = attacker_weapon.damage();
            defender.take_damage(damage);
            println!("Defender {} has took {} damage", defender.get_name(), damage);
        }
    }
}

pub fn unit_combat(attacker: &Unit, defender: &Unit) {
    let mut attacker_models: &Vec<Model> = attacker.get_models();
    let mut defender_dummy_model: Option<Model> = defender.get_unit_dummy();

    match defender_dummy_model {
        Some(mut model) => {
            for a_model in attacker_models {
                combat(a_model, &mut model);
            }
        }
        None => println!("There are no Units left"),
    }
}
