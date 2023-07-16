rust
impl<'a, T> Cow<'a, T> {
   fn shallow_copy<'b>(&'b self) -> Cow<'b, T>;
}
