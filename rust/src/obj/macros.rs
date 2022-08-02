macro_rules! value_impls {
	($object_field:ident : $object:ty => $($name:ty,)*) => {
		paste::paste! {
		$(
			impl __sealed::ObjTy for [<Obj $name>] {
				const OBJ_TYPE: ObjType = ObjType:: $name;
			}

			impl std::ops::Deref for [<Obj $name>] {
				type Target = $object;

				fn deref(&self) -> &Self::Target {
					&self. $object_field
				}
			}
		)*}
	};
}

pub(super) use value_impls;
