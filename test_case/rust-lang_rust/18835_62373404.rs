 rust
#![feature(overloaded_calls)]
// Implements http://rosettacode.org/wiki/Accumulator_factory
pub struct G<T, U> {
    n: T,
}

impl<T: Add<U, T> + Clone, U> FnMut<(U,), T> for G<T, U> {
    extern "rust-call" fn call_mut(&mut self, (i,):(U,)) -> T {
        self.n = self.n + i;
        self.n.clone()
    }
}

pub fn accum<T: Add<T, U> + Clone, U>(n: T) -> G<T, U> {
G { n: n }
}

pub fn main() {
    println!("{}", accumulate());
}

fn accumulate() -> f32 {
    let mut g = accum(1f32);
    g(5.);
    accum(3i32);
    g(2.3)
}
