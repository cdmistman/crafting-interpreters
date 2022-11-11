use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;

use super::*;

pub struct GcVec<T>(ManuallyDrop<MaybeUninit<Box<Vec<T>>>>);

impl<T> Clone for GcVec<T> {
	fn clone(&self) -> Self {
		let mut boxed = MaybeUninit::uninit();
		boxed.write(unsafe { self.0.assume_init_read() });
		Self(ManuallyDrop::new(boxed))
	}
}

impl<T> Default for GcVec<T> {
	fn default() -> Self {
		Self(ManuallyDrop::new(MaybeUninit::new(Box::new(
			Vec::with_capacity(8),
		))))
	}
}

impl<T> Deref for GcVec<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		unsafe { self.0.assume_init_ref() }
	}
}

impl<T> DerefMut for GcVec<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { self.0.assume_init_mut() }
	}
}
