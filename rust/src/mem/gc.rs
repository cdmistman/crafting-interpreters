use std::borrow::Borrow;
use std::cell::RefCell;

use dashmap::DashSet;

use super::GcRef;
use crate::obj::Obj;
use crate::obj::ObjString;
use crate::obj::ObjTy;

scoped_tls::scoped_thread_local!(pub static GC: GarbageCollector);

pub struct GarbageCollector {
	objects: RefCell<Option<GcRef<Obj>>>,
	strings: DashSet<GcRef<ObjString>>,
}

impl GarbageCollector {
	pub fn intern_string(
		&self,
		text: impl Borrow<str> + Into<String>,
	) -> GcRef<ObjString> {
		let as_str: &str = text.borrow();
		if let Some(interned) = self.strings.get(as_str) {
			return interned.clone();
		}

		let hash = self.strings.hash_usize(&as_str);
		let text = String::leak(text.into());

		let mut string = self.new_object::<ObjString>();
		string.hash = hash;
		string.text = text;
		string
	}

	pub fn new_object<Type: ObjTy>(&self) -> GcRef<Type> {
		let alloc =
			Box::leak(unsafe { Box::<Type>::new_zeroed().assume_init() });
		let mut res = GcRef::new_raw(alloc);

		res.ty = Type::OBJ_TYPE;
		{
			let mut objects = self.objects.borrow_mut();
			res.next = objects.take();
			*objects = Some(res.clone().downcast())
		}
		res
	}
}
