rust
trait Any {
    const TYPE_ID: TypeId = TypeId::of::<Self>();
}
