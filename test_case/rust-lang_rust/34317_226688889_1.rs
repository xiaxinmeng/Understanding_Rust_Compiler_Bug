 rust
struct S;
impl S {
    fn f(&self, x: i32) {
        my_macro!(x); // Because of hygiene, we know that `my_macro!` cannot access `self`.
    }
}
