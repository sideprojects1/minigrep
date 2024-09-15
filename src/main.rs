use std::env;

fn parse_command_line_arguments() -> (String, String){
    let args: Vec<String> = env::args().collect();

    let query = args[1].clone();
    let file_path = args[2].clone();

    println!("Searching for {query}");
    println!("In file {file_path}");

    return (query, file_path)
}

fn main() {
    let (query, filepath) = parse_command_line_arguments();
}
