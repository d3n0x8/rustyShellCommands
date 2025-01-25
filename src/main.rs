use clap::Parser;
use std::fs::*;

#[derive(Parser)]
struct Parameters {
    param: String,

    #[arg(short, long)]
    all: bool,

    #[arg(default_value_t = String::from("."))]
    argum: String,
}

fn main() {
    let args = Parameters::parse();

    let commande = &args.param;

    match commande.as_str() {
        "ls" => ls(args.all, args.argum),
        _ => println!("Command not found."),
    };
}

fn ls(all: bool, argum: String) {
    let path = argum;
    if let Ok(dir) = read_dir(path) {
        for file in dir {
            if let Ok(file) = file {
                if let Some(file_name) = file.file_name().to_str() {
                    if all || !file_name.starts_with('.') {
                        print!("{}  ", file_name);
                    }
                } else {
                    println!("Error : Failed to convert name to string.");
                }
            } else {
                println!("Error reading");
            }
        }
    } else {
        println!("Error reading directory");
    }
}
