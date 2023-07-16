 rust
impl<'a, 'b, P: Pattern<'a> + ?Sized + 'b> Pattern<'a> for &'b P {
    ...
}
