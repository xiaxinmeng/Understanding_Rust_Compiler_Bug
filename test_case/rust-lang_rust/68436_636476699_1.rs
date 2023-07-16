rust
#![feature(const_generics)]
#![allow(incomplete_features)]
struct Arr<const N: usize>
where Assert::<{N + N}>: IsValid,
{
}

enum Assert<const CHECK: usize> {}

trait IsValid {}

impl<const CHECK: usize> IsValid for Assert<CHECK> {}

fn main() {
    let x: Arr<{usize::max_value()}> = Arr {};
}
