use std::fmt::Debug;
use std::hash::Hash;

use super::*;

#[repr(transparent)]
pub struct GcRef<T = Obj>(NonNull<T>);

impl<T> GcRef<T> {
	pub fn new_raw(ptr: NonNull<T>) -> GcRef<T> {
		Self(ptr)
	}

	pub fn as_ptr(&self) -> *const T {
		self.0.as_ptr()
	}
}

impl<T> Clone for GcRef<T> {
	fn clone(&self) -> Self {
		GcRef(self.0)
	}
}

impl<T> Copy for GcRef<T> {}

impl<T> Deref for GcRef<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		// SAFETY: ObjRefs should only ever be constructed with a pointer to a
		// valid T - the validity check should have already happened, and the
		// memory should be allocated and initialized
		unsafe { self.0.as_ref() }
	}
}

impl<T> DerefMut for GcRef<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			// TODO:
			// SAFETY: Ok... so... this *technically* isn't safe. Technically
			// this exposes a race condition on values that are shared across
			// threads, which is dangerous.
			//
			// HOWEVER: this implementation of Lox is only supposed to be
			// feature-complete with the clox implementation in Crafting
			// Interpreters, which doesn't have any concept of threads or any
			// other async primitive. That means that as long as this `rlox`
			// implementation doesn't add new features, we don't have an async
			// race condition!
			self.0.as_mut()
		}
	}
}

impl<T: Display> Display for GcRef<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl<T: PartialEq> PartialEq for GcRef<T> {
	fn eq(&self, other: &Self) -> bool {
		self.0.eq(&other.0)
	}
}

impl<T: Eq> Eq for GcRef<T> {}

impl<T: Hash> Hash for GcRef<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.deref().hash(state)
	}
}
