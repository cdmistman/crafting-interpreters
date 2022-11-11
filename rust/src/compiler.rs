mod parser;
mod scanner;
mod token;
mod token_kind;

use std::rc::Rc;

use eyre::Result;

use self::parser::Parser;
use self::token::Token;
use crate::mem::GcRef;
use crate::mem::InlineVec;
use crate::obj::ObjFunction;

scoped_tls::scoped_thread_local!(static ENCLOSING: Compiler);

pub struct Compiler<'source> {
	parser: Rc<Parser<'source>>,

	function: GcRef<ObjFunction>,
	fn_kind:  FunctionKind,

	locals:      InlineVec<{ u8::MAX as _ }, Local<'source>>,
	upvalues:    InlineVec<{ u8::MAX as _ }, Upvalue>,
	scope_depth: usize,
}

pub enum FunctionKind {
	Function,
	Initializer,
	Method,
	Script,
}

struct Local<'token> {
	name:        Token<'token>,
	depth:       usize,
	is_captured: bool,
}

struct Upvalue {
	index:    u8,
	is_local: bool,
}

impl<'source> Compiler<'source> {
	pub fn new(source: &'source str, function_kind: FunctionKind) -> Self {
		let parser = Rc::new(Parser::new(source));
		let function = ObjFunction::new();
		todo!()
	}

	pub fn compile(&mut self) -> Result<GcRef<ObjFunction>> {
		todo!()
	}
}
