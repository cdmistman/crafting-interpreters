mod call_frame;
mod run;

use std::ptr::NonNull;

use eyre::Result;
use fnv::FnvHashMap;

use self::call_frame::CallFrame;
use crate::mem::GcRef;
use crate::mem::InlineVec;
use crate::obj::*;
use crate::value::Value;

#[repr(C)]
pub struct Vm<const MAX_FRAMES: usize, const STACK_SIZE: usize> {
	frames: InlineVec<MAX_FRAMES, CallFrame>,
	stack:  InlineVec<STACK_SIZE, Value>,

	globals:       FnvHashMap<GcRef<ObjString>, Value>,
	open_upvalues: Vec<GcRef<ObjUpvalue>>,
}

impl<const MAX_FRAMES: usize, const STACK_SIZE: usize>
	Vm<MAX_FRAMES, STACK_SIZE>
{
	pub fn new() -> Self {
		let mut res = Self::default();
		res.reset();
		res
	}

	pub fn reset(&mut self) {
		self.stack.clear()
	}

	pub fn interpret(&mut self, src: &str) -> Result<()> {
		todo!()
	}

	pub fn peek(&self, index: usize) -> Value {
		todo!()
	}

	pub fn pop(&mut self) -> Value {
		todo!()
	}

	pub fn push(&mut self, value: Value) {
		todo!()
	}
}

/// Note: SHOULD call `reset` if using this impl
impl<const MAX_FRAMES: usize, const STACK_SIZE: usize> Default
	for Vm<MAX_FRAMES, STACK_SIZE>
{
	fn default() -> Self {
		// Self {
		// 	stack:     Pin::new(Box::new_zeroed_slice(STACK_SIZE)),
		// 	stack_top: NonNull::dangling(),
		// }
		todo!()
	}
}
