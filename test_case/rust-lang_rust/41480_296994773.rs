rust
pub struct S;

pub fn a() {
    impl From<u8> for S {
        fn from(_: u8) -> S {
            S
        }
    }
}

fn main() {
    let s = S::from(10u8);
}
