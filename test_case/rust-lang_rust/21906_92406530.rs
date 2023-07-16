 rust
struct Foo (u8);

impl Foo {
    fn bar(&mut self) -> &u8 {
        self.0 += 1;
        &self.0
    }
}

fn main() {
    let mut x = Foo(42);
    let a = x.bar(); // note: borrow of `x` occurs here
    let b = x.0; // error: cannot use `x.0` because it was mutably borrowed
}
