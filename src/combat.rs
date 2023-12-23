use crate::{
    model::Model,
    unit::Unit,
    weapon::{self, Weapon},
};
use rand::Rng;
use std::io::{self, Write};

pub fn t_a_save(a_save: i32, ap: i32, i_save: i32) -> bool {
    if i_save <= a_save {
        rand::thread_rng().gen_range(1..7) >= i_save
    } else {
        rand::thread_rng().gen_range(1..7) + ap >= a_save
    }
}

pub fn attack(attacking_unit: &Unit, defending_unit: &mut Unit) {
    attacking_unit.description();
    let models = attacking_unit.get_models();

    let mut total_hits = 0;
    let mut total_successful_hits = 0;
    let mut total_successful_wounds = 0;
    let mut total_damage = 0;
    let mut wounding_weapons: Vec<&Weapon> = Vec::new();

    let result = defending_unit.get_first_model();
    let defender_model = match result {
        Some(model) => model,
        None => {
            println!("No models in the defending unit");
            return;
        }
    };
    let defender_tougness = defender_model.get_toughness() as u8;
    let defender_keywords = defender_model.get_keywords();

    for model in models {
        let weapons = model.get_weapon();
        for weapon in weapons {
            total_hits += weapon.get_n_attacks();
            let successful_hits = std::iter::repeat_with(|| weapon.hit())
                .take(weapon.get_n_attacks() as usize)
                .filter(|hit_result| hit_result.0)
                .count() as i32;
            total_successful_hits += successful_hits;

            let wound_results = std::iter::repeat_with(|| weapon.wound(defender_tougness, defender_keywords))
                .take(successful_hits as usize)
                .filter_map(|(is_wound, _, _)| {
                    if is_wound {
                        // Map successful wounds to their damage values
                        wounding_weapons.push(weapon);
                        Some(weapon.damage())
                    } else {
                        None
                    }
                });

            // Collect the damage values into a Vector and sum them up
            let damage_values: Vec<i32> = wound_results.collect();
            let wounds_damage: i32 = damage_values.iter().sum();

            total_successful_wounds += damage_values.len() as i32;
            total_damage += wounds_damage;
        }
    }

    println!("Total attacks: {}", total_hits);
    println!("Successful hits: {}", total_successful_hits);
    println!("Successful wounds: {}", total_successful_wounds);
    println!("Total damage: {}", total_damage);

    for i in 0..total_successful_wounds {
        println!("Alocate {} of wounds", total_successful_wounds - i);

        let defender_model = select_defender(defending_unit);
        let defender_armor_save = defender_model.get_a_save();
        let defender_i_save = defender_model.get_i_save();

        // Assuming each successful wound is independent and should be resolved separately
        let weapon = wounding_weapons[i as usize];
        let weapon_ap = weapon.get_ap();

        if t_a_save(defender_armor_save, weapon_ap, defender_i_save) {
            println!(
                "{} failed to save against {}'s attack",
                defender_model.get_name(),
                weapon.get_name()
            );
            let damage = weapon.damage();
            defender_model.take_damage(damage);
            println!("{} has took {} damage", defender_model.get_name(), damage);
        } else {
            println!(
                "{} saved against {}'s attack",
                defender_model.get_name(),
                weapon.get_name()
            );
        }
    }

    defending_unit.remove_dead_models();
}

pub fn select_defender(unit: &mut Unit) -> &mut Model {
    // Display the unit description and list of models with indexes
    // unit.description();
    for (index, model) in unit.get_models().iter().enumerate() {
        println!(
            "{}: {} ({} wounds remaining)",
            index,
            model.get_name(),
            model.get_wounds()
        );
    }

    // Prompt the user to select a model
    println!("Enter the index of the model to allocate wounds:");
    let mut selected_index = String::new();
    io::stdout().flush().unwrap(); // Make sure the prompt is printed before reading input
    io::stdin().read_line(&mut selected_index).expect("Failed to read line");
    let selected_index: usize = selected_index.trim().parse().expect("Please type a number!");

    // Validate the selected index
    if selected_index >= unit.get_models().len() {
        println!("Invalid index, please select a correct model index.");
        panic!();
    }

    // For now, just print out which model is selected and the wounds to allocate
    let mut selected_model = &unit.get_models_mut()[selected_index];

    let armor_save = selected_model.get_a_save();
    let i_save = selected_model.get_i_save();

    &mut unit.get_models_mut()[selected_index]

    // Here you would add logic to apply wounds to the selected model
    // This might involve modifying the 'Unit' structure to allow for mutable access to models
    // For example, if you had a method like 'apply_wounds_to_model(index: usize, wounds: i32)'
    // you would call it here
}
