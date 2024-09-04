use std::fs::{self, File};
use std::io::Write;
use crate::parser::parse;
use crate::program::Instruction as ProgInstr;
use crate::tokenizer::tokenize;

use wasm_encoder::{BlockType, CodeSection, EntityType, ExportKind, ExportSection, Function, FunctionSection, ImportSection, Instruction as WasmInstr, MemArg, MemoryType, Module, TypeSection, ValType};

struct Compiler {
	program: Vec<ProgInstr>,
}

impl Compiler {
	pub fn from_file(filepath: &str) -> Self {
		let src = fs::read_to_string(filepath)
			.expect("File not found");

		let tokens = tokenize(&src);

		let program = parse(&tokens);

		Self {
			program
		}
	}

	pub fn compile(&self, output: &str) {
		// (module
		let mut module = Module::new();

		//   (type (###) (func (result i32)))
		let mut types = TypeSection::new();
		types.function([], [ValType::I32]);
		types.function([ValType::I32], []);
		module.section(&types);

		// let mut memory = MemorySection::new();
		// let memory_local_idx = memory.len();
		// memory.memory(MemoryType {
		// 	minimum: 1,
		// 	maximum: None,
		// 	memory64: false,
		// 	shared: false,
		// 	page_size_log2: None,
		// });


		let mut imports = ImportSection::new();
		imports.import(
			"env",
			"memory",
			MemoryType {
				minimum: 1,
				maximum: None,
				memory64: false,
				shared: false,
				page_size_log2: None,
			}
		);
		imports.import(
			"env",
			"putch",
			EntityType::Function(1),
		);  // fn_idx = 0
		module.section(&imports);

		//   (type (;{func_type_index};) (func (result i32)))
		let mut functions = FunctionSection::new();
		let fn_type_bf_wasm_idx = 0;
		functions.function(fn_type_bf_wasm_idx);
		module.section(&functions);

		//  (export "bf_wasm" (func {func_type_index}))
		let mut exports = ExportSection::new();
		let fn_body_bf_wasm_idx = 1;
		exports.export("bf_wasm", ExportKind::Func, fn_body_bf_wasm_idx);
		module.section(&exports);

		// Encode the code section.
		let mut codes = CodeSection::new();

		//   (func $bf_wasm
		//     (local $ptr i32)
		let mut f = Function::new_with_locals_types([ValType::I32]); // fn_idx = 1

		Self::generate_body(&mut f, &self.program);

		//    )
		f.instruction(&WasmInstr::LocalGet(0))
		 .instruction(&WasmInstr::Return)
		 .instruction(&WasmInstr::End);
		codes.function(&f);
		module.section(&codes);

		// Extract the encoded Wasm bytes for this module.
		let wasm_bytes = module.finish();

		let mut file = File::create(output).unwrap();
		file.write_all(wasm_bytes.as_slice()).unwrap();
	}

	fn generate_body(f: &mut Function, instructions: &Vec<ProgInstr>) {
		let mut it = instructions.iter();
		while let Some(instr) = it.next() {
			match instr {
				ProgInstr::Block(instructions) => {
					f.instruction(&WasmInstr::Loop(BlockType::Empty)) // Create label at beginning of loop
					 .instruction(&WasmInstr::LocalGet(0)) // Get ptr from local
					 .instruction(&WasmInstr::I32Load8U(MemArg {offset: 0, align: 0, memory_index: 0}))
					 .instruction(&WasmInstr::If(BlockType::Empty));

					Self::generate_body(f, instructions);

					f.instruction(&WasmInstr::Br(1)); // Jump back to loop label
					f.instruction(&WasmInstr::End); // Close if block
					f.instruction(&WasmInstr::End); // Close loop block
				},
				ProgInstr::PtrInc(align) => {
					f.instruction(&WasmInstr::LocalGet(0));
					f.instruction(&WasmInstr::I32Const(*align as i32));
					f.instruction(&WasmInstr::I32Add);
					f.instruction(&WasmInstr::LocalSet(0));
				},
				ProgInstr::PtrDec(align) => {
					f.instruction(&WasmInstr::LocalGet(0));
					f.instruction(&WasmInstr::I32Const(*align as i32));
					f.instruction(&WasmInstr::I32Sub);
					f.instruction(&WasmInstr::LocalSet(0));
				},
				ProgInstr::ValInc(val) => {
					f.instruction(&WasmInstr::LocalGet(0)); // Get ptr from local
					f.instruction(&WasmInstr::LocalGet(0)); // Get ptr from local
					f.instruction(&WasmInstr::I32Load8U(MemArg {offset: 0, align: 0, memory_index: 0}));
					f.instruction(&WasmInstr::I32Const(*val as i32));
					f.instruction(&WasmInstr::I32Add);
					f.instruction(&WasmInstr::I32Store8(MemArg {offset: 0, align: 0, memory_index: 0}));
				},
				ProgInstr::ValDec(val) => {
					f.instruction(&WasmInstr::LocalGet(0)); // Get ptr from local
					f.instruction(&WasmInstr::LocalGet(0)); // Get ptr from local
					f.instruction(&WasmInstr::I32Load8U(MemArg {offset: 0, align: 0, memory_index: 0}));
					f.instruction(&WasmInstr::I32Const(*val as i32));
					f.instruction(&WasmInstr::I32Sub);
					f.instruction(&WasmInstr::I32Store8(MemArg {offset: 0, align: 0, memory_index: 0}));
				},
				ProgInstr::GetCh => {
					todo!("GetCh");
				},
				ProgInstr::PutCh => {
					f.instruction(&WasmInstr::LocalGet(0)); // Get ptr from local
					f.instruction(&WasmInstr::I32Load8U(MemArg {offset: 0, align: 0, memory_index: 0}));
					f.instruction(&WasmInstr::Call(0));
				},
			}
		}

	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptr() {
	    let c = Compiler::from_file("examples/ptr.bf");
	    c.compile("examples/ptr.wasm");
    }

	#[test]
	fn test_val() {
		let c = Compiler::from_file("examples/val.bf");
		c.compile("examples/val.wasm");
	}

	#[test]
	fn test_ptr_val() {
		let c = Compiler::from_file("examples/ptr_val.bf");
		c.compile("examples/ptr_val.wasm");
	}

	#[test]
	fn test_hello() {
		let c = Compiler::from_file("examples/hello.bf");
		c.compile("examples/hello.wasm");
	}
}
