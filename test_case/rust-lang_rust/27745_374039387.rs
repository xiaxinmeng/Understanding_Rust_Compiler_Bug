rust
trait Any {
    const fn get_type_id(&self) -> TypeId;
}
