mod compile_declaration;
mod compile_expression;
mod compile_statement;
mod compiler_root;
mod parser;
mod scanner;
mod token;
mod token_kind;

use drop_bomb::DropBomb;

use self::compile_expression::Precedence;
use self::compiler_root::CompilerRoot;
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

type Result<T = (), E = ()> = std::result::Result<T, E>;

const MAX_UPVALUES: usize = u8::MAX as _;

scoped_tls::scoped_thread_local!(static CURRENT: Compiler);

struct ConstId(u8);

pub struct Compiler<'enclosing, 'source> {
	root:   CompilerRoot<'enclosing, 'source>,
	errors: Vec<eyre::Report>,

	function:   GcPtr<ObjFunction>,
	fn_kind:    FunctionKind,
	superclass: Option<bool>,

	locals:      InlineVec<{ u8::MAX as _ }, Local<'source>>,
	upvalues:    InlineVec<MAX_UPVALUES, Upvalue>,
	scope_depth: usize,

	bomb: DropBomb,
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

impl<'source> Compiler<'static, 'source> {
	pub fn new(source: &'source str) -> Self {
		let parser = Parser::new(source);
		Compiler::init(CompilerRoot::Parser(parser), FunctionKind::Script)
	}
}

impl<'enclosing, 'source> Compiler<'enclosing, 'source> {
	unsafe fn alloc_safe<T, F: FnOnce() -> T>(&self, f: F) -> T {
		todo!();
		CURRENT.set(self, f)
	}

	fn child<'child: 'enclosing>(
		&'child mut self,
		fn_kind: FunctionKind,
	) -> Compiler<'child, 'source> {
		let root = CompilerRoot::Compiler(self);
		Compiler::init(root, fn_kind)
	}

	fn finish(
		mut self,
	) -> Result<
		(GcRef<ObjFunction>, InlineVec<MAX_UPVALUES, Upvalue>),
		Vec<eyre::Report>,
	> {
		if self.errors.is_empty() {
			self.emit_return();
			self.bomb.defuse();
			Ok((self.function.into(), self.upvalues))
		} else {
			self.bomb.defuse();
			Err(self.errors)
		}
	}

	fn init(
		root: CompilerRoot<'enclosing, 'source>,
		fn_kind: FunctionKind,
	) -> Self {
		let mut compiler = Self {
			root,
			fn_kind,
			scope_depth: 0,
			superclass: None,
			errors: Vec::new(),
			function: unsafe { GcPtr::null() },
			locals: InlineVec::new(),
			upvalues: InlineVec::new(),
			bomb: DropBomb::new("Compiler must be `finish`ed to handle errors"),
		};

		compiler.function =
			GcPtr::new(unsafe { compiler.alloc_safe(ObjFunction::new) });
		if fn_kind != FunctionKind::Script {
			compiler.function.name = Some(unsafe {
				compiler.alloc_safe(|| {
					ObjString::new(compiler.parser().previous.text.as_ref())
				})
			});
		}

		let local_name = (fn_kind == FunctionKind::Function)
			.then_some("")
			.unwrap_or("this");
		compiler.locals.push(Local {
			name:        Token::synthetic(local_name),
			depth:       Some(0),
			is_captured: false,
		});

		compiler
	}

	fn trace_current(&self) {
		todo!()
	}
}

impl<'enclosing, 'source> Compiler<'enclosing, 'source> {
	fn parser(&self) -> &Parser<'source> {
		let mut root = &self.root;
		loop {
			match root {
				CompilerRoot::Compiler(compiler) => root = &compiler.root,
				CompilerRoot::Parser(parser) => return &parser,
			}
		}
	}

	fn parser_mut(&mut self) -> &mut Parser<'source> {
		let mut root = &mut self.root;
		loop {
			match root {
				CompilerRoot::Compiler(compiler) => root = &mut compiler.root,
				CompilerRoot::Parser(parser) => return &mut parser,
			}
		}
	}
}

impl<'enclosing, 'source> Compiler<'enclosing, 'source> {
	fn advance(&mut self) -> Token<'source> {
		self.parser_mut().advance(&mut self.errors)
	}

	fn check_eat(&mut self, kind: TokenKind) -> Option<Token<'source>> {
		self.parser_mut().check_eat(kind, &mut self.errors)
	}

	fn consume(
		&mut self,
		kind: TokenKind,
		msg: impl AsRef<str>,
	) -> Result<Token<'source>> {
		self.parser_mut().consume(kind, msg, &mut self.errors)
	}

	fn error<T>(&mut self, msg: impl AsRef<str>) -> Result<T> {
		self.errors.push(self.parser().error(msg));
		Err(())
	}

	fn error_at<T>(&mut self, tok: Token, msg: impl AsRef<str>) -> Result<T> {
		self.errors.push(parser::error_at(tok, msg));
		Err(())
	}

	fn error_at_current<T>(&mut self, msg: impl AsRef<str>) -> Result<T> {
		self.errors.push(self.parser().error_at_current(msg));
		Err(())
	}
}

impl<'enclosing, 'source> Compiler<'enclosing, 'source> {
	fn emit_byte(&mut self, byte: Bytecode) {
		let line = self.parser().previous.line;
		self.function.chunk.push(byte, line)
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

	fn emit_loop(&mut self, loop_start: usize) -> Result {
		self.emit_byte(Bytecode { op: Op::Loop });

		let offset = self.function.chunk.bytecode.len() - loop_start + 2;
		if offset > u16::MAX as _ {
			return self.error("Loop body too large.");
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
		self.emit_byte(Bytecode { op: Op::Return });
	}
}

impl<'enclosing, 'source> Compiler<'enclosing, 'source> {
	fn add_local(&mut self, name: Token<'source>) -> Result {
		if self.locals.is_full() {
			return self
				.error_at(name, "Too many local variables in function.");
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

	fn declare_variable(&mut self) -> Result {
		if self.scope_depth == 0 {
			return Ok(());
		}

		let name = self.parser().previous;
		for local in self.locals.iter() {
			if let Some(depth) = local.depth && depth < self.scope_depth {
				break;
			}

			if local.name.text == name.text {
				return self.error_at(
					name,
					format!(
						"Variable {} already defined in this scope.",
						name.text
					),
				);
			}
		}

		self.add_local(name)
	}

	fn define_variable(&mut self, ConstId(byte): ConstId) {
		if self.scope_depth > 0 {
			self.mark_initialized();
		} else {
			self.emit_bytes(
				Bytecode {
					op: Op::DefineGlobal,
				},
				Bytecode { byte },
			);
		}
	}

	fn end_scope(&mut self) {
		self.scope_depth -= 1;

		while let Some(local) = self.locals.last()
				&& let Some(depth) = local.depth
				&& depth > self.scope_depth {
			self.emit_byte(Bytecode {
				op: if local.is_captured { Op::CloseUpvalue } else { Op::Pop }
			});
			self.locals.pop();
		}
	}

	fn identifier_constant(&mut self, token: Token) -> Result<ConstId> {
		let string = unsafe { self.alloc_safe(|| ObjString::new(token.text)) };
		self.make_constant(Value::Obj(string.downcast()))
	}

	fn make_constant(&mut self, value: Value) -> Result<ConstId> {
		let Ok(const_id) = self.function.chunk.constants.len().try_into() else {
			return self.error("Too many constants in one chunk.");
		};
		self.function.chunk.constants.push(value);
		Ok(ConstId(const_id))
	}

	fn mark_initialized(&mut self) {
		if self.scope_depth > 0 {
			let local_ii = self.locals.len() - 1;
			let mut local = &mut self.locals[local_ii];
			local.depth = Some(self.scope_depth);
		}
	}

	fn resolve_local(&mut self, name: Token) -> Result<Option<usize>> {
		for (ii, local) in self.locals.iter().enumerate() {
			if local.name.text == name.text {
				if local.depth.is_none() {
					return self.error_at(
						name,
						"Can't read local variable in its own initializer.",
					);
				}

				return Ok(Some(ii));
			}
		}

		Ok(None)
	}
}

impl<'enclosing, 'source: 'enclosing> Compiler<'enclosing, 'source> {
	fn block(&mut self) -> Result<()> {
		while !matches!(
			self.parser().current.kind,
			TokenKind::RBrace | TokenKind::Eof
		) {
			self.declaration();
		}

		self.consume(TokenKind::RBrace, "Expect '}' after block.")?;
		Ok(())
	}

	fn declaration(&mut self) {
		let parse_fn = match self.parser().current.kind {
			TokenKind::Class => Self::class_declaration,
			TokenKind::Fun => Self::fun_declaration,
			TokenKind::Var => Self::var_declaration,
			_ => Self::statement,
		};
		self.advance();

		if parse_fn(self).is_err() {
			let parser = self.parser_mut();
			while parser.current.kind != TokenKind::Eof {
				if parser.previous.kind == TokenKind::Semicolon {
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

				parser.advance(&mut self.errors);
			}
		}
	}

	fn expression(&mut self) -> Result {
		self.parse_precedence(Precedence::Assignment)
	}

	fn function(&mut self, fn_kind: FunctionKind) -> Result {
		let mut compiler = self.child(fn_kind);
		compiler.begin_scope();

		if compiler
			.consume(TokenKind::LParen, "Expect '(' after function name.")
			.is_err()
		{
			compiler.bomb.defuse();
			self.errors.extend(compiler.errors);
			return Err(compiler.errors);
		}

		if !compiler.parser().check(TokenKind::RParen) {
			let params_ok = loop {
				compiler.function.arity += 1;
				if compiler.function.arity > 255 {
					compiler.error_at_current::<()>(
						"Can't have more than 255 parameters.",
					);
					break false;
				}

				let var_id =
					match compiler.parse_variable("Expect parameter name.") {
						Ok(constant) => constant,
						Err(error) => break false,
					};
				compiler.define_variable(var_id);

				if compiler.check_eat(TokenKind::Comma).is_none() {
					break true;
				}
			};
			let ok = params_ok && compiler.parser().check(TokenKind::RParen);

			if !ok {
				while !compiler.parser().check(TokenKind::RParen) {
					compiler.error_at_current::<()>(
						"Expect ')' after function parameters.",
					);
					compiler.advance();
				}
			}
		}

		compiler.advance(); // RParen

		if compiler
			.consume(TokenKind::LBrace, "Expect '{' before function body.")
			.is_err()
		{
			compiler.bomb.defuse();
			return Err(compiler.errors);
		}

		if compiler.block().is_err() {
			compiler.bomb.defuse();
			return Err(compiler.errors);
		}

		compiler.finish().map(|(function, upvalues)| {
			let ConstId(byte) = self.make_constant(function.value())?;
			self.emit_bytes(Bytecode { op: Op::Closure }, Bytecode { byte });
			for upvalue in upvalues {
				let local_byte = u8::from(upvalue.is_local);
				self.emit_bytes(Bytecode { byte: local_byte }, Bytecode {
					byte: upvalue.index,
				});
			}
			Ok(())
		})
	}

	fn method(&mut self) -> Result {
		let name = self.consume(TokenKind::Identifier, "Expect class name.")?;
		let ConstId(constant) = self.identifier_constant(name)?;

		let kind = if name.text == "init" {
			FunctionKind::Initializer
		} else {
			FunctionKind::Method
		};
		self.function(kind)
			.map_err(|errs| self.errors.extend(errs))??;

		self.emit_bytes(Bytecode { op: Op::Method }, Bytecode {
			byte: constant,
		});
		Ok(())
	}

	fn parse_variable(&mut self, error_message: &str) -> Result<ConstId> {
		let name = self.consume(TokenKind::Identifier, error_message)?;
		self.declare_variable()?;
		if self.scope_depth > 0 {
			Ok(ConstId(0))
		} else {
			self.identifier_constant(name)
		}
	}

	fn statement(&mut self) -> Result<()> {
		let parse_fn = match self.parser().current.kind {
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
