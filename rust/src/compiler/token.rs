use std::borrow::Cow;

use super::token_kind::TokenKind;

#[derive(Clone, Debug)]
pub struct Token<'source> {
	pub kind: TokenKind,
	pub text: Cow<'source, str>,
	pub line: u32,
}

impl<'source> Token<'source> {
	pub fn synthetic(text: &'source str) -> Self {
		Self {
			text: text.into(),
			kind: TokenKind::Error,
			line: 0,
		}
	}

	pub fn to_static(&self) -> Token<'static> {
		Self {
			text: self.text.to_owned().into(),
			kind: self.kind,
			line: self.line,
		}
	}
}
