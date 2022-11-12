use super::*;

type NativeFn = fn(args: &[Value]) -> Value;

#[repr(C)]
pub struct ObjNative {
	pub(super) obj: Obj,

	pub function: NativeFn,
}

impl ObjNative {
	pub fn new(function: NativeFn) -> GcRef<Self> {
		let mut native = GC.with(|gc| gc.new_object::<Self>());
		native.function = function;
		native
	}
}

impl Display for ObjNative {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		"<native fn>".fmt(f)
	}
}

impl Trace for ObjNative {}
