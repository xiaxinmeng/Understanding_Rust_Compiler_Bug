 rust
fn main() {
    struct Foo<'a, 'b: 'a>(&'a Option<&'b u8>);
    let a = 0u8;
    let mut a = Some(&a);
    if let Foo::Foo(&Some(ref b)) = Foo::Foo(&a) {
        a = None;
        println!("{}, {}", a, b);
    }
}
