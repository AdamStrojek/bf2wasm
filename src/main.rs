mod interpreter;
mod tokenizer;
mod parser;
mod program;
mod compiler;

use std::{fs, io};

use clap::Parser;

use crate::interpreter::Interpreter;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    filename: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    for filename in &cli.filename {
        println!("Running {}", filename);
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let tokens = tokenizer::tokenize(&contents);
        let parser = parser::parse(&tokens);
        let mut stdout = io::stdout();
        let mut interpreter = Interpreter::new(parser, &mut stdout);

        interpreter.eval();
        println!("MemSlice: {:?}", &interpreter.memory[0..10]);
    }
}


