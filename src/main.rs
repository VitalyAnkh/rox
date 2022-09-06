use rox::scanner::Scanner;
use std::path::Path;
fn main() {
    println!("Hello, world!");
}

fn run_file(path: &Path) {
    let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    run(contents);
}

fn run_prompt() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        run(input);
    }
}

fn run(source: String){
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
}