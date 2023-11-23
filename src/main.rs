mod combat;
mod model;
mod unit;
mod unit_builder;
mod weapon;

use crate::combat::*;
use crate::unit_builder::*;
use model::Model;
use unit::Unit;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug)]
struct Dog {
    name: String,
    year_born: i32,
    owner: DogOwner,
}

#[derive(Serialize, Deserialize, Debug)]
struct DogOwner {
    first_name: String,
    last_name: String,
}

fn example() {
    let owner_01 = DogOwner {
        first_name: "Trevor".to_string(),
        last_name: "Sullivan".to_string(),
    };

    let dog_01: Dog = Dog {
        name: "Cheyenne".to_string(),
        year_born: 2021,
        owner: owner_01,
    };
    let result = to_string(&dog_01);
    if let Ok(s) = result {
        println!("{}", s);
    } else {
        println!("There was a problem with serilazation");
    }
}

fn deserialize() {
    let json_string = r#"{"name":"Cheyenne","year_born":2021,"owner":{"first_name":"Trevor","last_name":"Sullivan"}}"#;
    let dog_deser = from_str::<Dog>(json_string);
    if let Ok(s) = dog_deser {
        println!("{:#?}", s);
    }
}

fn main() {
    let mut u_kaballite_warriors: Vec<Model> = drukhari_kabalite_warriors_fjson();

    let mut u_intercessor_squad: Vec<Model> = sm_intercessor_squad(5);

    // for model in &u_intercessor_squad {
    //     println!("{}", model.description());
    //     for weapon in model.get_weapon() {
    //         println!("  {}", weapon.description());
    //     }
    // }

    let mut u_kaballite_warriors_2: Unit = Unit::new(
        "Kabalite Warriors".to_string(),
        drukhari_kabalite_warriors(),
        arsenal_kabalite_warrior(),
    );

    let mut u_intercessor_squad_2 = Unit::new(
        "Intercessor Squad".to_string(),
        sm_intercessor_squad(5),
        arsenal_kabalite_warrior(),
    );

    u_kaballite_warriors_2.description();
    u_intercessor_squad_2.description();

    //unit_combat(&u_kaballite_warriors_2, &u_intercessor_squad_2);

    // let var = u_intercessor_squad_2.get_models();

    // for val in var  {
    //     println!("{:?}",val.get_keywords());
    // }

    //combat(&u_kaballite_warriors[0], &mut u_intercessor_squad[0]);

    // for model in u_kaballite_warriors {
    //     println!("{}", model.description());
    // }

    // for model in u_intercessor_squad {
    //     println!("{}", model.description());
    // }

    // combat(&kaballite_warrior_1, &mut kaballite_warrior_2);

    //example();

    //deserialize();

    //build_arsenal_from_json();
}
