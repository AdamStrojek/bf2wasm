use std::io::{Write, Read};
use crate::program::{Instruction};

pub struct Interpreter<'a> {
	pub program: Vec<Instruction>,
	pub memory: Vec<u8>,

	ptr: usize,

	// input: &'a dyn Read,
	output: &'a mut dyn Write,
}

impl<'a> Interpreter<'a> {
	pub fn new(program: Vec<Instruction>, output: &'a mut dyn Write) -> Self {
		// TODO avoid copying the whole program
		Self {
			program, memory: vec![0; 4000], ptr: 0, output
		}
	}

	pub fn eval(&mut self) {
		assert_eq!(self.ptr, 0, "Trying to rerun interpreter!");
		self.eval_vec(&self.program.clone());
	}

	fn eval_vec(&mut self, instructions: &Vec<Instruction>) {
		for instruction in instructions {
			self.eval_single(instruction)
		}
	}

	fn eval_single(&mut self, instruction: &Instruction) {
		match instruction {
			Instruction::PtrInc(align) => {
				self.ptr = self.ptr.overflowing_add(*align).0;
			},
			Instruction::PtrDec(align) => {
				self.ptr = self.ptr.overflowing_sub(*align).0;
			},
			Instruction::ValInc(val) => {
				self.memory[self.ptr] = self.memory[self.ptr].overflowing_add(*val).0;
			},
			Instruction::ValDec(val) => {
				self.memory[self.ptr] = self.memory[self.ptr].overflowing_sub(*val).0;
			},
			Instruction::PutCh => {
				self.output.write_all(&self.memory[self.ptr..self.ptr+1]).unwrap();
			}
			Instruction::GetCh => {
				todo!("GetCh not implemented");
			}
			Instruction::Block(instructions) => {
				while self.memory[self.ptr] != 0 {
					self.eval_vec(instructions);
				}
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_eval() {
	    let mut output: Vec<u8> = vec![];
	    let mut interpreter = Interpreter::new(vec![Instruction::ValInc(65), Instruction::PutCh], &mut output);
	    interpreter.eval();
	    assert_eq!(output, vec![65]);
    }
}