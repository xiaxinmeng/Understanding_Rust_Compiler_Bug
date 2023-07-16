rust
pub struct Thing<T> {
    a: (u8, u8, u8),
    b: T,
}

impl<T> Thing<T> {
    pub fn new(b: T) -> Thing<T> {
        Thing { a: (0, 0, 0), b }
    }
}

fn main() {
    Thing::new(0usize);
}
