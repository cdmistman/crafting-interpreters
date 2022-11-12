use super::*;

impl<'enclosing, 'source: 'enclosing> Compiler<'enclosing, 'source> {
	pub(super) fn class_declaration(&mut self) -> Result<()> {
		let class_name =
			self.consume(TokenKind::Identifier, "Expect class name.")?;
		let name_constant =
			self.identifier_constant(class_name).context("class name")?;
		self.declare_variable().context("class declaration")?;

		self.emit_bytes(Bytecode { op: Op::Class }, Bytecode {
			byte: name_constant,
		});
		self.define_variable(name_constant);

		let prev_superclass = self.superclass.take();
		self.superclass = Some(false);
		if self.check_eat(TokenKind::Less) {
			self.consume(TokenKind::Identifier, "Expect superclass name.")?;
			self.variable(false);
			if self.previous().text == class_name.text {
				return Err(eyre!("A class can't inherit from itself."));
			}

			self.begin_scope();
			self.add_local(Token::synthetic("super"))?;
			self.define_variable(0);

			self.named_variable(class_name, false);
			self.emit_byte(Bytecode { op: Op::Inherit });
			self.superclass = Some(true);
		}

		self.named_variable(class_name, false);
		self.consume(TokenKind::LBrace, "Expect '{' before class body.")?;
		while !self.check(TokenKind::RBrace) && !self.check(TokenKind::Eof) {
			if let Err(err) = self
				.method()
				.with_context(|| format!("class `{}` methods", class_name.text))
			{
				self.errors.push(err);
			}
		}
		self.consume(TokenKind::RBrace, "Expect '}' after class body.")?;

		self.emit_byte(Bytecode { op: Op::Pop });
		if self.superclass.unwrap() {
			self.end_scope();
		}
		Ok(self.superclass = prev_superclass)
	}

	pub(super) fn fun_declaration(&mut self) -> Result<()> {
		todo!()
	}

	pub(super) fn var_declaration(&mut self) -> Result<()> {
		todo!()
	}
}
