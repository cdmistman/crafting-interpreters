use crate::mem::GcVec;
use crate::value::Value;

pub union Bytecode {
	pub instr: Op,
	pub value: Value,
	pub index: u8,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Op {
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

#[derive(Default)]
pub struct Chunk {
	pub bytecode:  GcVec<Bytecode>,
	pub lines:     GcVec<u32>,
	pub constants: GcVec<Value>,
}
