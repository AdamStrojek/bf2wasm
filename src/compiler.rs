
use crate::program::{FunctionBody, Instruction as ProgInstr, Program};

use wasm_encoder::{BlockType, CodeSection, EntityType, ExportKind, ExportSection, Function,
                   FunctionSection, ImportSection, Instruction as WasmInstr, MemArg, MemoryType,
                   Module, TypeSection, ValType};

pub struct Compiler<'a> {
	pub program: &'a Program,

	types: TypeSection,
	// memory: MemorySection,
	imports: ImportSection,
	functions: FunctionSection,
	exports: ExportSection,
	codes: CodeSection,
}

impl<'a> Compiler<'a> {
	pub fn new(program: &'a Program) -> Self {
		Self {
			program,
			types: TypeSection::new(),
			// memory: MemorySection::new(),
			imports: ImportSection::new(),
			functions: FunctionSection::new(),
			exports: ExportSection::new(),
			codes: CodeSection::new(),
		}
	}

	pub fn compile(&mut self) -> Vec<u8> {

		self.imports.import(
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

		// self.memory.memory(MemoryType {
		// 	minimum: 1,
		// 	maximum: None,
		// 	memory64: false,
		// 	shared: false,
		// 	page_size_log2: None,
		// });

		for (fn_idx, function) in self.program.functions.iter().enumerate() {
			let fn_idx = fn_idx as u32;
			let fn_type_idx = fn_idx;

			self.types.func_type(&function.signature);

			match &function.body {
				FunctionBody::Import => {
					self.imports.import(
						&function.module,
						&function.name,
						EntityType::Function(fn_type_idx),
					);
				},
				FunctionBody::Body(fn_body) => {
					self.functions.function(fn_idx);
					self.exports.export(&function.name, ExportKind::Func, fn_idx);

					let mut f = Function::new_with_locals_types([ValType::I32]);

					self.generate_body(&mut f, &fn_body);

					f.instruction(&WasmInstr::LocalGet(0))
						.instruction(&WasmInstr::Return)
						.instruction(&WasmInstr::End);
					self.codes.function(&f);
				}
			};
		}

		let mut module = Module::new();

		module.section(&self.types);
		// module.section(&self.memory);
		module.section(&self.imports);
		module.section(&self.functions);
		module.section(&self.exports);
		module.section(&self.codes);

		// Extract the encoded Wasm bytes for this module.
		module.finish()
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
