use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;

use fnv::FnvHashMap;

use crate::object::Obj;

pub const GC_HEAP_GROW_FACTOR: usize = 2;

pub struct Gc<'roots> {
	bytes_allocated: usize,
	next_gc:         usize,

	roots:   Vec<&'roots dyn GcRoot>,
	objects: Option<GcObj<Obj>>,
	gray:    GrayStack,
}

pub struct GrayStack(VecDeque<GcObj<Obj>>);

impl GrayStack {
	pub fn push(&mut self, obj: GcObj<Obj>) {
		self.0.push_back(obj)
	}
}

impl<'roots> Gc<'roots> {
	pub fn run(&mut self) {
		self.mark_roots();
		self.trace_references();
		self.sweep();

		self.next_gc = self.bytes_allocated * GC_HEAP_GROW_FACTOR;
	}

	fn mark_roots(&mut self) {
		for root in self.roots.iter() {
			root.mark(&mut self.gray)
		}
	}

	fn sweep(&mut self) {
		let mut previous = None;
		let mut object = self.objects.take();
		while let Some(mut obj) = object {
			if obj.is_marked {
				obj.is_marked = false;
				previous = Some(obj);
			} else {
				if let Some(mut prev) = previous {
					prev.next = obj.next.and_then(|next| next.next);
				} else {
					self.objects = obj.next;
				}
				match obj.ty {
					_ => todo!(),
				}
			}
			object = obj.next;
		}
		todo!()
	}

	fn trace_references(&mut self) {
		todo!()
	}
}

pub trait GcRoot {
	fn mark(&self, gray_stack: &mut GrayStack);
}

#[repr(transparent)]
pub struct GcObj<T = Obj>(NonNull<T>);

impl<T> Clone for GcObj<T> {
	fn clone(&self) -> Self {
		Self(self.0)
	}
}

impl<T> Copy for GcObj<T> {}

impl<T> Deref for GcObj<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { self.0.as_ref() }
	}
}

impl<T> DerefMut for GcObj<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { self.0.as_mut() }
	}
}

#[repr(transparent)]
pub struct GcBox<T: ?Sized>(NonNull<T>);

#[repr(transparent)]
pub struct GcMap<K, V>(ManuallyDrop<FnvHashMap<K, V>>);
