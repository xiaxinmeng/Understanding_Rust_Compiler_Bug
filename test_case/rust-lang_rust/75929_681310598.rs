rust
struct Db<'a>(&'a str);
impl<'a> Db<'a> {
    fn new(x: &'a str) -> Self {
        Self(x)
    }
}

fn foo<'a>(buf: &'a mut str) -> Db<'a> {
    Db::<'a>::new(&buf)
}
