rust
unsafe trait TypeInfo {
  fn type_id(&self) -> TypeId;
}

impl<T: ?Sized> TypeInfo for T {
  fn type_id(&self) -> TypeId {
    TypeId::of::<Self>()
  }
}

trait Error : TypeInfo {}
