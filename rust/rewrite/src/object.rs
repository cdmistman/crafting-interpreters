mod bound_method;
mod class;
mod closure;
mod function;
mod instance;
mod native;
mod string;
mod upvalue;

use self::bound_method::ObjBoundMethod;
use self::class::*;
use self::closure::ObjClosure;
use self::function::ObjFunction;
use self::instance::*;
use self::native::*;
use self::string::ObjString;
use self::upvalue::*;
use crate::mem::GcObj;

pub unsafe trait ObjTy {
	const OBJ_TYPE: ObjType;
}

pub enum ObjType {
	BoundMethod,
	Class,
	Closure,
	Function,
	Instance,
	Native,
	String,
	Upvalue,
}

pub struct Obj {
	pub ty:        ObjType,
	pub is_marked: bool,
	pub next:      Option<GcObj<Obj>>,
	__noconstruct: (),
}
