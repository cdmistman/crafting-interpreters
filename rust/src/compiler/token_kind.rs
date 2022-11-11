#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
	// single character tokens
	LParen,
	RParen,
	LBrace,
	RBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,

	// one or two character tokens
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// literals
	Identifier,
	String,
	Number,

	// keywords
	And,
	Class,
	Else,
	False,
	For,
	Fun,
	If,
	Nil,
	Or,
	Print,
	Return,
	Super,
	This,
	True,
	Var,
	While,

	// rest
	Error,
	Eof,
}
