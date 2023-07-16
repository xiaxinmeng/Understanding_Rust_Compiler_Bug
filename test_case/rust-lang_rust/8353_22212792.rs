 rust
use std::rand::{IsaacRng, XorShiftRng, RngUtil, Rng};

fn main() {
    let mut rng = rng();
    printfln!(rng.gen_bytes(10));
}

struct StrongRng(IsaacRng);
impl Rng for StrongRng {
    pub fn next(&mut self) -> u32 {
        return (**self).next();
    }
}

pub fn rng() -> StrongRng {
    StrongRng(IsaacRng::new())
}

struct WeakRng(XorShiftRng);
impl Rng for WeakRng {
    pub fn next(&mut self) -> u32 {
        return (**self).next();
    }
}

pub fn weak_rng() -> WeakRng {
    WeakRng(XorShiftRng::new())
}
