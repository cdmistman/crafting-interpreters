use super::*;

impl<'enclosing, 'source: 'enclosing> Compiler<'enclosing, 'source> {
	pub(super) fn class_declaration(&mut self) -> Result {
		let class_name =
			self.consume(TokenKind::Identifier, "Expect class name.")?;
		let name_constant = self.identifier_constant(class_name)?;
		self.declare_variable()?;

		self.emit_bytes(Bytecode { op: Op::Class }, Bytecode {
			byte: name_constant.0,
		});
		self.define_variable(name_constant);

		let prev_superclass = self.superclass.take();
		self.superclass = Some(false);
		if self.check_eat(TokenKind::Less).is_some() {
			self.consume(TokenKind::Identifier, "Expect superclass name.")?;
			self.variable(false);
			if self.parser().previous.text == class_name.text {
				return self.error("A class can't inherit from itself.");
			}

			self.begin_scope();
			self.add_local(Token::synthetic("super"))?;
			self.define_variable(ConstId(0));

			self.named_variable(class_name, false);
			self.emit_byte(Bytecode { op: Op::Inherit });
			self.superclass = Some(true);
		}

		self.named_variable(class_name, false);
		self.consume(TokenKind::LBrace, "Expect '{' before class body.")?;
		while !self.parser().check(TokenKind::RBrace)
			&& !self.parser().check(TokenKind::Eof)
		{
			self.method();
		}
		self.consume(TokenKind::RBrace, "Expect '}' after class body.")?;

		self.emit_byte(Bytecode { op: Op::Pop });
		if self.superclass.unwrap() {
			self.end_scope();
		}
		Ok(self.superclass = prev_superclass)
	}

	pub(super) fn fun_declaration(&mut self) -> Result {
		let global = self.parse_variable("Expect function name.")?;
		self.mark_initialized();
		self.function(FunctionKind::Function)?;
		self.define_variable(global);
		Ok(())
	}

	pub(super) fn var_declaration(&mut self) -> Result {
		let global = self.parse_variable("Expect variable name.")?;
		if self.check_eat(TokenKind::Equal) {
			self.expression()?;
		} else {
			self.emit_byte(Bytecode { op: Op::Nil });
		}
		self.consume(
			TokenKind::Semicolon,
			"Expect ';' after variable declaration.",
		)?;

		self.define_variable(global);
		Ok(())
	}
}
