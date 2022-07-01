use std::mem::MaybeUninit;

use crate::chunk::Chunk;
use crate::value::Value;

#[repr(C)]
pub struct Vm<const STACK_SIZE: usize> {
	chunk:     Vec<Chunk>,
	stack:     [u8; STACK_SIZE],
	stack_top: MaybeUninit<*mut u8>,
}

impl<const STACK_SIZE: usize> Vm<STACK_SIZE> {
	pub fn new() -> Self {
		let mut res = Self::default();
		res.reset();
		res
	}

	pub fn reset(&mut self) {
		self.stack_top.write(self.stack.as_mut().as_mut_ptr());
	}

	pub fn interpret(&mut self, src: &str) -> Result<(), InterpretError> {
		todo!()
	}

	pub fn push(value: Value) {
		todo!()
	}
}

/// Note: SHOULD call `reset` if using this impl
impl<const STACK_SIZE: usize> Default for Vm<STACK_SIZE> {
	fn default() -> Self {
		Self {
			chunk: Vec::new(),
			stack: unsafe { MaybeUninit::uninit().assume_init() },
			stack_top: MaybeUninit::uninit(),
		}
	}
}

pub enum InterpretError {
	CompileError(),
	RuntimeError,
}
