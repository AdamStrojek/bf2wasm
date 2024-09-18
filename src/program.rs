use std::path::PathBuf;
use wasm_encoder::{FuncType, ValType};
use crate::parser::parse;
use crate::tokenizer::tokenize;

pub struct Program {
	pub functions: Vec<Function>,
}

impl Program {
	pub fn new() -> Self {
		Program { functions: Vec::new() }
	}

	pub fn add_function(&mut self, function: Function) {
		self.functions.push(function);
	}
}

#[derive(Debug, PartialEq)]
pub struct Function {
	pub module: String,
	pub name: String,
	pub signature: FuncType,
	pub body: FunctionBody,
}

#[derive(Debug, PartialEq)]
pub enum FunctionBody {
	Import,
	Body(Vec<Instruction>),
}

impl Function {
	pub fn from_file(file_path: PathBuf) -> Result<Self, String> {
		let filename = file_path.file_name()
			.ok_or("Encountered problems during file name parsing")?
			.to_str().ok_or("Failed to convert file name to string")?;

		let name = filename.replace(".bf", "").to_string();

		// TODO: Fix signature
		let signature = FuncType::new([], [ValType::I32]);
		
		let content = std::fs::read_to_string(&file_path).map_err(|err| err.to_string() )?;
		
		let tokens = tokenize(&content);

		let body = FunctionBody::Body(parse(&tokens));

		Ok(Function {
			module: "".to_string(),
			name,
			signature,
			body,
		})
	}

	pub fn import(module: &str, name: &str) -> Result<Self, String> {
		// TODO: Fix signature
		let signature = FuncType::new([ValType::I32], []);

		Ok(Function {
			module: module.to_string(),
			name: name.to_string(),
			signature,
			body: FunctionBody::Import,
		})
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
	PtrInc(usize),
	PtrDec(usize),
	ValInc(u8),
	ValDec(u8),
	GetCh,
	PutCh,
	Block(Vec<Instruction>),
}