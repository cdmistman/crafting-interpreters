mod gc;
mod gc_map;
mod gc_ref;
mod gc_vec;
mod inline_vec;

use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;

pub use self::gc::*;
pub use self::gc_map::*;
pub use self::gc_ref::*;
pub use self::gc_vec::*;
pub use self::inline_vec::*;
use crate::obj::Obj;
