use std::env;
use std::process::exit;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        print!("Problem parsing arguments: {err}\n");
        exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        exit(1)
    }
}
