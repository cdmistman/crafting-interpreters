use std::fmt::Write;

use eyre::Result;

use super::scanner::Scanner;
use super::scanner::Token;
use super::scanner::TokenKind;

pub struct Parser<'source> {
	pub scanner: Scanner<'source>,

	pub current:   Token<'source>,
	pub previous:  Option<Token<'source>>,
	pub had_error: bool,
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

	pub fn consume(
		&mut self,
		kind: TokenKind,
		msg: &str,
	) -> Result<Token<'source>> {
		if self.check(kind) {
			let res = self.current;
			self.advance();
			Ok(res)
		} else {
			Err(self.error_at_current(msg))
		}
	}

	pub fn error(&mut self, msg: &str) -> eyre::Report {
		let at = self
			.previous
			.expect("error for previous token but there is no previous token");
		self.error_at(at, msg)
	}

	pub fn error_at(&mut self, tok: Token<'source>, msg: &str) -> eyre::Report {
		let mut error = String::with_capacity(22 + msg.len());
		write!(error, "[line {}] Error", tok.line);
		match tok.kind {
			TokenKind::Eof => write!(error, " at end").unwrap(),
			TokenKind::Error => (),
			_ => write!(error, " at {}", tok.text).unwrap(),
		}

		write!(error, ": {msg}");
		self.had_error = true;

		eyre::Report::msg(error)
	}

	pub fn error_at_current(&mut self, msg: &str) -> eyre::Report {
		self.error_at(self.current, msg)
	}
}
