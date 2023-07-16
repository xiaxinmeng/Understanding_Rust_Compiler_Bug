 rust
struct Point;

impl Square for Point {
    fn length(&self) -> f64 { 0.0 }
}

impl Circle for Point {
    fn radius(&self) -> f64 { 0.0 }
}
