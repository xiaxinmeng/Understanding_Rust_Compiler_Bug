rust
pub unsafe trait ErrorTypeId {
    fn type_id(&self) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }
}

unsafe impl<T: ?Sized> ErrorTypeId for T {}

pub trait Error: Debug + Display + ErrorTypeId {
   // ...
}
