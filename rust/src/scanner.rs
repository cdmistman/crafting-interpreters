mod token;
mod token_kind;

pub use token::*;
pub use token_kind::*;

pub struct Scanner<'source> {
	start:   &'source str,
	current: &'source str,
	line:    u32,
}

impl<'source> Scanner<'source> {
	pub fn new(source: &'source str) -> Self {
		Self {
			start:   source,
			current: source,
			line:    1,
		}
	}

	pub fn scan_token(&mut self) -> Token {
		self.skip_whitespace();
		self.start = self.current;

		if self.is_at_end() {
			return self.make_token(TokenKind::Eof);
		}

		let Some(ch) = self.advance() else {
			return self.error_token("Unexpected character.");
		};

		match ch {
			ch if is_alpha(ch) => self.identifier(),
			ch if is_digit(ch) => self.number(),
			'"' => self.string(),
			'(' => self.make_token(TokenKind::LParen),
			')' => self.make_token(TokenKind::RParen),
			'{' => self.make_token(TokenKind::LBrace),
			'}' => self.make_token(TokenKind::RBrace),
			';' => self.make_token(TokenKind::Semicolon),
			',' => self.make_token(TokenKind::Comma),
			'.' => self.make_token(TokenKind::Dot),
			'-' => self.make_token(TokenKind::Minus),
			'+' => self.make_token(TokenKind::Plus),
			'/' => self.make_token(TokenKind::Slash),
			'*' => self.make_token(TokenKind::Star),

			'!' => self.consume_match('=', |scanner| {
				scanner.make_token(TokenKind::BangEqual)
			}).unwrap_or_else(|| {
				self.make_token(TokenKind::Bang)
			}),
			'=' => self.consume_match('=', |scanner| {
				scanner.make_token(TokenKind::EqualEqual)
			}).unwrap_or_else(|| {
				self.make_token(TokenKind::Equal)
			}),
			'<' => self.consume_match('=', |scanner| {
				scanner.make_token(TokenKind::LessEqual)
			}).unwrap_or_else(|| {
				self.make_token(TokenKind::Less)
			}),
			'>' => self.consume_match('=', |scanner| {
				scanner.make_token(TokenKind::GreaterEqual)
			}).unwrap_or_else(|| {
				self.make_token(TokenKind::Greater)
			}),

			_ => self.error_token("Unexpected character."),
		}
	}
}

impl<'source> Scanner<'source> {
	fn error_token(&self, msg: &str) -> Token {
		todo!()
	}

	fn make_token(&self, kind: TokenKind) -> Token<'source> {
		todo!()
	}
}

impl<'source> Scanner<'source> {
	fn advance(&mut self) -> Option<char> {
		self.peek().map(|ch| {
			self.current = &self.current[1..];
			ch
		})
	}

	fn identifier(&mut self) -> Token {
		todo!()
	}

	fn is_at_end(&self) -> bool {
		self.current.len() == 0
	}

	fn consume_match<F, T>(&mut self, expect: char, eval: F) -> Option<T>
	where
		F: FnOnce(&mut Self) -> T,
	{
		todo!()
	}

	fn number(&mut self) -> Token {
		todo!()
	}

	fn peek(&self) -> Option<char> {
		self.current.chars().next()
	}

	fn peek_next(&self) -> Option<char> {
		self.current.chars().skip(1).next()
	}

	fn skip_whitespace(&mut self) {
		loop {
			let Some(ch) = self.peek() else {
				return;
			};

			match ch {
				' ' | '\r' | '\t' => _ = self.advance(),
				'\n' => {
					self.line += 1;
					self.advance();
				},

				'/' => {
					if let Some('/') = self.peek_next() {
						while self.peek() != Some('\n') && !self.is_at_end() {
							self.advance();
						}
					} else {
						return;
					}
				},
				_ => return,
			}
		}
	}

	fn string(&mut self) -> Token {
		todo!()
	}
}

fn is_alpha(ch: char) -> bool {
	matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_digit(ch: char) -> bool {
	matches!(ch, '0'..='9')
}
