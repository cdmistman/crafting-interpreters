use std::borrow::Cow;

use super::scanner::Scanner;
use super::scanner::Token;
use super::scanner::TokenKind;

pub struct Parser<'source, 'token: 'source> {
	pub scanner: Scanner<'source>,

	pub current:    Token<'token>,
	pub previous:   Option<Token<'token>>,
	pub had_error:  bool,
	pub panic_mode: bool,
}

impl<'source, 'token: 'source> Parser<'source, 'token> {
	pub fn new(mut scanner: Scanner<'source>) -> Self {
		let current = scanner.scan_token();
		Self {
			scanner,
			current,
			previous: None,
			had_error: false,
			panic_mode: false,
		}
	}

	pub fn error_at(&mut self, tok: &Token<'token>, msg: &str) {
		if self.panic_mode {
			return;
		}
		self.panic_mode = true;

		eprint!("[line {}] Error", tok.line);
		match tok.kind {
			TokenKind::Eof => eprint!(" at end"),
			TokenKind::Error => (),
			_ => eprint!(" at {}", tok.start),
		}

		eprint!(": {msg}");
		self.had_error = true;
	}

	pub fn error(&mut self, msg: &str) {
		self.error_at(&self.current.clone(), msg)
	}
}
