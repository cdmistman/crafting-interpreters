use super::*;

#[repr(C)]
pub struct ObjBoundMethod {
	pub(super) obj: Obj,

	pub receiver: Value,
	pub method:   GcRef<ObjClosure>,
}

impl ObjBoundMethod {
	pub fn new(receiver: Value, method: GcRef<ObjClosure>) -> GcRef<Self> {
		let mut res = GC.with(|gc| gc.new_object::<Self>());
		res.receiver = receiver;
		res.method = method;
		res
	}
}

impl Display for ObjBoundMethod {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.method.function.fmt(f)
	}
}

impl Trace for ObjBoundMethod {}
