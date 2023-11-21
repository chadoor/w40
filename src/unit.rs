use crate::model::Model;
use crate::weapon::*;
use std::collections::HashMap;
pub struct Unit {
    name: String,
    models: Vec<Model>,
    arsenal: Vec<Weapon>,
}

impl Unit {
    pub fn new(name: String, models: Vec<Model>, arsenal: Vec<Weapon>) -> Unit {
        Unit { name, models, arsenal }
    }

    pub fn assign_weapons() {}

    /*
     INTERCESSOR SQUAD
     T SV W
     4 3+ 2
     1xIntercessor Sergeant : bolt pistol, bolt rifle
     4xIntercessors: bolt pistol, bolt rifle
    */

    // todo  Optional:make this function return string
    pub fn description(&self) {
        let mut counter: i32;

        let m_tougness = self.models[0].get_toughness();
        let m_save = self.models[0].get_a_save();
        let m_wound = self.models[0].get_wounds();

        println!("");
        println!("{}", self.name);
        println!("T SV W");
        println!("{} {}+ {}", m_tougness, m_save, m_wound);

        // let mut model_count: HashMap<&str, i32> = HashMap::new();
        // for model in &self.models {
        //     *model_count.entry(model.get_name()).or_insert(0) += 1;
        // }

        let mut model_count = HashMap::new();
        for model in &self.models {
            let entry = model_count
                .entry(model.get_name())
                .or_insert_with(|| (0, model.get_weapon_names()));
            entry.0 += 1;
        }

        for (model_name, (count, equipment)) in model_count {
            println!("{}x{}: {}", count, model_name, equipment);
        }
        println!("");
    }

    pub fn get_models(&self) -> &Vec<Model> {
        &self.models
    }

    pub fn get_unit_dummy(&self) -> Option<Model> {
        let f_model = self.models.first();
        match f_model {
            Some(model) => {
                let mut dummy = Model::new(
                    model.get_name().to_string(),
                    model.get_toughness(),
                    model.get_a_save(),
                    model.get_wounds(),
                    Weapon::new("Dark Lance".to_string(), 1, 4, 12, -3, 2, 6, vec![Ability::HEAVY]),
                );
                dummy.set_i_save(model.get_i_save());
                dummy.add_keyword_vr(model.get_keywords());
                Some(dummy)
            }
            None => {
                println!("There are no more models");
                None
            }
        }
    }
}
