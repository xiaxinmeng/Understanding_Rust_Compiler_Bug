rust
macro_rules! stringify_vis {
    ($v:vis) => { stringify!($v) }
}
fn main() {
    println!("{}", stringify_vis!(pub(in self)));
}
