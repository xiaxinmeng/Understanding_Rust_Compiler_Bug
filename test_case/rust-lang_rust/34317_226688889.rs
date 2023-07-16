 rust
fn f(x: i32, y: i32) {
    my_macro!(x); // Because of hygiene, we know that `my_macro!` cannot access `y`.
}
