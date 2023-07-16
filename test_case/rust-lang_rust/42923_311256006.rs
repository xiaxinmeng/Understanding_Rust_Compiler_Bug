rust
impl<'a, Type: Trait + ?Sized> AsObj for &'a Type {
    fn as_obj(&self) -> &Trait { *self }
}
