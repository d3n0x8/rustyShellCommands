use clap::Parser;
use std::fs::*;

#[derive(Parser)]
struct Parameters {
    param: String,
}

fn main() {
    let args = Parameters::parse();

    let commande = &args.param;

    match commande.as_str() {
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
        Err(e) => println!("Error reading directory: {}", e),
    }
}
