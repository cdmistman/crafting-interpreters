use super::*;
use crate::chunk::Chunk;

#[repr(C)]
pub struct ObjFunction {
	obj: Obj,

	pub arity:         usize,
	pub upvalue_count: usize,
	pub chunk:         Chunk,
	pub name:          GcObj<ObjString>,
}
