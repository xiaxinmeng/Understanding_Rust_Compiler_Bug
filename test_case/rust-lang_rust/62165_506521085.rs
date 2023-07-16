rust
#![feature(nll, never_type)]

pub fn main() {
    loop {
        match None {
            None => return,
            Some(val) => val,
        };
    };
}
