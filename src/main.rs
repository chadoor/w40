mod combat;
mod model;
mod unit;
mod unit_builder;
mod weapon;

use crate::combat::*;
use crate::unit_builder::*;
use model::Model;
use unit::Unit;

fn main() {
    let mut u_kaballite_warriors: Vec<Model> = drukhari_kabalite_warriors();

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

    unit_combat(&u_kaballite_warriors_2, &u_intercessor_squad_2);

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
}
