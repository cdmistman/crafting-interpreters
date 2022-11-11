use std::mem::MaybeUninit;
use std::ops::Index;
use std::ops::IndexMut;

pub struct InlineVec<const CAPACITY: usize, T> {
	len: usize,
	buf: [MaybeUninit<T>; CAPACITY],
}

impl<const CAPACITY: usize, T> InlineVec<CAPACITY, T> {
	pub fn clear(&mut self) {
		self.len = 0;
	}

	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	pub fn iter<'item, 'me: 'item>(
		&'me self,
	) -> impl Iterator<Item = &'item T> {
		(&self.buf[..self.len])
			.into_iter()
			.map(|item| unsafe { item.assume_init_ref() })
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.len == 0 {
			return None;
		}
		let slot = &self.buf[self.len];
		self.len -= 1;
		Some(unsafe { slot.assume_init_read() })
	}

	pub fn pop_n(&mut self, mut n: usize) {
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

impl<const CAPACITY: usize, T> Default for InlineVec<CAPACITY, T> {
	fn default() -> Self {
		Self {
			len: 0,
			buf: MaybeUninit::uninit_array(),
		}
	}
}

impl<const CAPACITY: usize, T> Index<usize> for InlineVec<CAPACITY, T> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		assert!(
			index < self.len,
			"index out of bounds: index is {index} but len is {}",
			self.len
		);
		let slot = &self.buf[index];
		unsafe { slot.assume_init_ref() }
	}
}

impl<const CAPACITY: usize, T> IndexMut<usize> for InlineVec<CAPACITY, T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		assert!(
			index < self.len,
			"index out of bounds: index is {index} but len is {}",
			self.len
		);
		let slot = &mut self.buf[index];
		unsafe { slot.assume_init_mut() }
	}
}
