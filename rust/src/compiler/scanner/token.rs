use super::TokenKind;

#[derive(Clone, Copy)]
pub struct Token<'source> {
	pub kind: TokenKind,
	pub text: &'source str,
	pub line: u32,
}
