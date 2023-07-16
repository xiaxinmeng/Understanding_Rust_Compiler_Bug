 rust
struct Foo {
    value: u32
}

impl std::ops::Deref for Foo {
    type Target = u32;
    fn deref(&self) -> &u32 { &self.value }
}

fn foo(_: &u32) {} 

fn main() {
    foo(&Foo { value: 5 });
}
