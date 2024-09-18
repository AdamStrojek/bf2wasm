
use crate::program::{FunctionBody, Instruction as ProgInstr, Program};

use wasm_encoder::{BlockType, CodeSection, EntityType, ExportKind, ExportSection, Function, FunctionSection, ImportSection, Instruction as WasmInstr, MemArg, MemoryType, Module, TypeSection, ValType};

pub struct Compiler<'a> {
	pub program: &'a Program,
}

impl<'a> Compiler<'a> {
	pub fn new(program: &'a Program) -> Self {
		Self {
			program,
		}
	}

	pub fn compile(&mut self) -> Vec<u8> {
		// (module
		let mut module = Module::new();
		let types = self.generate_types();
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

		self.generate_imports(&mut module);

		//   (type (;{func_type_index};) (func (result i32)))
		let mut functions = FunctionSection::new();
		functions.function(1);  //TODO!
		module.section(&functions);

		//  (export "bf_wasm" (func {func_type_index}))
		let mut exports = ExportSection::new();

		for (fn_num, function) in self.program.functions.iter().enumerate() {
			if let FunctionBody::Body(fn_body) = &function.body {
				exports.export(&function.name, ExportKind::Func, fn_num as u32);
				module.section(&exports);

				// Encode the code section.
				let mut codes = CodeSection::new();

				//   (func $bf_wasm
				//     (local $ptr i32)
				let mut f = Function::new_with_locals_types([ValType::I32]);

				self.generate_body(&mut f, &fn_body);

				//    )
				f.instruction(&WasmInstr::LocalGet(0))
					.instruction(&WasmInstr::Return)
					.instruction(&WasmInstr::End);
				codes.function(&f);
				module.section(&codes);
			}
		}

		// Extract the encoded Wasm bytes for this module.
		module.finish()
	}

	fn generate_imports(&mut self, module: &mut Module) {
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

		for (fn_num, function) in self.program.functions.iter().enumerate() {
			if function.body == FunctionBody::Import {
				imports.import(
					&function.module,
					&function.name,
					EntityType::Function(fn_num as u32),
				);
			}
		}

		module.section(&imports);
	}

	fn generate_types(&mut self) -> TypeSection {
		let mut types = TypeSection::new();

		for fun in &self.program.functions {
			//   (type (###) (func (result i32)))
			types.func_type(&fun.signature);
		}

		types
	}

	fn generate_body(&mut self, f: &mut Function, instructions: &Vec<ProgInstr>) {
		let mut it = instructions.iter();
		while let Some(instr) = it.next() {
			match instr {
				ProgInstr::Block(instructions) => {
					f.instruction(&WasmInstr::Loop(BlockType::Empty)) // Create label at beginning of loop
					 .instruction(&WasmInstr::LocalGet(0)) // Get ptr from local
					 .instruction(&WasmInstr::I32Load8U(MemArg {offset: 0, align: 0, memory_index: 0}));
					f.instruction(&WasmInstr::If(BlockType::Empty));

					self.generate_body(f, instructions);

					f.instruction(&WasmInstr::Br(1)); // Jump back to loop label, value is always 1, jump back by 1 label (skip if)
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_ptr() {
// 	    let mut c = Compiler::from_file("examples/ptr.bf");
// 	    c.compile("examples/ptr.wasm");
//     }
//
// 	#[test]
// 	fn test_val() {
// 		let mut c = Compiler::from_file("examples/val.bf");
// 		c.compile("examples/val.wasm");
// 	}
//
// 	#[test]
// 	fn test_ptr_val() {
// 		let mut c = Compiler::from_file("examples/ptr_val.bf");
// 		c.compile("examples/ptr_val.wasm");
// 	}
//
// 	#[test]
// 	fn test_loop() {
// 		let mut c = Compiler::from_file("examples/loop.bf");
// 		c.compile("examples/loop.wasm");
// 	}
//
// 	#[test]
// 	fn test_loop_inner() {
// 		let mut c = Compiler::from_file("examples/loop_inner.bf");
// 		c.compile("examples/loop_inner.wasm");
// 	}
//
// 	#[test]
// 	fn test_hello() {
// 		let mut c = Compiler::from_file("examples/hello.bf");
// 		c.compile("examples/hello.wasm");
// 	}
//
// 	#[test]
// 	fn test_hello_inner() {
// 		let mut c = Compiler::from_file("examples/hello_inner_loop.bf");
// 		c.compile("examples/hello_inner_loop.wasm");
// 	}
// }
