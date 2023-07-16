rust
#[repr(transparent)]
struct Rtemp<const N: usize>([usize; N]);
type R = Rtemp<0>;
