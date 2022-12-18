use std::io::prelude::*;

fn main() {
    let file = std::fs::File::open("./src/main.rs");

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

    println!("{content}");
}
