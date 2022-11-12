use super::*;

pub struct GcPtr<T: Trace>(Option<GcRef<T>>);

impl<T: Trace> GcPtr<T> {
	pub fn new(raw: GcRef<T>) -> Self {
		Self(Some(raw))
	}

	pub unsafe fn null() -> Self {
		Self(None)
	}
}

impl<T: Trace> Deref for GcPtr<T> {
	type Target = GcRef<T>;

	fn deref(&self) -> &Self::Target {
		match self.0.as_ref() {
			Some(res) => res,
			None => unreachable!("null GC pointer"),
		}
	}
}

impl<T: Trace> DerefMut for GcPtr<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self.0.as_mut() {
			Some(res) => res,
			None => unreachable!("null GC pointer"),
		}
	}
}

impl<T: Trace> Into<GcRef<T>> for GcPtr<T> {
	fn into(self) -> GcRef<T> {
		match self.0 {
			Some(res) => res,
			None => unreachable!("null GC pointer"),
		}
	}
}
