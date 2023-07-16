 rust
struct Point {
    x: float,
    y: float,
}

impl ToStr for Point {
    fn new(x: float, y: float) -> Point {
        Point { x: x, y: y }
    }

    fn to_str(&self) -> ~str {
        fmt!("(%f, %f)", self.x, self.y)
    }
}

fn main() {
}
