use crate::mem::GcVec;
use crate::mem::Trace;
use crate::value::Value;

pub union Bytecode {
	pub op:    Op,
	pub value: Value,
	pub byte:  u8,
}

impl Trace for Bytecode {}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Op {
	// raw value instructions
	Closure,
	Constant,
	False,
	Nil,
	True,

	// stack interactions
	Jump,
	JumpIfFalse,
	Loop,
	Pop,
	Return,

	// call operations
	Call,
	Invoke,
	SuperInvoke,

	// variables
	CloseUpvalue,
	DefineGlobal,
	GetGlobal,
	GetLocal,
	GetProperty,
	GetSuper,
	GetUpvalue,
	SetGlobal,
	SetLocal,
	SetProperty,
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

	// class operations
	Class,
	Inherit,
	Method,

	// other operations
	Print,
}

#[derive(Default)]
pub struct Chunk {
	pub bytecode:  GcVec<Bytecode>,
	pub lines:     GcVec<u32>,
	pub constants: GcVec<Value>,
}

impl Chunk {
	pub fn push(&mut self, bytecode: Bytecode, line: u32) {
		self.bytecode.push(bytecode);
		self.lines.push(line);
	}
}

impl Trace for Chunk {}
