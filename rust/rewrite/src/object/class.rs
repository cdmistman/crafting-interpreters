use super::*;
use crate::mem::GcMap;

#[repr(C)]
pub struct ObjClass {
	obj: Obj,

	pub name:    GcObj<ObjString>,
	pub methods: GcMap<GcObj<ObjString>, GcObj<ObjFunction>>,
}
