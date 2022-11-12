use std::borrow::Borrow;

use super::*;

#[repr(C)]
pub struct ObjString {
	pub(super) obj: Obj,

	pub text: &'static str,
	pub hash: usize,
}

impl ObjString {
	pub fn new(text: impl Borrow<str> + Into<String>) -> GcRef<Self> {
		GC.with(|gc| gc.intern_string(text))
	}

	pub fn concat(
		left: GcRef<ObjString>,
		right: GcRef<ObjString>,
	) -> GcRef<ObjString> {
		let mut data =
			String::with_capacity(left.text.len() + right.text.len());
		data += left.text;
		data += right.text;
		Self::new(data)
	}
}

impl Borrow<str> for GcRef<ObjString> {
	fn borrow(&self) -> &str {
		&self.text
	}
}

impl Display for ObjString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.text.fmt(f)
	}
}

impl PartialEq for ObjString {
	fn eq(&self, other: &Self) -> bool {
		self.hash == other.hash
	}
}

impl Eq for ObjString {}

impl Hash for ObjString {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_usize(self.hash);
	}
}

impl Trace for ObjString {}
