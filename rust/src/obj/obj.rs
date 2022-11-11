use super::*;

#[repr(C)]
pub struct Obj {
	pub ty:        ObjType,
	pub next:      Option<GcRef<Obj>>,
	__noconstruct: (),
}

impl GcRef<Obj> {
	pub unsafe fn cast_unchecked<Type: ObjTy>(self) -> GcRef<Type> {
		std::mem::transmute(self)
	}

	pub fn try_cast<Type: ObjTy>(self) -> Option<GcRef<Type>> {
		if Type::OBJ_TYPE == (*self).ty {
			Some(unsafe {
				// SAFETY: we've type-checked the value through `self.ty` - as
				// long as the value was correctly initialized (that is,
				// `self.ty` is correctly set) then the memory is guaranteed to
				// be of type `Type`.
				// Guarantees enforced:
				// - `__sealed::Sealed` supertrait on `super::ObjTy` guarantees
				//   that only types in `super` are passed as the casted type
				// - `macros::value_impls!` guarantees that the `impl ObjTy`
				//   uses the correct enum value for `ObjTy::OBJ_TYPE`
				std::mem::transmute(self)
			})
		} else {
			None
		}
	}
}

impl Display for GcRef<Obj> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		unsafe {
			match self.ty {
				ObjType::BoundMethod => {
					write!(f, "{}", self.cast_unchecked::<ObjBoundMethod>())
				},
				ObjType::Class => {
					write!(f, "{}", self.cast_unchecked::<ObjClass>())
				},
				ObjType::Closure => {
					write!(f, "{}", self.cast_unchecked::<ObjClosure>())
				},
				ObjType::Function => {
					write!(f, "{}", self.cast_unchecked::<ObjFunction>())
				},
				ObjType::Instance => {
					write!(f, "{}", self.cast_unchecked::<ObjInstance>())
				},
				ObjType::Native => {
					write!(f, "{}", self.cast_unchecked::<ObjNative>())
				},
				ObjType::String => {
					write!(f, "{}", self.cast_unchecked::<ObjString>())
				},
				ObjType::Upvalue => {
					write!(f, "{}", self.cast_unchecked::<ObjUpvalue>())
				},
			}
		}
	}
}
