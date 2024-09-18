mod interpreter;
mod tokenizer;
mod parser;
mod program;
mod compiler;

use std::process;
use std::io::Write;
use std::fs::File;
use clap::Parser;
use crate::program::{Function, Program};
use crate::compiler::Compiler;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input Brainfuck file
    input: std::path::PathBuf,

    /// Output WASM module
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    if !cli.input.exists() {
        eprintln!("Error: Specified file doesn't exist: {}", cli.input.display());
        process::exit(1);
    }

    let output_name = match cli.output {
        Some(path) => path,
        None => cli.input.with_extension("wasm"),
    };

    let mut program = Program::new();

    let func = Function::import("env", "putch").unwrap();
    program.add_function(func);

    // for each input in input_list
    let func = Function::from_file(cli.input).expect("Failed to parse input file");
    program.add_function(func);

    let mut compiler = Compiler::new(&program);

    let output_bytes = compiler.compile();



    let mut file = File::create(output_name).unwrap();
    file.write_all(output_bytes.as_slice()).unwrap();
}
