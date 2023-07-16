rust
fn main() {
    type MyOption<T = i32> = Option<T>;
    // Does not compile, requires type hint
    let _ = MyOption::default();
}
