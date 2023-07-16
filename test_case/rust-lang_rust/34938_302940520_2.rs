rust
macro_rules! do_something {
    ($x:expr) => {$x + 20};
    ($i:ident, $x:expr) => {$i = $x + 20};
}
fn main() {
    println!("done: {}", do_something!(2));
}
#[cfg(windows)]
fn foo() {
    let mut i = 4;
    println!("done: {}", do_something!(i, 2));
}
