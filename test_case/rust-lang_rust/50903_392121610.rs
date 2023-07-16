rust
macro_rules! no_bug {
    ($($lif2:ident ,)* #) => {};
}
fn main() {
    no_bug!(a, #);
}
