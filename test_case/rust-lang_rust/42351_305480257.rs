rust
trait Foo {
    fn foo(&self);
}

impl Foo for u8 {
    fn foo(&self) {}
}

fn main() {
    let foo_foo_explicit = <Foo as Foo>::foo;
    let foo_foo_shortcut = <Foo>::foo;
    let foo_u8 = <u8 as Foo>::foo;
    println!("foo_foo_explicit {:?}", foo_foo_explicit as usize);
    println!("foo_foo_shortcut {:?}", foo_foo_shortcut as usize);
    println!("foo_u8 {:?}", foo_u8 as usize);
}
