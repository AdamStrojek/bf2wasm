use std::iter::Peekable;
use std::slice::Iter;
use crate::program::Instruction;
use crate::tokenizer::Token;

fn count_ch(current: &Token, it: &mut Peekable<Iter<Token>>) -> usize {
	let mut count = 1;
	while it.next_if(|&peek| peek == current).is_some() {
		count += 1;
	}
	count
}

fn parse_expr(it: &mut Peekable<Iter<Token>>) -> Vec<Instruction> {
	let mut instructions: Vec<Instruction> = vec![];

	while let Some(token) = it.next() {
		instructions.push(match token {
			Token::LoopStart => Instruction::Block(parse_expr(it)),
			Token::LoopEnd => break,
			Token::PtrInc => Instruction::PtrInc(count_ch(token, it)),
			Token::PtrDec => Instruction::PtrDec(count_ch(token, it)),
			Token::ValInc => Instruction::ValInc(count_ch(token, it) as u8),
			Token::ValDec => Instruction::ValDec(count_ch(token, it) as u8),
			Token::PutCh => Instruction::PutCh,
			Token::GetCh => Instruction::GetCh,
		});
	}

	instructions
}

pub fn parse(input: &[Token]) -> Vec<Instruction> {
	let mut it = input.iter().peekable();

	parse_expr(&mut it)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expr() {
	    let input = vec![Token::PtrInc, Token::PtrInc, Token::GetCh];
	    assert_eq!(parse(&input), vec![Instruction::PtrInc(2), Instruction::GetCh]);
    }

	#[test]
	fn test_parse_expr_with_blocks() {
		let input = vec![Token::PtrInc, Token::PtrInc, Token::GetCh,
		                             Token::LoopStart,
		                                 Token::PutCh,
		                             Token::LoopEnd,
		                             Token::ValInc];
		assert_eq!(parse(&input), vec![Instruction::PtrInc(2), Instruction::GetCh,
		                               Instruction::Block(vec![Instruction::PutCh]),
		                               Instruction::ValDec(1)]);
	}
}