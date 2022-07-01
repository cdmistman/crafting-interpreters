use super::TokenKind;

pub struct Token<'source> {
	pub kind: TokenKind,
	pub start: &'source str,
	pub len: u32,
	pub line: u32,
}
