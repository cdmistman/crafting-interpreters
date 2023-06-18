use super::*;

pub enum Precedence {
	None,
	Assignment,
	Or,
	And,
	Equality,
	Comparison,
	Term,
	Factor,
	Unary,
	Call,
	Primary,
}

impl<'enclosing, 'source: 'enclosing> Compiler<'enclosing, 'source> {
	pub(super) fn parse_precedence(
		&mut self,
		precedence: Precedence,
	) -> Result<()> {
		todo!()
	}

	pub(super) fn named_variable(&mut self, name: Token, can_assign: bool) {
		todo!()
	}

	pub(super) fn variable(&mut self, can_assign: bool) {
		todo!()
	}
}
