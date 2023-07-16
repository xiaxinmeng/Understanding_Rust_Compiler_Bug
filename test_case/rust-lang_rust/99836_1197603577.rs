rust
fn main() {
    enum Error { A, B }

    type Foo = Result<[usize; 0], Error>;

    dbg!(std::mem::size_of::<Foo>());
    dbg!(std::mem::align_of::<Foo>());
}
