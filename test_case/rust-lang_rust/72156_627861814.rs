rust
fn test<T: Sync>(_: T) {}

fn ok() -> impl Eq {
    7
}

fn not_ok() -> impl Eq {
    std::cell::Cell::new(7)
}

fn main() {
    test(ok());
    // This works, `ok` returns a type implementing `Sync`.
    //
    // Note that we do not explicitly mention `Send` in the return type of `ok`.
    
    test(not_ok());
    // This fails, `not_ok` returns a type not implementing `Sync`.
}
