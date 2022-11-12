use super::*;

#[repr(C)]
pub struct ObjClass {
	pub(super) obj: Obj,

	pub name:    GcRef<ObjString>,
	pub methods: GcMap<GcRef<ObjString>, GcRef<ObjFunction>>,
}

impl ObjClass {
	pub fn new(name: GcRef<ObjString>) -> GcRef<Self> {
		let mut klass = GC.with(|gc| gc.new_object::<Self>());
		klass.name = name;
		klass
	}
}

impl Display for ObjClass {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.name.fmt(f)
	}
}

impl Trace for ObjClass {}
