 rust
struct WeakRng(XorShiftRng);

impl Rng for WeakRng {
    pub fn next(&mut self) -> u32 {
        (*self).next()
    }
}
