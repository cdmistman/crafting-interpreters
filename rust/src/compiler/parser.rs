use std::cell::RefCell;

use super::scanner::Scanner;
use super::scanner::Token;
use super::scanner::TokenKind;

pub struct Parser<'source> {
	pub scanner: RefCell<Scanner<'source>>,

	pub current:    Token<'source>,
	pub previous:   Option<Token<'source>>,
	pub had_error:  bool,
	pub panic_mode: bool,
}

impl<'source> Parser<'source> {
	pub fn new(source: &'source str) -> Self {
		let mut scanner = RefCell::new(Scanner::new(source));
		let current = scanner.get_mut().scan_token();
		Self {
			scanner,
			current,
			previous: None,
			had_error: false,
			panic_mode: false,
		}
	}

	pub fn error_at(&mut self, tok: &Token<'source>, msg: &str) {
		if self.panic_mode {
			return;
		}
		self.panic_mode = true;

		eprint!("[line {}] Error", tok.line);
		match tok.kind {
			TokenKind::Eof => eprint!(" at end"),
			TokenKind::Error => (),
			_ => eprint!(" at {}", tok.text),
		}

		eprint!(": {msg}");
		self.had_error = true;
	}

	pub fn error(&mut self, msg: &str) {
		self.error_at(&self.current.clone(), msg)
	}
}
