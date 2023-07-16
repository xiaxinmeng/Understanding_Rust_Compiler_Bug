rust
trait Trait {}
const _: &dyn Trait = unsafe { std::mem::transmute((&(), &[3_i128; 2])) };

fn main() {}
