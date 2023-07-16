 rust
macro_rules! m {
    () => {
        let x = 1;
        println!("{}", x); // This is OK since `x` was declared in the macro.
    }
}
fn main() {
    m!(); // this expands to `let x = 1; println!("{}", x);` and prints "1".
    x; // This is a unresolved name error, since the binding can only be used by the macro
}
