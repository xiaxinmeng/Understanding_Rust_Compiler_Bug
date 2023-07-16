rust
fn generic<T: Bar>(object: T) {
    // works!
    object.foo();
}

fn concrete_to_generic(object: Qux) {
    // works!
    generic(object);
}
