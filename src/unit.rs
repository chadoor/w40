use crate::model::Model;
use crate::weapon::*;
use std::collections::HashMap;
pub struct Unit {
    name: String,
    models: Vec<Model>,
    arsenal: HashMap<String, Vec<Weapon>>,
}

impl Unit {
    pub fn new(name: String, models: Vec<Model>, arsenal: HashMap<String, Vec<Weapon>>) -> Unit {
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

    pub fn get_models_mut(&mut self) -> &mut Vec<Model> {
        &mut self.models
    }

    pub fn get_first_model(&self) -> Option<&Model> {
        self.models.first()
    }

    pub fn get_first_model_mut(&mut self) -> Option<&mut Model> {
        self.models.first_mut()
    }

    pub fn remove_dead_models(&mut self) {
        self.models.retain(|model| model.get_wounds() > 0);
    }
}
