 rust
fn main() {
    struct Foo<'a, 'b: 'a> { data: &'a Option<&'b u8> };
    let a = 0u8;
    let mut a = Some(&a);
    if let Foo { data: &Some(ref b) } = (Foo { data: &a }) {
        a = None;
        println!("{}, {}", a, b);
    }
}
