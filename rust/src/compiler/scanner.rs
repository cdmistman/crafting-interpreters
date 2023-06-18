pub use super::token::*;
pub use super::token_kind::*;

pub struct Scanner<'source> {
	start:   &'source str,
	current: &'source str,
	line_no: u32,
}

impl<'source> Scanner<'source> {
	pub fn new(source: &'source str) -> Self {
		Self {
			start:   source,
			current: source,
			line_no: 1,
		}
	}

	pub fn scan_token(&mut self) -> Token<'source> {
		self.skip_whitespace();

		if self.is_at_end() {
			return self.make_token(TokenKind::Eof);
		}

		match self.advance() {
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

			'!' => self
				.match_ch('=', |scanner| {
					Some(scanner.make_token(TokenKind::BangEqual))
				})
				.unwrap_or_else(|| self.make_token(TokenKind::Bang)),
			'=' => self
				.match_ch('=', |scanner| {
					Some(scanner.make_token(TokenKind::EqualEqual))
				})
				.unwrap_or_else(|| self.make_token(TokenKind::Equal)),
			'<' => self
				.match_ch('=', |scanner| {
					Some(scanner.make_token(TokenKind::LessEqual))
				})
				.unwrap_or_else(|| self.make_token(TokenKind::Less)),
			'>' => self
				.match_ch('=', |scanner| {
					Some(scanner.make_token(TokenKind::GreaterEqual))
				})
				.unwrap_or_else(|| self.make_token(TokenKind::Greater)),

			_ => self.error_token("Unexpected character."),
		}
	}
}

impl<'source> Scanner<'source> {
	fn error_token(&self, msg: &'static str) -> Token<'source> {
		Token {
			kind: TokenKind::Error,
			text: msg.into(),
			line: self.line_no,
		}
	}

	fn make_token(&mut self, kind: TokenKind) -> Token<'source> {
		let res = Token {
			kind,
			text: (&self.start[..self.start.len() - self.current.len()]).into(),
			line: self.line_no,
		};
		self.start = self.current;
		res
	}
}

impl<'source> Scanner<'source> {
	fn advance(&mut self) -> char {
		let res = self.peek();
		if res != '\0' {
			self.current = &self.current[1..];
		}
		res
	}

	fn check_keyword(
		&self,
		offset: usize,
		kw: &str,
		kind: TokenKind,
	) -> TokenKind {
		if offset + kw.len() == self.start.len() - self.current.len() {
			if kw == &self.start[offset..offset + kw.len()] {
				return kind;
			}
		}
		TokenKind::Identifier
	}

	fn identifier_type(&self) -> TokenKind {
		let mut chars = self
			.start
			.chars()
			.take(self.start.len() - self.current.len());

		match chars.next() {
			None => TokenKind::Error,
			Some('a') => self.check_keyword(1, "nd", TokenKind::And),
			Some('c') => self.check_keyword(1, "lass", TokenKind::Class),
			Some('e') => self.check_keyword(1, "lse", TokenKind::Else),
			Some('f') => match chars.next() {
				Some('a') => self.check_keyword(2, "lse", TokenKind::False),
				Some('o') => self.check_keyword(2, "or", TokenKind::For),
				Some('u') => self.check_keyword(2, "n", TokenKind::Fun),
				_ => TokenKind::Identifier,
			},
			Some('i') => self.check_keyword(1, "f", TokenKind::If),
			Some('n') => self.check_keyword(1, "il", TokenKind::Nil),
			Some('o') => self.check_keyword(1, "r", TokenKind::Or),
			Some('p') => self.check_keyword(1, "rint", TokenKind::Print),
			Some('r') => self.check_keyword(1, "eturn", TokenKind::Return),
			Some('s') => self.check_keyword(1, "uper", TokenKind::Super),
			Some('t') => match chars.next() {
				Some('h') => self.check_keyword(2, "is", TokenKind::This),
				Some('r') => self.check_keyword(2, "ue", TokenKind::True),
				_ => TokenKind::Identifier,
			},
			Some('v') => self.check_keyword(1, "ar", TokenKind::Var),
			Some('w') => self.check_keyword(1, "hile", TokenKind::While),
			Some(_) => TokenKind::Identifier,
		}
	}

	fn identifier(&mut self) -> Token<'source> {
		while let ch = self.peek() && (is_alpha(ch) || is_digit(ch)) {
			self.advance();
		}
		self.make_token(self.identifier_type())
	}

	fn is_at_end(&self) -> bool {
		self.current.len() == 0
	}

	fn match_ch<F, T>(&mut self, expect: char, eval: F) -> Option<T>
	where
		F: FnOnce(&mut Self) -> Option<T>,
	{
		if self.peek() != expect {
			return None;
		}
		self.advance();
		eval(self)
	}

	fn number(&mut self) -> Token<'source> {
		while is_digit(self.peek()) {
			self.advance();
		}

		if self.peek() == '.' && is_digit(self.peek_next()) {
			// consume '.'
			self.advance();
			// consume the first decimal place
			self.advance();

			while is_digit(self.peek()) {
				self.advance();
			}
		}

		return self.make_token(TokenKind::Number);
	}

	fn peek(&self) -> char {
		self.current.chars().next().unwrap_or('\0')
	}

	fn peek_next(&self) -> char {
		self.current.chars().skip(1).next().unwrap_or('\0')
	}

	fn skip_whitespace(&mut self) {
		loop {
			match self.peek() {
				' ' | '\r' | '\t' => _ = self.advance(),
				'\n' => {
					self.line_no += 1;
					self.advance();
				},
				'/' => {
					if self.peek_next() == '/' {
						while self.peek() != '\n' && !self.is_at_end() {
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

	fn string(&mut self) -> Token<'source> {
		while !self.is_at_end() {
			match self.peek() {
				'"' => {
					self.advance();
					return self.make_token(TokenKind::String);
				},
				'\n' => self.line_no += 1,
				'\\' if self.peek_next() == '"' => {
					self.advance();
				},
				_ => (),
			}
			self.advance();
		}
		self.error_token("Unterminated string.")
	}
}

fn is_alpha(ch: char) -> bool {
	matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_digit(ch: char) -> bool {
	matches!(ch, '0'..='9')
}
