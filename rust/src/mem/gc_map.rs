use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct GcMap<K, V, S = RandomState>(
	ManuallyDrop<MaybeUninit<Box<HashMap<K, V, S>>>>,
);

impl<K, V> GcMap<K, V, RandomState> {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<K, V, S> Clone for GcMap<K, V, S> {
	fn clone(&self) -> Self {
		Self(ManuallyDrop::new(MaybeUninit::new(unsafe {
			self.0.assume_init_read()
		})))
	}
}

impl<K, V, S: Default> Default for GcMap<K, V, S> {
	fn default() -> Self {
		Self(ManuallyDrop::new(MaybeUninit::new(Box::new(HashMap::<
			K,
			V,
			S,
		>::default()))))
	}
}

impl<K, V, S> Deref for GcMap<K, V, S> {
	type Target = HashMap<K, V, S>;

	fn deref(&self) -> &Self::Target {
		unsafe { self.0.assume_init_ref() }
	}
}

impl<K, V, S> DerefMut for GcMap<K, V, S> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { self.0.assume_init_mut() }
	}
}
