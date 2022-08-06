use std::fmt::Display;

use crate::obj::Obj;
use crate::obj::ObjRef;
use crate::obj::ObjTy;

#[derive(Clone, Copy)]
pub enum Value {
	Bool(bool),
	Nil,
	Number(f64),
	Obj(ObjRef<Obj>),
}

impl Value {
	pub fn as_obj<Type: ObjTy>(self) -> Option<ObjRef<Type>> {
		let Value::Obj(obj) = self else {
			return None;
		};
		obj.try_cast().ok()
	}

	pub fn is_falsey(&self) -> bool {
		matches!(self, Value::Nil | Value::Bool(false))
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Bool(b) => write!(f, "{b}"),
			Self::Nil => write!(f, "nil"),
			Self::Number(float) => write!(f, "{float}"),
			Self::Obj(obj) => write!(f, "{obj}"),
		}
	}
}

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Value::Bool(this), Value::Bool(that)) => this == that,
			(Value::Nil, Value::Nil) => true,
			(Value::Number(this), Value::Number(that)) => this == that,
			(Value::Obj(this), Value::Obj(that)) => {
				// clox does a pointer equality check
				this.as_ptr() == that.as_ptr()
			},
			_ => false,
		}
	}
}

impl Eq for Value {}
