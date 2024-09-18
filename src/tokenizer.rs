const VALID_BF_TOKENS: [char; 8] = ['>', '<', '+', '-', ',', '.', '[', ']'];

pub fn is_valid_bf_token(c: &char) -> bool {
	VALID_BF_TOKENS.contains(c)
}

pub fn tokenize(src: &str) -> Vec<char> {
	src.chars().filter(is_valid_bf_token).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
	    let input = "+a>>b-   -\n-<<<<.";
		let result = tokenize(input);
	    assert_eq!(result, vec!['+', '>', '>', '-', '-', '-', '<', '<', '<', '<', '.']);
    }
}
