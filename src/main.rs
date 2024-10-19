use std::env;
use std::fs;
use std::process::exit;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn get_file_contents(filepath: &str) -> String {
    println!("In file {filepath}\n");

    let contents = fs::read_to_string(filepath).expect("Failed to read input file.");

    print!("With text:\n{contents}");

    return contents;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        print!("Problem parsing arguments: {err}\n");
        exit(1);
    });

    let contents = get_file_contents(&config.file_path);
}
