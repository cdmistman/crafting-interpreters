use super::*;

#[repr(C)]
pub struct ObjUpvalue {
	pub(super) obj: Obj,

	pub location: NonNull<Value>,
	pub closed:   Value,
	pub next:     Option<GcRef<ObjUpvalue>>,
}

impl ObjUpvalue {
	pub fn new(slot: NonNull<Value>) -> GcRef<Self> {
		let mut upvalue = GC.with(|gc| gc.new_object::<Self>());
		upvalue.location = slot;
		upvalue.closed = Value::Nil();
		upvalue.next = None;
		upvalue
	}
}

impl Display for ObjUpvalue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		"upvalue".fmt(f)
	}
}

impl Trace for ObjUpvalue {}
