use super::parser::Parser;
use super::token::Token;
use super::token_kind::TokenKind;
use super::Compiler;

pub enum CompilerRoot<'enclosing, 'source> {
	Compiler(&'enclosing mut Compiler<'enclosing, 'source>),
	Parser(Parser<'source>),
}

// impl<'enclosing, 'source> CompilerRoot<'enclosing, 'source> {
// 	fn find<T, F: FnOnce(&Parser<'source>) -> T>(&self, f: F) -> T {
// 		let mut root = self;
// 		loop {
// 			match root {
// 				CompilerRoot::Compiler(compiler) => root = &compiler.root,
// 				CompilerRoot::Parser(parser) => return f(&parser),
// 			}
// 		}
// 	}

// 	fn find_mut<T, F: FnOnce(&mut Parser<'source>) -> T>(&mut self, f: F) -> T {
// 		let mut root = self;
// 		loop {
// 			match root {
// 				CompilerRoot::Compiler(compiler) => root = &mut compiler.root,
// 				CompilerRoot::Parser(parser) => return f(&mut parser),
// 			}
// 		}
// 	}

// 	pub fn advance(&mut self) -> Token<'source> {
// 		self.find_mut(Parser::advance)
// 	}

// 	pub fn check(&mut self, kind: TokenKind) -> bool {
// 		self.find_mut(|p| p.check(kind))
// 	}

// 	pub fn check_eat(&mut self, kind: TokenKind) -> Option<Token<'source>> {
// 		self.find_mut(|p| p.check_eat(kind))
// 	}

// 	pub fn current(&self) -> Token<'source> {
// 		self.find(|p| p.current)
// 	}

// 	pub fn is_at_end(&self) -> bool {
// 		self.find(Parser::is_at_end)
// 	}

// 	pub fn parser(&mut self) -> &mut Parser<'source> {
// 		self.find_mut(|p| p)
// 	}

// 	pub fn previous(&self) -> Token<'source> {
// 		self.find(|p| p.previous.unwrap())
// 	}
// }
