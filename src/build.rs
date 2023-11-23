use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;
use serde_json::{from_str, to_string};

use crate::model::Model;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_string = fs::read_to_string("models.json")?;
    let data: HashMap<String, Vec<Model>> = serde_json::from_str(&json_string)?;

    let out_dir = std::env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("generated_units.rs");
    let mut f = File::create(&dest_path)?;

    for (key, _) in &data {
        writeln!(f, r#"
            pub fn {}_json() -> Vec<Model> {{
                let file = "models.json";
                let json_string = fs::read_to_string(file).expect("Failed to read file");
                let data: HashMap<String, Vec<Model>> = serde_json::from_str(&json_string).unwrap();
    
                let mut unit: Vec<Model> = vec![];
    
                if let Some(models) = data.get("{}") {{
                    for model in models {{
                        let count = if model.get_name() == "SpecialModelName" {{ 1 }} else {{ 9 }};
                        for _ in 0..count {{
                            unit.push(model.clone());
                        }}
                    }}
                }}
    
                unit
            }}
        "#, key.to_snake_case(), key).unwrap();
    }
    

    Ok(())
}
