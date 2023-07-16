 rust
pub fn weak_rng() -> WeakRng { ... }

pub struct WeakRng(XorShiftRng);

impl Rng for WeakRng { ... }
