use super::*;

#[repr(C)]
pub struct ObjClosure {
	pub(super) obj: Obj,

	pub function: GcRef<ObjFunction>,
	pub upvalues: GcVec<GcRef<ObjUpvalue>>,
}

impl ObjClosure {
	pub fn new(function: GcRef<ObjFunction>) -> GcRef<Self> {
		let mut closure = GC.with(|gc| gc.new_object::<Self>());
		closure.function = function;
		closure.upvalues = GcVec::default();
		closure
	}
}

impl Display for ObjClosure {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.function.fmt(f)
	}
}
