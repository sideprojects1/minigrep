use std::env;
use std::fs;

fn parse_command_line_arguments() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    let query = args[1].clone();
    let file_path = args[2].clone();

    println!("Searching for {query}");
    println!("In file {file_path}");

    return (query, file_path);
}

fn get_file_contents(filepath: String) -> String {
    println!("In file {filepath}");

    let contents = fs::read_to_string(filepath)
        .expect("Failed to read input file.");

    print!("With text:\n{contents}");

    return contents
}

fn main() {
    let (query, filepath) = parse_command_line_arguments();

    let contents = get_file_contents(filepath);
}
