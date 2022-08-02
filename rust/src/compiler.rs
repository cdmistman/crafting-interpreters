mod parser;
mod scanner;

use std::rc;

use self::parser::Parser;
use self::scanner::Scanner;
use crate::obj::ObjClosure;
use crate::obj::ObjFunction;
use crate::obj::ObjRef;

enum FunctionKind {}

struct Compiler<'source, 'tokens: 'source> {
	parser:    Parser<'source, 'tokens>,
	enclosing: rc::Weak<Compiler<'source, 'tokens>>,
	function:  ObjRef<ObjFunction>,
}

impl<'source, 'tokens: 'source> Compiler<'source, 'tokens> {
	fn new(
		enclosing: rc::Weak<Compiler<'source, 'tokens>>,
		fun_kind: FunctionKind,
	) -> Self {
		todo!()
	}
}

pub fn compile(source: &str) -> Option<ObjRef<ObjClosure>> {
	let mut parser = Parser::new(Scanner::new(source));

	todo!()
}
