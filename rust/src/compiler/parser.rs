use std::fmt::Write;

use super::scanner::Scanner;
use super::scanner::Token;
use super::scanner::TokenKind;

pub type Error = eyre::Report;

pub struct Parser<'source> {
	pub scanner: Scanner<'source>,

	pub current:  Token<'source>,
	pub previous: Token<'source>,
}

impl<'source> Parser<'source> {
	pub fn new(source: &'source str) -> Self {
		let mut scanner = Scanner::new(source);
		let current = scanner.scan_token();
		Self {
			scanner,
			current,
			previous: Token {
				kind: TokenKind::Sof,
				text: source[0..=0].into(),
				line: 1,
			},
		}
	}

	pub fn advance(&mut self, errors: &mut Vec<Error>) -> Token<'source> {
		self.previous = self.current;

		loop {
			self.current = self.scanner.scan_token();
			if self.current.kind != TokenKind::Error {
				return self.current;
			}

			errors.push(self.error_at_current(self.current.text.as_ref()));
		}
	}

	pub fn check(&self, kind: TokenKind) -> bool {
		self.current.kind == kind
	}

	pub fn check_eat(
		&mut self,
		kind: TokenKind,
		errors: &mut Vec<Error>,
	) -> Option<Token<'source>> {
		self.check(kind).then(|| self.advance(errors))
	}

	pub fn consume(
		&mut self,
		kind: TokenKind,
		msg: impl AsRef<str>,
		errors: &mut Vec<Error>,
	) -> Result<Token<'source>, ()> {
		if self.check(kind) {
			Ok(self.advance(errors))
		} else {
			errors.push(self.error_at_current(msg));
			Err(())
		}
	}

	pub fn error(&self, msg: impl AsRef<str>) -> Error {
		error_at(self.previous, msg)
	}

	pub fn error_at_current(&self, msg: impl AsRef<str>) -> Error {
		error_at(self.current, msg)
	}
}

pub fn error_at(tok: Token, msg: impl AsRef<str>) -> Error {
	let msg = msg.as_ref();
	let mut error = String::with_capacity(22 + msg.len());

	write!(error, "[line {}] Error", tok.line);
	match tok.kind {
		TokenKind::Eof => write!(error, " at end").unwrap(),
		TokenKind::Error => (),
		_ => write!(error, " at {}", tok.text).unwrap(),
	}
	write!(error, ": {msg}");

	eyre::Report::msg(error)
}
