 rust
enum E { V1(isize), V0 }
const C: &'static [E] = &[E::V0, E::V1(0xDEADBEE)];
static C1: E = C[1];
static C0: E = C[0];
