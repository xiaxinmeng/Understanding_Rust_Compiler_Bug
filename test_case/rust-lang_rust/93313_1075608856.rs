rust
pub enum Unimplemented {}

const fn todo() -> Unimplemented {
    panic!();
}

fn f() {
    let a: u32;
    todo();
    &a; // Previously: borrow of possibly-uninitialized variable
}

fn g() {
    let b = 10;
    todo();
    b = 15; // Previously: cannot assign twice to immutable variable
}

const fn h<T>(c: T) { // Previously: destructors cannot be evaluated at compile-time
    todo();
}
