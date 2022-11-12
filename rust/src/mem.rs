mod gc;
mod gc_map;
mod gc_ptr;
mod gc_ref;
mod gc_vec;
mod inline_vec;

use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;

pub use self::gc::*;
pub use self::gc_map::*;
pub use self::gc_ptr::*;
pub use self::gc_ref::*;
pub use self::gc_vec::*;
pub use self::inline_vec::*;

pub trait Trace {}

macro_rules! trace_primitives {
	($($prim:ty),* $(,)?) => {
		$(
			impl Trace for $prim {
			}
		)*
	};
}

trace_primitives![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize];
