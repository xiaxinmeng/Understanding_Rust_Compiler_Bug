 rust
pub struct Foo {
    a: uint
}

impl Foo {
    priv fn foo(&self) {
        println(fmt!("%u", self.a));
    }
}
