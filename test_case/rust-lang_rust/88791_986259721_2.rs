rust
//! /examples/world.rs

// external dependencies:
#[cfg(not(doc))]
use rand::random;

// internal dependencies:
#[cfg(any(test, doc))]
use crate::Hello;
#[cfg(not(any(test, doc)))]
use mycrate::Hello;

/// World example.
#[derive(Debug)]
pub struct World;

fn main() {
    let (h, w) = (Hello, World);
    println!("{:?} {:?} {}", h, w, random::<u8>());
}

#[cfg(test)]
mod test {
    #[test]
    fn testing_world() {
        assert![true];
    }
}
