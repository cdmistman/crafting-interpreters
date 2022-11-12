use std::mem::MaybeUninit;

use super::*;

pub struct GcVec<T: Trace> {
	vec:    NonNull<Vec<T>>,
	gc_tmp: NonNull<Option<T>>,
}

impl<T: Trace> GcVec<T> {
	pub fn push(&mut self, t: T) {
		let t = MaybeUninit::new(t);
		unsafe {
			*self.gc_tmp.as_mut() = Some(t.assume_init_read());
			self.vec.as_mut().push(t.assume_init_read());
			self.gc_tmp.as_mut().take();
		}
	}
}

impl<T: Trace> Clone for GcVec<T> {
	fn clone(&self) -> Self {
		Self {
			vec:    self.vec,
			gc_tmp: self.gc_tmp,
		}
	}
}

impl<T: Trace> Default for GcVec<T> {
	fn default() -> Self {
		let vec = Box::leak(Box::new(Vec::new())).into();
		let gc_tmp = Box::leak(Box::new(None)).into();
		Self { vec, gc_tmp }
	}
}

impl<T: Trace> Deref for GcVec<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		unsafe { self.vec.as_ref() }
	}
}

impl<T: Trace> DerefMut for GcVec<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { self.vec.as_mut() }
	}
}

impl<T: Trace> Trace for GcVec<T> {}
