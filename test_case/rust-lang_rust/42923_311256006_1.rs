rust
impl<'a> AsObj for &'a Trait {
    fn as_obj(&self) -> &Trait { *self }
}
