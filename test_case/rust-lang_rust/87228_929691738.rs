rust
#[macro_use]
pub mod internal {
    macro_rules! format {
        ($x:tt) => { "hell world!" }
    }
}

fn main() {
    let x = format!("hello, world!");
    assert_eq!("hell world!", x);
}
