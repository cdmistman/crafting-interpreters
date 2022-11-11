use super::scanner::Scanner;
use super::scanner::Token;
use super::scanner::TokenKind;

pub struct Parser<'source> {
	pub scanner: Scanner<'source>,

	pub current:    Token<'source>,
	pub previous:   Option<Token<'source>>,
	pub had_error:  bool,
	pub panic_mode: bool,
}

impl<'source> Parser<'source> {
	pub fn new(source: &'source str) -> Self {
		let mut scanner = Scanner::new(source);
		let current = scanner.scan_token();
		Self {
			scanner,
			current,
			previous: None,
			had_error: false,
			panic_mode: false,
		}
	}

	pub fn advance(&mut self) {
		self.previous = Some(self.current);

		loop {
			self.current = self.scanner.scan_token();
			if self.current.kind != TokenKind::Error {
				break;
			}

			self.error_at_current(self.current.text);
		}
	}

	pub fn check(&self, kind: TokenKind) -> bool {
		self.current.kind == kind
	}

	pub fn check_eat(&mut self, kind: TokenKind) -> bool {
		let res = self.check(kind);
		if res {
			self.advance();
		}
		res
	}

	pub fn consume(&mut self, kind: TokenKind, msg: &str) {
		if self.check(kind) {
			self.advance();
		} else {
			self.error_at_current(msg);
		}
	}

	pub fn error(&mut self, msg: &str) {
		let at = self
			.previous
			.expect("error for previous token but there is no previous token");
		self.error_at(at, msg)
	}

	pub fn error_at(&mut self, tok: Token<'source>, msg: &str) {
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

	pub fn error_at_current(&mut self, msg: &str) {
		self.error_at(self.current, msg)
	}
}
