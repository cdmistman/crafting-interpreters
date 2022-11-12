use std::fmt::Display;

use crate::mem::GcRef;
use crate::mem::Trace;
use crate::obj::Obj;
use crate::obj::ObjTy;

#[derive(Clone, Copy)]
pub struct Value(ValueKind, ValueData);

impl Value {
	#[allow(non_snake_case)]
	pub const fn Bool(bool: bool) -> Value {
		Value(ValueKind::Bool, ValueData { bool })
	}

	#[allow(non_snake_case)]
	pub const fn Nil() -> Value {
		Value(ValueKind::Nil, ValueData { nil: () })
	}

	#[allow(non_snake_case)]
	pub const fn Number(number: f64) -> Value {
		Value(ValueKind::Number, ValueData { number })
	}

	#[allow(non_snake_case)]
	pub fn Obj(obj: GcRef<Obj>) -> Value {
		Value(ValueKind::Obj, ValueData { obj })
	}
}

#[derive(Clone, Copy)]
pub enum ValueKind {
	Bool,
	Nil,
	Number,
	Obj,
}

#[derive(Clone, Copy)]
pub union ValueData {
	bool:   bool,
	nil:    (),
	number: f64,
	obj:    GcRef<Obj>,
}

impl Value {
	pub fn as_bool(&self) -> Option<bool> {
		match self {
			Value(ValueKind::Bool, data) => Some(unsafe { data.bool }),
			_ => None,
		}
	}

	pub fn as_number(&self) -> Option<f64> {
		match self {
			Value(ValueKind::Number, data) => Some(unsafe { data.number }),
			_ => None,
		}
	}

	pub fn as_obj(&self) -> Option<GcRef<Obj>> {
		let Value(ValueKind::Obj, data) = self else {
			return None;
		};
		Some(unsafe { data.obj })
	}

	pub fn as_casted_obj<Type: ObjTy>(&self) -> Option<GcRef<Type>> {
		self.as_obj()?.try_cast()
	}

	pub fn is_bool(&self) -> bool {
		matches!(self.0, ValueKind::Bool)
	}

	pub fn is_falsey(&self) -> bool {
		match self {
			Value(ValueKind::Nil, _) => true,
			Value(ValueKind::Bool, data) if unsafe { !data.bool } => true,
			_ => false,
		}
	}

	pub fn is_nil(&self) -> bool {
		matches!(self.0, ValueKind::Nil)
	}

	pub fn is_number(&self) -> bool {
		matches!(self.0, ValueKind::Number)
	}

	pub fn is_obj(&self) -> bool {
		matches!(self.0, ValueKind::Obj)
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value(ValueKind::Bool, data) => unsafe { data.bool }.fmt(f),
			Value(ValueKind::Nil, _) => "nil".fmt(f),
			Value(ValueKind::Number, data) => unsafe { data.number }.fmt(f),
			Value(ValueKind::Obj, data) => unsafe { data.obj }.fmt(f),
		}
	}
}

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Value(ValueKind::Bool, l), Value(ValueKind::Bool, r)) => unsafe {
				l.bool == r.bool
			},
			(Value(ValueKind::Nil, _), Value(ValueKind::Nil, _)) => true,
			(Value(ValueKind::Number, l), Value(ValueKind::Number, r)) => unsafe {
				l.number == r.number
			},
			(Value(ValueKind::Obj, l), Value(ValueKind::Obj, r)) => unsafe {
				// clox does a pointer equality check
				l.obj.as_ptr() == r.obj.as_ptr()
			},
			_ => false,
		}
	}
}

impl Eq for Value {}

impl Trace for Value {}
