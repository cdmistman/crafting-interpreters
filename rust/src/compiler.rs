mod compile_declaration;
mod compile_expression;
mod compile_statement;
mod parser;
mod scanner;
mod token;
mod token_kind;

use std::cell::RefCell;
use std::rc::Rc;

use eyre::Context;
use eyre::Result;

use self::parser::Parser;
use self::token::Token;
use self::token_kind::TokenKind;
use crate::chunk::Bytecode;
use crate::chunk::Op;
use crate::mem::GcPtr;
use crate::mem::GcRef;
use crate::mem::InlineVec;
use crate::obj::ObjFunction;
use crate::obj::ObjString;
use crate::value::Value;

const MAX_UPVALUES: usize = u8::MAX as _;

scoped_tls::scoped_thread_local!(static CURRENT: Compiler);

pub struct Compiler<'enclosing, 'source: 'enclosing> {
	enclosing: Option<&'enclosing Compiler<'enclosing, 'source>>,

	parser: Rc<RefCell<Parser<'source>>>,
	errors: Vec<eyre::Report>,

	function:   GcPtr<ObjFunction>,
	fn_kind:    FunctionKind,
	superclass: Option<bool>,

	locals:      InlineVec<{ u8::MAX as _ }, Local<'source>>,
	upvalues:    InlineVec<MAX_UPVALUES, Upvalue>,
	scope_depth: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FunctionKind {
	Function,
	Initializer,
	Method,
	Script,
}

struct Local<'token> {
	name:        Token<'token>,
	depth:       Option<usize>,
	is_captured: bool,
}

#[derive(Clone)]
struct Upvalue {
	index:    u8,
	is_local: bool,
}

impl<'enclosing, 'source: 'enclosing> Compiler<'enclosing, 'source> {
	#[doc(hidden)]
	pub fn trace() {
		if CURRENT.is_set() {
			CURRENT.with(|_compiler| todo!())
		}
	}

	/// Creates a safe environment for allocating via the Garbage Collector.
	///
	/// # Safety
	/// The callback passed should only be used to create exactly 1 allocation.
	unsafe fn safe_alloc<T, F: FnOnce() -> T>(&self, f: F) -> T {
		CURRENT.set(std::mem::transmute(self), f)
	}

	fn init(
		parser: Rc<RefCell<Parser<'source>>>,
		fn_kind: FunctionKind,
	) -> Self {
		let p = parser.clone();
		let mut res = Self {
			parser,
			fn_kind,
			enclosing: None,
			errors: Vec::new(),
			function: unsafe { GcPtr::null() },
			superclass: None,
			locals: InlineVec::new(),
			upvalues: InlineVec::new(),
			scope_depth: 0,
		};

		res.function = GcPtr::new(unsafe { res.safe_alloc(ObjFunction::new) });

		if fn_kind != FunctionKind::Script {
			res.function.name = Some(unsafe {
				res.safe_alloc(move || {
					ObjString::new(p.borrow().previous.unwrap().text)
				})
			});
		}

		let token_name = (fn_kind == FunctionKind::Function)
			.then_some("")
			.unwrap_or("this");
		res.locals.push(Local {
			name:        Token::synthetic(token_name),
			depth:       Some(0),
			is_captured: false,
		});

		res
	}

	pub fn new(source: &'source str) -> Self {
		Self::init(
			Rc::new(RefCell::new(Parser::new(source))),
			FunctionKind::Script,
		)
	}

	fn child<'child: 'enclosing>(
		&'child self,
		fn_kind: FunctionKind,
	) -> Compiler<'child, 'source> {
		let parser = self.parser.clone();
		let mut res = Self::init(parser, fn_kind);
		res.enclosing = Some(self);
		res
	}

	pub fn compile(mut self) -> Result<GcRef<ObjFunction>, Vec<eyre::Report>> {
		while !self.parser.borrow_mut().check_eat(TokenKind::Eof) {
			self.declaration();
		}

		let (fun, ups) = self.finish()?;
		assert!(ups.is_empty());
		Ok(fun)
	}

	fn finish(
		mut self,
	) -> Result<
		(GcRef<ObjFunction>, InlineVec<MAX_UPVALUES, Upvalue>),
		Vec<eyre::Report>,
	> {
		if self.errors.is_empty() {
			self.emit_return();
			Ok((self.function.into(), self.upvalues))
		} else {
			Err(self.errors)
		}
	}
}

impl<'enclosing, 'source: 'enclosing> Compiler<'enclosing, 'source> {
	fn advance(&mut self) {
		self.parser.borrow_mut().advance();
	}

	fn check(&self, kind: TokenKind) -> bool {
		self.parser.borrow().check(kind)
	}

	fn check_eat(&mut self, kind: TokenKind) -> bool {
		self.parser.borrow_mut().check_eat(kind)
	}

	fn consume(
		&mut self,
		kind: TokenKind,
		msg: &str,
	) -> Result<Token<'source>> {
		self.parser.borrow_mut().consume(kind, msg)
	}

	fn error(&mut self, msg: &str) -> eyre::Report {
		self.parser.borrow_mut().error(msg)
	}

	fn error_at(&mut self, tok: Token<'source>, msg: &str) -> eyre::Report {
		self.parser.borrow_mut().error_at(tok, msg)
	}

	fn error_at_current(&mut self, msg: &str) -> eyre::Report {
		self.parser.borrow_mut().error_at_current(msg)
	}

	fn previous(&self) -> Token<'source> {
		self.parser.borrow().previous.unwrap()
	}
}

impl<'enclosing, 'source: 'enclosing> Compiler<'enclosing, 'source> {
	fn add_local(&mut self, name: Token<'source>) -> Result<()> {
		if self.locals.is_full() {
			return Err(eyre!("Too many local variables in function."));
		}

		self.locals.push(Local {
			name,
			depth: None,
			is_captured: false,
		});
		Ok(())
	}

	fn begin_scope(&mut self) {
		self.scope_depth += 1;
	}

	fn declare_variable(&mut self) -> Result<()> {
		if self.scope_depth == 0 {
			return Ok(());
		}

		let name = self.previous();
		for local in self.locals.iter() {
			if let Some(depth) = local.depth && depth < self.scope_depth {
				break;
			}

			if local.name.text == name.text {
				return Err(eyre!(
					"Variable {} already defined in this scope.",
					name.text,
				));
			}
		}

		self.add_local(name)
			.with_context(|| format!("declaring variable `{}`", name.text))
	}

	fn define_variable(&mut self, global: u8) {
		if self.scope_depth > 0 {
			self.mark_initialized();
		} else {
			self.emit_bytes(
				Bytecode {
					op: Op::DefineGlobal,
				},
				Bytecode { byte: global },
			)
		}
	}

	fn end_scope(&mut self) {
		self.scope_depth -= 1;

		while let Some(local) = self.locals.last()
				&& let Some(depth) = local.depth
				&& depth > self.scope_depth {
			self.emit_byte(Bytecode {
				op: if local.is_captured { Op::CloseUpvalue } else { Op::Pop },
			});
			self.locals.pop();
		}
	}

	fn identifier_constant(&mut self, token: Token) -> Result<u8> {
		self.make_constant(Value::Obj(ObjString::new(token.text).downcast()))
			.with_context(|| format!("identifier constant `{}`", token.text))
	}

	fn make_constant(&mut self, value: Value) -> eyre::Result<u8> {
		let Ok(const_id) = self.function.chunk.constants.len().try_into() else {
			return Err(self.error("Too many constants in one chunk."));
		};
		self.function.chunk.constants.push(value);
		Ok(const_id)
	}

	fn mark_initialized(&mut self) {
		if self.scope_depth > 0 {
			let ii = self.locals.len() - 1;
			let mut local = &mut self.locals[ii];
			local.depth = Some(self.scope_depth);
		}
	}

	fn resolve_local(&mut self, name: Token) -> Result<Option<usize>> {
		for (ii, local) in self.locals.iter().enumerate() {
			if local.name.text == name.text {
				if local.depth.is_none() {
					return Err(eyre!(
						"Can't read local variable in its own initializer."
					));
				}

				return Ok(Some(ii));
			}
		}
		Ok(None)
	}
}

impl<'enclosing, 'source: 'enclosing> Compiler<'enclosing, 'source> {
	fn emit_byte(&mut self, byte: Bytecode) {
		self.function
			.chunk
			.push(byte, self.parser.borrow().previous.unwrap().line)
	}

	fn emit_bytes(&mut self, byte1: Bytecode, byte2: Bytecode) {
		self.emit_byte(byte1);
		self.emit_byte(byte2);
	}

	fn emit_jump(&mut self, op: Op) -> usize {
		self.emit_byte(Bytecode { op });
		let offset = self.function.chunk.bytecode.len() - 2;
		self.emit_bytes(Bytecode { byte: 0xFF }, Bytecode { byte: 0xFF });
		offset
	}

	fn emit_loop(&mut self, loop_start: usize) -> eyre::Result<()> {
		self.emit_byte(Bytecode { op: Op::Loop });

		let offset = self.function.chunk.bytecode.len() - loop_start + 2;
		if offset > u16::MAX as _ {
			return Err(self.error("Loop body too large."));
		}

		self.emit_bytes(
			Bytecode {
				byte: ((offset >> 8) & 0xFF) as _,
			},
			Bytecode {
				byte: (offset & 0xFF) as _,
			},
		);

		Ok(())
	}

	fn emit_return(&mut self) {
		match self.fn_kind {
			FunctionKind::Initializer => self
				.emit_bytes(Bytecode { op: Op::GetLocal }, Bytecode {
					byte: 0,
				}),
			_ => self.emit_byte(Bytecode { op: Op::Nil }),
		}
		self.emit_byte(Bytecode { op: Op::Return })
	}
}

impl<'enclosing, 'source: 'enclosing> Compiler<'enclosing, 'source> {
	fn block(&mut self) -> Result<()> {
		while !matches!(
			self.parser.borrow_mut().current.kind,
			TokenKind::RBrace | TokenKind::Eof
		) {
			self.declaration();
		}

		self.consume(TokenKind::RBrace, "Expect '}' after block.")
			.context("statement")?;
		Ok(())
	}

	fn declaration(&mut self) {
		let parse_fn = match self.parser.borrow().current.kind {
			TokenKind::Class => Self::class_declaration,
			TokenKind::Fun => Self::fun_declaration,
			TokenKind::Var => Self::var_declaration,
			_ => Self::statement,
		};

		self.advance();
		if let Err(err) = parse_fn(self) {
			self.errors.push(err);
			let mut parser = self.parser.borrow_mut();

			while parser.current.kind != TokenKind::Eof {
				if let Some(Token { kind, .. }) = parser.previous
						&& kind == TokenKind::Semicolon {
					return;
				}

				if matches!(
					parser.current.kind,
					TokenKind::Class
						| TokenKind::Fun | TokenKind::Var
						| TokenKind::For | TokenKind::If
						| TokenKind::While | TokenKind::Print
						| TokenKind::Return
				) {
					return;
				}

				parser.advance();
			}
		}
	}

	fn function(&mut self, kind: FunctionKind) -> Result<()> {
		let mut compiler = self.child(kind);
		compiler.begin_scope();

		compiler
			.consume(TokenKind::LParen, "Expect '(' after function name.")?;
		if !compiler.check(TokenKind::RParen) {
			loop {
				compiler.function.arity += 1;
				if compiler.function.arity > 255 {
					return Err(compiler.error_at_current(
						"Can't have more than 255 parameters.",
					));
				}
				let constant =
					compiler.parse_variable("Expect parameter name.")?;
				compiler.define_variable(constant);

				if compiler.check_eat(TokenKind::Comma) {
					break;
				}
			}
		}

		compiler.consume(TokenKind::RParen, "Expect ')' after parameters.")?;
		compiler
			.consume(TokenKind::LBrace, "Expect '{' before function body.")?;
		compiler.block().context("function body")?;

		match compiler.finish() {
			Ok((function, upvalues)) => {
				let byte = self.make_constant(function.value())?;
				self.emit_bytes(Bytecode { op: Op::Closure }, Bytecode {
					byte,
				});
				for upvalue in upvalues {
					let local_byte = upvalue.is_local.then_some(1).unwrap_or(0);
					self.emit_bytes(Bytecode { byte: local_byte }, Bytecode {
						byte: upvalue.index,
					});
				}
				Ok(())
			},
			Err(errors) => {
				self.errors.extend(errors);
				Err(eyre!("encountered errors while parsing function"))
			},
		}
	}

	fn method(&mut self) -> Result<()> {
		self.consume(TokenKind::Identifier, "Expect class name.")?;
		let constant = self
			.identifier_constant(self.previous())
			.context("method name")?;

		let kind = if self.previous().text == "init" {
			FunctionKind::Initializer
		} else {
			FunctionKind::Method
		};
		self.function(kind)?;
		self.emit_bytes(Bytecode { op: Op::Method }, Bytecode {
			byte: constant,
		});
		Ok(())
	}

	fn parse_variable(&mut self, error_message: &str) -> Result<u8> {
		todo!()
	}

	fn statement(&mut self) -> Result<()> {
		let parse_fn = match self.parser.borrow().current.kind {
			TokenKind::LBrace => |p: &mut Self| {
				p.begin_scope();
				p.block()?;
				Ok(p.end_scope())
			},
			TokenKind::For => Self::for_statement,
			TokenKind::If => Self::if_statement,
			TokenKind::Print => Self::print_statement,
			TokenKind::Return => Self::return_statement,
			TokenKind::While => Self::while_statement,
			_ => Self::expression_statement,
		};

		self.advance();
		parse_fn(self)
	}
}
