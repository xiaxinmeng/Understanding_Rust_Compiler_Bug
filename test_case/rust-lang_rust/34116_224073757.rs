 rust
fn main() {
    let mut x = 33;
    let mut_ref = &mut x;
    let foo1 = Foo::new(mut_ref);
    let foo2 = Foo::new(mut_ref);
}
