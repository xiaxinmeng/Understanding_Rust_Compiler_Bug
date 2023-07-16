 Rust
struct S<'a>(&'a ());

impl<'a> S<'a> {
    fn foo(self) {
        <Self>::foo(self);
    }
}

fn main() {}
