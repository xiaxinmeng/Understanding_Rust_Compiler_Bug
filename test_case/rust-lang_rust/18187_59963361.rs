 Rust

use std::num::{One, one};

#[cfg(not(test))]
fn main() {
    let _: Hamming<int> = Hamming::new(0);
}
/// representation of a Hamming number, allows to abstract on how the hamming number is stored
pub trait HammingNumber: Eq + Ord + Mul<Self, Self> + One {
    fn multipliers() -> Self;
}

#[cfg(compiles)]
impl HammingNumber for i64 {
    // returns the multipliers 2, 3 and 5 in the representation for the HammingNumber
    fn multipliers() -> i64 {
        0
    }
}

impl HammingNumber for int {
    // returns the multipliers 2, 3 and 5 in the representation for the HammingNumber
    fn multipliers() -> int {
        0
    }
}

pub struct Hamming<T> {
    // Using a RingBuf as a queue, push to the back, pop from the front
    q2: Vec<T>,
}

impl <T: HammingNumber> Hamming<T> {
    /// Static constructor method
    /// `n` initializes the capacity of the queues
    pub fn new(n: uint) -> Hamming<T> {
        let mut h =
            Hamming{q2: Vec::with_capacity(n),};
        h.q2.push(one());
        h
    }
    /// Pushes the next multiple of `n` (x2, x3, x5) to the queues
    pub fn enqueue(&mut self, n: &T) {
        let two : T = HammingNumber::multipliers();
        self.q2.push(*n * two);
    }
}
