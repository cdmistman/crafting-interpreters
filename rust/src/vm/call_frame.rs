use crate::obj::ObjClosure;
use crate::obj::ObjRef;
use crate::value::Value;

pub struct CallFrame {
	pub closure: ObjRef<ObjClosure>,
	pub ip:      *const u8,
	pub slots:   *const Value,
}