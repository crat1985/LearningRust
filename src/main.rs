use std::io::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config{
    name: String,
    age: u8
}

fn main() {
    let file = std::fs::File::open("./config.json");

    match file {
        Err(e) => panic!("Erreur lors de la lecture du fichier ! {e}"),
        _ => ""
    };

    let mut file = file.unwrap();

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(content) => println!("Success ! Size : {content}o"),
        Err(e) => {
            println!("{e}");
            return;
        },
    }

    let parsed: Config = serde_json::from_str(content.as_str()).unwrap();

    println!("{:?}",parsed);
}
