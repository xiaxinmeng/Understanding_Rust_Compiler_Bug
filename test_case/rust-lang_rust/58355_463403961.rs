rust
pub trait JsSerializable {
    fn size(&self) -> u32;
    fn ser(&self, cursor: &mut Cursor<Vec>);
}

impl JsSerializable for &'static dyn Fn() -> dyn JsSerializable {...}
