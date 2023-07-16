rust
fn do_a_thing<T>(foo: T) -> T {
    let my_fn = || foo;

    my_fn()
}
