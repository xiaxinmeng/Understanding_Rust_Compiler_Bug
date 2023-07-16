Rust
struct Thing(usize);

impl Thing {
    fn set(&mut self, val: usize) {
        self.0 = val;
    }
}

const CONST_THING: Thing = Thing(0);

fn main() {
    println!("Before: {}", CONST_THING.0);
    CONST_THING.set(1);
    println!("After: {}", CONST_THING.0);
}
