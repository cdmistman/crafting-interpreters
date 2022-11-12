use super::token_kind::TokenKind;

#[derive(Clone, Copy)]
pub struct Token<'source> {
	pub kind: TokenKind,
	pub text: &'source str,
	pub line: u32,
}

impl<'source> Token<'source> {
	pub fn synthetic(text: &'source str) -> Self {
		Self {
			text,
			kind: TokenKind::Error,
			line: 0,
		}
	}
}
