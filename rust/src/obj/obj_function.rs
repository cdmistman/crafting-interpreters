use super::*;

#[repr(C)]
pub struct ObjFunction {
	pub(super) obj: Obj,

	pub arity:         usize,
	pub upvalue_count: usize,
	pub chunk:         Chunk,
	pub name:          Option<GcRef<ObjString>>,
}

impl ObjFunction {
	pub fn new() -> GcRef<Self> {
		let mut function = GC.with(|gc| gc.new_object::<Self>());
		function.arity = 0;
		function.upvalue_count = 0;
		function.name = None;
		function.chunk = Chunk::default();
		function
	}
}

impl Display for ObjFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.name.as_ref() {
			Some(name) => {
				"<fn ".fmt(f)?;
				name.fmt(f)?;
				">".fmt(f)
			},
			None => "<script>".fmt(f),
		}
	}
}
