use crate::mem::GcMap;

use super::*;

#[repr(C)]
pub struct ObjInstance {
	obj: Obj,

	pub class: GcObj<ObjClass>,
	pub methods: GcMap<GcObj<ObjString>, GcObj<Gc>>
}
