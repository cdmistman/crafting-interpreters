use crate::value::Value;

pub union Bytecode {
	pub instr: Instr,
	pub value: Value,
	pub index: u8,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Instr {
	// raw value instructions
	Class,
	Closure,
	Constant,
	False,
	Nil,
	True,

	// stack interactions
	Call,
	Jump,
	JumpIfFalse,
	Loop,
	Pop,
	Return,

	// variables
	DefineGlobal,
	GetGlobal,
	SetGlobal,
	GetLocal,
	SetLocal,
	GetProperty,
	SetProperty,
	CloseUpvalue,
	GetUpvalue,
	SetUpvalue,

	// comparisons
	Equal,
	Greater,
	Less,

	// arithmetic operations
	Add,
	Divide,
	Multiply,
	Negate,
	Subtract,

	// boolean operations
	Not,

	// other operations
	Print,
}

pub struct Chunk {
	pub bytecode:  Vec<Bytecode>,
	pub lines:     Vec<u32>,
	pub constants: Vec<Value>,
}

impl Default for Chunk {
	fn default() -> Self {
		todo!()
	}
}
