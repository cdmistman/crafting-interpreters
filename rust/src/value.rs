use crate::obj::Obj;
use crate::obj::ObjRef;

pub enum Value {
	Bool(bool),
	Nil,
	Number(f64),
	Obj(ObjRef<Obj>),
}
