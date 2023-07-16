rust
struct Foo<'a>(&'a mut usize);
impl Iterator for Foo<'_> {
    type Item = ();
    fn next(&mut self) -> Option<()> {
        *self.0 += 1;
        None
    }
}

fn main() {
    let mut counter = 0;
    let mut foo = Foo(&mut counter).fuse();
    for _ in 0..10 {
        foo.next();
    }
    dbg!(counter);
}
