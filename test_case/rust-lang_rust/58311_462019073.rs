rust
type Array = [u8; 1];

struct S<F: Fn() -> Array>(F);

impl<F: Fn() -> Array> Drop for S<F> {
    fn drop(&mut self) {}
}

fn main() {}
