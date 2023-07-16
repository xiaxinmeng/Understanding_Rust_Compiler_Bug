rust
trait Error {
    final fn type_id() -> TypeId {
        TypeId::of<Self>()
    }
}
