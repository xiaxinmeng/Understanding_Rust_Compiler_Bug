rust
#![allow(dead_code)]

struct X;

impl X {
    const fn do_nothing(self) -> Self {
        self
    }
}

// Compiles just fine
fn works() -> &'static X {
    &X
}

// Fails to compile, saying that `X.do_nothing()` creates a temporary value owned by the function
fn doesnt_work() -> &'static X {
    &X.do_nothing()
}

// This on the other hand works
fn works_again() -> &'static X {
    const x: &X = &X.do_nothing();
    x
}
