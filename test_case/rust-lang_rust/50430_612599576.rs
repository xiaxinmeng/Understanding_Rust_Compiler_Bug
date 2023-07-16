rust
#![deny(private_in_public)]
fn main() {
    
}

struct Inner;

mod A {
    use super::*;
    
    pub trait O {
        fn K() -> Inner;
    }
}
