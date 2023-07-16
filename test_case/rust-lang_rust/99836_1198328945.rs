rust
fn main() {
    type Foo = Result<[usize; 0], bool>;

    println!(
        "size:{} align:{}",
        std::mem::size_of::<Foo>(),
        std::mem::align_of::<Foo>()
    );
}
