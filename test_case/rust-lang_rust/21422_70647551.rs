 rust
impl <'a> PartialEq for P<'a> {
    fn eq(&self, other: &P<'a>) -> bool {
        // (self as *const _) == (other as *const _)
        (self as *const P<'a>) == (other as *const P<'a>)
    }
}
