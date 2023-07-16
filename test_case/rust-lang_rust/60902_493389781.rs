rust
impl dyn Error + 'static {
    fn type_id(&self) -> TypeId {
        self.__type_id(Internal)
    }
}
