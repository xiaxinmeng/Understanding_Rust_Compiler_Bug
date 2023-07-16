 rust
trait example_trait {fn stuff(&self);}

impl example_trait for u64 {
    fn stuff(&self) {}
}

fn main() {
    let x = 0u64;
    let ptr = (&x as *const example_trait);
}
