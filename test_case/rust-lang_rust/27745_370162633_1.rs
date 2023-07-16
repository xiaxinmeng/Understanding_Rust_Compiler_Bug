rust
trait Any {}
impl Any {
    const TYPE_ID: TypeId = TypeId::of::<Self>();
}
