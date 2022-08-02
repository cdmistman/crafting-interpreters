use super::TokenKind;

#[derive(Clone, Copy)]
pub struct Token<'source> {
	pub kind:  TokenKind,
	pub start: &'source str,
	pub line:  u32,
}
