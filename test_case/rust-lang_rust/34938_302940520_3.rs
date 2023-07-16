rust
macro_rules! do_something {
    ($x:expr) => {$x + 20};
    #[cfg(windows)]
    ($i:ident, $x:expr) => {$i = $x + 20};
}
