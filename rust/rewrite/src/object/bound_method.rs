use super::*;
use crate::value::Value;

#[repr(C)]
pub struct ObjBoundMethod {
	obj:          Obj,
	pub receiver: Value,
}

unsafe impl ObjTy for ObjBoundMethod {
	const OBJ_TYPE: ObjType = ObjType::BoundMethod;
}
