use std::process::exit;
use std::env;
use std::fs;
use std::io::Read;
use std::time;

mod lexical_parser;
mod syntactic_analyzer;


fn main() {
    let mut content: String = "".to_string();
    let filepath = match env::args().nth(1) {
        None => {
            eprintln!("File path not given.");
            exit(0x0);
        },
        Some(file) => {file}
    };

    let mut file = match fs::File::open(filepath) {
        Ok(content) => {content}
        Err(_) => {
            eprintln!("Could not find the given file.");
            exit(0x3);
        }
    };


    file.read_to_string(&mut content).expect("");

    let mut parser = lexical_parser::Automaton::new();

    let timer = time::Instant::now();
    let lexemes = parser.process_input(content);
    println!("lexical parser took {}μs.", timer.elapsed().as_micros());


    for lexeme in lexemes {
        println!("{:?}", lexeme)
    }
}
