 rust
struct StrHolder<'a>(&'a str);
impl<'a, 'b> Index<&'b str> for StrHolder<'a> {
    type Output = str;
    fn index(&self, _name: &str) -> &str { self.0 }
}
