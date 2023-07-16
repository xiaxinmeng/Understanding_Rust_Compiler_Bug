rust
use std::ops::Index;
struct StrHolder<'a>(&'a str);
impl<'a> Index<&'a str> for StrHolder<'a> {
    type Output = str;
    fn index<'b>(&'b self, _name: &str) -> &'b str { self.0 }
}

pub fn problem(s: &str) -> usize {
    let sh = StrHolder(s);
    sh["whatever"].len()
}

