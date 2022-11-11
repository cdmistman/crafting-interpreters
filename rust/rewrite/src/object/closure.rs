use super::*;
use crate::mem::GcBox;

#[repr(C)]
pub struct ObjClosure {
	obj: Obj,

	pub function: GcObj<ObjFunction>,
	pub upvalues: GcBox<[GcObj<ObjUpvalue>]>,
}
