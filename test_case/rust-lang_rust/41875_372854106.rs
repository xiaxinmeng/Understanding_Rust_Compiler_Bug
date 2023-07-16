rust
pub struct TypeId(u64);

impl TypeId {
    // No 'static bound, return is Option.
    pub const fn of<T: ?Sized>() -> Option<TypeId> {
        TypeId(unsafe { intrinsics::type_id::<T>().ok() })
    }
}

// No 'static bound, return is Option.
pub trait Any { fn get_type_id(&self) -> Option<TypeId>; }

// No 'static bound.
impl<T: ?Sized> Any for T {
    fn get_type_id(&self) -> Option<TypeId> { TypeId::of::<T>() }
}

impl Any {
    pub fn is<T: Any>(&self) -> bool {
        // Dynamically checks if `Self` and `T` are static and are the same type.
        match (TypeId::of::<T>(), self.get_type_id()) {
            (Some(t_id), Some(this_id)) => t_id == this_id,
            _ => false,
        }
    }
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> { /* Same as std `Any` */ }
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> { /* Same as std `Any` */ }
}
