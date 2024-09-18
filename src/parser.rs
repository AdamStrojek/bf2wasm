use std::iter::Peekable;
use std::slice::Iter;
use crate::program::Instruction;

fn count_ch(current: &char, it: &mut Peekable<Iter<char>>) -> usize {
	let mut count = 1;
	while it.next_if(|&peek| peek == current).is_some() {
		count += 1;
	}
	count
}

fn parse_expr(it: &mut Peekable<Iter<char>>) -> Vec<Instruction> {
	let mut instructions: Vec<Instruction> = vec![];

	while let Some(token) = it.next() {
		instructions.push(match token {
			'[' => Instruction::Block(parse_expr(it)),
			']' => break,
			'>' => Instruction::PtrInc(count_ch(token, it)),
			'<' => Instruction::PtrDec(count_ch(token, it)),
			'+' => Instruction::ValInc(count_ch(token, it) as u8),
			'-' => Instruction::ValDec(count_ch(token, it) as u8),
			'.' => Instruction::PutCh,
			',' => Instruction::GetCh,
			_ => panic!("Invalid token: {}", token),
		});
	}

	instructions
}

pub fn parse(input: &[char]) -> Vec<Instruction> {
	let mut it = input.iter().peekable();

	parse_expr(&mut it)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expr() {
	    let input = ">>,".chars().collect::<Vec<char>>();
	    assert_eq!(parse(&input), vec![Instruction::PtrInc(2), Instruction::GetCh]);
    }

	#[test]
	fn test_parse_expr_with_blocks() {
		let input = ">>,[.]-".chars().collect::<Vec<char>>();
		assert_eq!(parse(&input), vec![Instruction::PtrInc(2), Instruction::GetCh,
		                               Instruction::Block(vec![Instruction::PutCh]),
		                               Instruction::ValDec(1)]);
	}
}
