rust
pub struct S;

trait Foo {
    fn foo(&self) {}
}

pub fn a() {
    impl Foo for S {
    }
}

fn main() {
    S.foo();
}
