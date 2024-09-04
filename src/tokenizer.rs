#[derive(Debug, PartialEq)]
pub enum Token {
	PtrInc,
	PtrDec,
	ValInc,
	ValDec,
	GetCh,
	PutCh,
	LoopStart,
	LoopEnd
}

impl TryFrom<char> for Token {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(Token::PtrInc),
            '<' => Ok(Token::PtrDec),
            '+' => Ok(Token::ValInc),
            '-' => Ok(Token::ValDec),
            ',' => Ok(Token::GetCh),
            '.' => Ok(Token::PutCh),
            '[' => Ok(Token::LoopStart),
            ']' => Ok(Token::LoopEnd),
            _ => Err(()),
        }
    }
}

pub fn tokenize(src: &str) -> Vec<Token> {
	src.chars().filter_map(|ch| ch.try_into().ok()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
	    let input = "+>>---<<<<.";
		let result = tokenize(input);
	    assert_eq!(result, vec![Token::ValInc, Token::PtrInc, Token::PtrInc, Token::ValDec,
	                            Token::ValDec, Token::ValDec, Token::PtrDec, Token::PtrDec,
	                            Token::PtrDec, Token::PtrDec, Token::PutCh])
    }
}
