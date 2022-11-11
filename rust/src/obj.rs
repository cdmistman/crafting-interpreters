mod macros;
mod obj;
mod obj_bound_method;
mod obj_class;
mod obj_closure;
mod obj_function;
mod obj_instance;
mod obj_native;
mod obj_string;
mod obj_upvalue;

mod __sealed {
	pub trait Sealed {}
}

use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;

pub use self::obj::Obj;
pub use self::obj_bound_method::ObjBoundMethod;
pub use self::obj_class::ObjClass;
pub use self::obj_closure::ObjClosure;
pub use self::obj_function::ObjFunction;
pub use self::obj_instance::ObjInstance;
pub use self::obj_native::ObjNative;
pub use self::obj_string::ObjString;
pub use self::obj_upvalue::ObjUpvalue;
use crate::chunk::Chunk;
use crate::mem::*;
use crate::value::Value;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObjType {
	BoundMethod,
	Class,
	Closure,
	Function,
	Instance,
	Native,
	String,
	Upvalue,
}

pub trait ObjTy: Deref<Target = Obj> + DerefMut + __sealed::Sealed {
	const OBJ_TYPE: ObjType;
}

macros::value_impls!(obj: Obj =>
	BoundMethod,
	Class,
	Closure,
	Function,
	Instance,
	Native,
	String,
	Upvalue,
);

impl<Type: ObjTy> GcRef<Type> {
	pub fn downcast(self) -> GcRef<Obj> {
		unsafe {
			// SAFETY: all of the types that impl `__sealed::ObjTy` are
			// `#[repr(C)]` and have `Obj` as the first field, so it's always
			// safe to downcast
			std::mem::transmute(self)
		}
	}

	pub fn value(self) -> Value {
		Value::Obj(self.downcast())
	}
}
