use clap::Parser;
use std::fs::*;

#[derive(Parser)]
struct Parameters {
    param: Option<String>,
}

fn main() {
    let args = Parameters::parse();
    let param = match args.param {
        Some(param) => param,
        None => {
            println!("Erreur : aucun argument fourni. Veuillez entrer une commande.");
            return;
        }
    };

    let mut command = param.split_whitespace();
    let commande = match command.next() {
        Some(commande) => commande,
        None => {
            println!("Erreur : aucune commande fournie.");
            return;
        }
    };

    match commande {
        "ls" => ls(),
        _ => println!("Command not found"),
    };
}

fn ls() {
    let path = ".";
    match read_dir(path) {
        Ok(dir) => {
            for file in dir {
                match file {
                    Ok(file) => print!("{}  ", file.path().display()),
                    Err(e) => println!("Error reading: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}
