 rust
pub trait StrSlice<'a> {
...
    fn lower_chars<'a>(s: &'a str) -> LowerChars<'a>;
    fn upper_chars<'a>(s: &'a str) -> UpperChars<'a>;
}
