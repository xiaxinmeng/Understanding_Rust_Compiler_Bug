 rust
macro_rules! foo {
    ( $($x:expr),* , ... ) => { 1 }
}

fn main() {
// error: local ambiguity: multiple parsing options: built-in NTs expr ('x') or 1 other options.
    let _ = foo!(1, 2, 3, ...);
//                        ^~~
}
