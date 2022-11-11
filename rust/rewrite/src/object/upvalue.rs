use std::ptr::NonNull;

use super::*;
use crate::value::Value;

#[repr(C)]
pub struct ObjUpvalue {
	obj: Obj,

	pub location: NonNull<Value>,
	pub closed:   Value,
	pub next:     Option<GcObj<ObjUpvalue>>,
}
