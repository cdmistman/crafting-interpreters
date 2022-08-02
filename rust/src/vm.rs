mod call_frame;

use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr::NonNull;

use self::call_frame::CallFrame;
use crate::obj::Obj;
use crate::obj::ObjRef;
use crate::obj::ObjString;
use crate::obj::ObjUpvalue;
use crate::value::Value;

#[repr(C)]
pub struct Vm<const MAX_FRAMES: usize, const STACK_SIZE: usize> {
	frames:      [CallFrame; MAX_FRAMES],
	frame_count: usize,

	// pin to ensure the allocation doesn't change, since `stackTop` is
	// self-referential. a lot of other pointers rely on the allocation
	// not changing as well
	stack:     Pin<Box<[MaybeUninit<u8>]>>,
	stack_top: NonNull<MaybeUninit<u8>>,

	globals: HashMap<ObjRef<ObjString>, Value>,
	strings: HashMap<ObjRef<ObjString>, Value>,

	open_upvalues: Vec<ObjRef<ObjUpvalue>>,
	objects:       Vec<ObjRef<Obj>>,
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
		let ptr = self.stack.as_mut().as_mut_ptr();
		unsafe {
			self.stack_top = NonNull::new_unchecked(ptr);
		}
	}

	pub fn interpret(&mut self, src: &str) -> Result<(), InterpretError> {
		todo!()
	}

	pub fn push(value: Value) {
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

pub enum InterpretError {
	CompileError(),
	RuntimeError,
}
