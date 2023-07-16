rust
impl<'a: 'b, 'b> BuildFrom<&'a str> for &'b str {
    fn from(f: &'a str) -> &'b str {
        f
    }
}
