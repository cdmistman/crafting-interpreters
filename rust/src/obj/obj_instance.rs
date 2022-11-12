use super::*;

#[repr(C)]
pub struct ObjInstance {
	pub(super) obj: Obj,

	pub klass:  GcRef<ObjClass>,
	pub fields: GcMap<GcRef<ObjString>, Value>,
}

impl ObjInstance {
	pub fn new(klass: GcRef<ObjClass>) -> GcRef<Self> {
		let mut instance = GC.with(|gc| gc.new_object::<Self>());
		instance.klass = klass;
		instance.fields = GcMap::default();
		instance
	}
}

impl Display for ObjInstance {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.klass.fmt(f)?;
		" instance".fmt(f)
	}
}

impl Trace for ObjInstance {}
