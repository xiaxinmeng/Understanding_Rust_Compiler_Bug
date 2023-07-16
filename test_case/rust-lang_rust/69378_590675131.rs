rust
struct Point {
    pub x: u64,
    pub y: u64,
    0: u64,
}

fn main() {
    let _ = |t: Point| {
        Point {
            nonexistent: 0,
            ..t
        }
    };
}

