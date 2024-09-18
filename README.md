# bf2wasm - BrainFuck to WebAssembly compiler

A compiler that converts Brainfuck code to WebAssembly (WASM), allowing you to run Brainfuck programs in the browser or on any platform that supports WASM.

This project is just a gimmick to learn more about Rust and WebAssembly

## Features

* Compile Brainfuck code to Webassembly

## How to use?

1. Install using Cargo: `cargo install bf2wasm`
2. Compile a Brainfuck program to WASM: `bf2wasm input.bf output.wasm`
3. Run the compiled WASM program in a browser or any platform that supports WebAssembly, such as Node.js.


## Getting Started

1. Clone this repository to get started with the project code: `git clone https://github.com/AdamStrojek/bf2wasm.git`
2. Install the required dependencies using Cargo, Rust's package manager: `cargo build --release`
3. Run the compiler on your Brainfuck code using the provided CLI tool: `cargo run brainfuck_code.bf`

## Running WebAssembly Modules

**Work in progress, keep in mind that all what is presented here may change at any time**

### Core concepts around Brainfuck WASM Modules
Compiled WASM modules revolve around a few core concepts, what allows to provide coherent expiriance

1. Import Object - you need to provide import object to module that will contain whole environment for execution

#### Import object
```json
{
  "env": {
    "memory": WebAssembly.Memory, // Memory object that is used to store data
    "putch": function(i32) -> void, // Function that prints a character at given index in memory
    "getch": function() -> i32, // Function that reads a character from input and returns it's index in memory
  }
}
```

### Running a WASM module in Node.js
1. Ready to use script is attached to repository `runner.js`
2. Provide WASM module as parameter
3. Run the script: `node runner.js brainfuck_code.wasm`
4. The output will be printed on console

### Running a WASM module in browser
1. Ready to use script is attached to repository `web-runner.js` and `index.html`
1. Start any webserver: `python3 -m http.server 8080`
2. Open browser and navigate to http://localhost:8080/
3. Provide WASM module as parameter in input field and click Start button

### Running a WASM module in wasmtime
1. TODO - need to provide import objects

# TODO

[ ] Add support for `getch`
[ ] Import only required functions from environment
[ ] Automated tests based on provided examples
[ ] Zero memory before each execution (who should be responsible for that?)
