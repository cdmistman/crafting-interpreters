use std::fmt::Display;

mod macros;

mod __sealed {
	pub trait ObjTy {
		const OBJ_TYPE: super::ObjType;
	}
}

use std::collections::HashMap;
use std::hash::Hash;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;

use crate::chunk::Chunk;
use crate::value::Value;

pub trait ObjTy = __sealed::ObjTy;

#[repr(C)]
pub struct Obj {
	pub ty:        ObjType,
	pub next:      ObjRef<Obj>,
	__noconstruct: (),
}

#[derive(PartialEq, Eq)]
pub enum ObjType {
	Class,
	Closure,
	Function,
	Instance,
	Native,
	String,
	Upvalue,
}

#[repr(transparent)]
pub struct ObjRef<T>(NonNull<T>);

#[repr(C)]
pub struct ObjClass {
	obj:      Obj,
	pub name: ObjRef<ObjString>,
}

#[repr(C)]
pub struct ObjClosure {
	obj:          Obj,
	pub function: ObjRef<ObjFunction>,
	// TODO: shouldn't be a `Vec`, should be some `GcVec` value
	pub upvalues: ManuallyDrop<Vec<ObjRef<ObjUpvalue>>>,
}

#[repr(C)]
pub struct ObjFunction {
	obj:               Obj,
	pub arity:         usize,
	pub upvalue_count: usize,
	pub chunk:         Chunk,
	pub name:          Option<ObjRef<ObjString>>,
}

#[repr(C)]
pub struct ObjInstance {
	obj:        Obj,
	pub klass:  ObjRef<ObjClass>,
	pub fields: HashMap<ObjRef<ObjString>, Value>,
}

#[repr(C)]
pub struct ObjNative {
	obj:          Obj,
	pub function: fn(args: &[Value]) -> Value,
}

#[repr(C)]
pub struct ObjString {
	obj:       Obj,
	pub len:   usize,
	pub chars: NonNull<u8>,
	pub hash:  u32,
}

#[repr(C)]
pub struct ObjUpvalue {
	obj:          Obj,
	pub location: NonNull<Value>,
	pub closed:   Value,
	pub next:     ObjRef<ObjUpvalue>,
}

macros::value_impls!(obj: Obj =>
	Class,
	Closure,
	Function,
	Instance,
	Native,
	String,
	Upvalue,
);

impl<T> Clone for ObjRef<T> {
	fn clone(&self) -> Self {
		todo!()
	}
}

impl<T> Copy for ObjRef<T> {}

impl<T> Deref for ObjRef<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		// SAFETY: ObjRefs should only ever be constructed with a pointer to a
		// valid T - the validity check should have already happened, and the
		// memory should be allocated and initialized
		unsafe { self.0.as_ref() }
	}
}

impl<T: __sealed::ObjTy> DerefMut for ObjRef<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			// TODO:
			// SAFETY: Ok... so... this *technically* isn't safe. Technically
			// this exposes a race condition on values that are shared across
			// threads, which is dangerous.
			//
			// HOWEVER: this implementation of Lox is only supposed to be
			// feature-complete with the clox implementation in Crafting
			// Interpreters, which doesn't have any concept of threads or any
			// other async primitive. That means that as long as this `rlox`
			// implementation doesn't add new features, we don't have an async
			// race condition!
			//
			// Are there still situations where this escape hatch is dangerous?
			// Sure. Imagine:
			// ```rs
			// fn print_obj_string(string: ObjRef<ObjString>) {
			// 	 let
			// }
			self.0.as_mut()
		}
	}
}

impl Display for ObjRef<Obj> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		unsafe {
			match self.0.as_ref().ty {
				ObjType::Class => {
					write!(f, "{}", self.cast_unchecked::<ObjClass>())
				},
				_ => todo!(),
			}
		}
	}
}

impl<Type: __sealed::ObjTy + Display> Display for ObjRef<Type> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", *self)
	}
}

impl<Type: __sealed::ObjTy + Hash> Hash for ObjRef<Type> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		(*self).hash(state)
	}
}

impl<Type: __sealed::ObjTy + PartialEq> PartialEq for ObjRef<Type> {
	fn eq(&self, other: &Self) -> bool {
		(*self).eq(other)
	}
}

impl<Type: __sealed::ObjTy + Eq> Eq for ObjRef<Type> {}

impl<T> ObjRef<T> {
	pub fn as_ptr(&self) -> *const T {
		self.0.as_ptr()
	}
}

impl ObjRef<Obj> {
	pub unsafe fn cast_unchecked<Type: __sealed::ObjTy>(self) -> ObjRef<Type> {
		std::mem::transmute(self)
	}

	pub fn try_cast<Type: __sealed::ObjTy>(self) -> Result<ObjRef<Type>, Self> {
		if Type::OBJ_TYPE == self.ty {
			Ok(unsafe {
				// SAFETY: we've type-checked the value through `self.ty` - as
				// long as the value was correctly initialized (that is,
				// `self.ty` is correctly set) then the memory is guaranteed to
				// be of type `Type`.
				// Guarantees enforced:
				// - `__sealed::ObjTy` guarantees that only types in this mod
				//   are passed as the casted type
				// - `macros::value_impls!` guarantees that the `impl
				//   __sealed::ObjTy` uses the correct enum value for
				//   `__sealed::ObjTy::OBJ_TYPE`
				std::mem::transmute(self)
			})
		} else {
			Err(self)
		}
	}
}

impl<Type: __sealed::ObjTy> ObjRef<Type> {
	pub fn downcast(self) -> ObjRef<Obj> {
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

impl Display for ObjClass {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

impl Display for ObjClosure {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.function)
	}
}

impl ObjClosure {
	pub fn new(function: ObjRef<ObjFunction>) -> ObjRef<ObjClosure> {
		todo!()
	}
}

impl Display for ObjFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.name {
			Some(name) => write!(f, "<fn {name}>"),
			None => write!(f, "<script>"),
		}
	}
}

impl Display for ObjInstance {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} instance", self.klass)
	}
}

impl Display for ObjNative {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "<native fn>")
	}
}

impl Display for ObjString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", unsafe {
			let bytes = NonNull::slice_from_raw_parts(self.chars, self.len);
			std::str::from_utf8_unchecked(bytes.as_ref())
		})
	}
}

impl ObjString {
	pub fn concat(
		left: ObjRef<ObjString>,
		right: ObjRef<ObjString>,
	) -> ObjRef<ObjString> {
		todo!()
	}

	pub fn take(buf: *const u8, len: usize) -> ObjRef<ObjString> {
		todo!()
	}
}

impl PartialEq for ObjString {
	fn eq(&self, other: &Self) -> bool {
		self.hash == other.hash
	}
}

impl Eq for ObjString {}

impl Display for ObjUpvalue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "upvalue")
	}
}

impl Hash for ObjString {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_u32(self.hash);
	}
}
