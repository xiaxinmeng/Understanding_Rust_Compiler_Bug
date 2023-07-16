 rust
trait StrSlice {
    fn slice<'a>(&'a self, uint, uint) -> &'a str;
    ...
}
