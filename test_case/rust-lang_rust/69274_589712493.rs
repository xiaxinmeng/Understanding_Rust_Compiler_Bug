Rust
#![feature(target_feature_11)]
trait Foo {
    fn sse2(&self);
}

struct Bar;

impl Foo for Bar {
    #[target_feature(enable = "sse2")]
    fn sse2(&self) {}
}

fn foo<T: Foo>(t: &T) {
    <T as Foo>::sse2(t);
}

fn main() {
    foo(&Bar); // no `unsafe`, and no guarantee `sse2` is available
}
