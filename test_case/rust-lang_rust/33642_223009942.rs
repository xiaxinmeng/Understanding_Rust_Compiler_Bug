 rust
macro_rules! t {
    ($a:expr, $b:expr) => { assert_eq!($a, $b) }
}

pub fn main() {
    t!(format!("{:x}{:X}{:x}{:X}{:x}{a:X}",1,2,3,4,5,a=15), "abcdefghijkl");




    // every time this code gets pretty-printed, a newline is eaten!
}
