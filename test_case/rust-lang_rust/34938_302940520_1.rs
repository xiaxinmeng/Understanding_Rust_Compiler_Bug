rust
#[cfg(windows)]
macro_rules! do_something {
    ($x:expr) => {$x + 20}
}
#[cfg(windows)]
fn foo() {
    do_something!();
}
