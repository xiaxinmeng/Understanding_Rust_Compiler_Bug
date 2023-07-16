 rust
trait StrSlice<'a> {
    fn slice(&self, uint, uint) -> &'a str;
    ...
}
