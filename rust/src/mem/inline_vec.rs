use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;

use super::Trace;

pub struct InlineVec<const CAPACITY: usize, T> {
	len: usize,
	buf: [MaybeUninit<T>; CAPACITY],
}

impl<const CAPACITY: usize, T> InlineVec<CAPACITY, T> {
	pub fn new() -> Self {
		Self {
			len: 0,
			buf: MaybeUninit::uninit_array(),
		}
	}

	pub fn clear(&mut self) {
		self.len = 0;
	}

	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	pub fn is_full(&self) -> bool {
		self.len == CAPACITY
	}

	pub fn last(&self) -> Option<&T> {
		if self.len == 0 {
			return None;
		}

		let slot = &self.buf[self.len - 1];
		Some(unsafe { slot.assume_init_ref() })
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.len == 0 {
			return None;
		}
		let slot = &self.buf[self.len];
		self.len -= 1;
		Some(unsafe { slot.assume_init_read() })
	}

	pub fn pop_n(&mut self, n: usize) {
		self.len = self.len.saturating_sub(n);
	}

	pub fn push(&mut self, value: T) {
		assert!(
			self.len <= CAPACITY,
			"overflow: InlineVec has capacity of {CAPACITY}"
		);
		let slot = &mut self.buf[self.len];
		self.len += 1;
		slot.write(value);
	}
}

impl<const CAPACITY: usize, T: Clone> Clone for InlineVec<CAPACITY, T> {
	fn clone(&self) -> Self {
		let mut res = Self::default();
		for item in self.iter().cloned() {
			res.push(item);
		}
		res
	}
}

impl<const CAPACITY: usize, T> Default for InlineVec<CAPACITY, T> {
	fn default() -> Self {
		Self {
			len: 0,
			buf: MaybeUninit::uninit_array(),
		}
	}
}

impl<const CAPACITY: usize, T> Deref for InlineVec<CAPACITY, T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		let slots = &self.buf[..self.len];
		unsafe { MaybeUninit::slice_assume_init_ref(slots) }
	}
}

impl<const CAPACITY: usize, T> DerefMut for InlineVec<CAPACITY, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		let slots = &mut self.buf[..self.len];
		unsafe { MaybeUninit::slice_assume_init_mut(slots) }
	}
}

impl<const CAPACITY: usize, T> IntoIterator for InlineVec<CAPACITY, T> {
	type IntoIter = InlineVecIterator<CAPACITY, T>;
	type Item = T;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		InlineVecIterator {
			start: 0,
			end:   self.len,
			buf:   self.buf,
		}
	}
}

impl<const CAPACITY: usize, T: Trace> Trace for InlineVec<CAPACITY, T> {}

pub struct InlineVecIterator<const CAPACITY: usize, T> {
	start: usize,
	end:   usize,
	buf:   [MaybeUninit<T>; CAPACITY],
}

impl<const CAPACITY: usize, T> Iterator for InlineVecIterator<CAPACITY, T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.start == self.end {
			return None;
		}
		let item = unsafe { self.buf[self.start].assume_init_read() };
		self.start += 1;
		Some(item)
	}
}
