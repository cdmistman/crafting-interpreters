use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::mem::GcRef;
use crate::obj::ObjClosure;
use crate::value::Value;

pub struct CallFrame {
	pub closure: GcRef<ObjClosure>,
	pub ip:      *const u8,
	pub slots:   NonNull<[MaybeUninit<Value>]>,
}
