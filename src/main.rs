use std::{env, process};
use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("query {:?}", config.query);
    println!("filename {:?}", config.file);

    if let Err(e) = run(config) {
        eprintln!("App error: {}", e);
        process::exit(1);
    }
}