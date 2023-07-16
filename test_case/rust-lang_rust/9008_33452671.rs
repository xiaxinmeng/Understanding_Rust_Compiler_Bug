 rust
trait Shape {
    fn n_sides(&self) -> uint;
}

trait Quadrilateral : Shape {
    fn sides(&self) -> ~[f32, ..4];
}

impl<T:Quadrilateral> Shape for T {
    fn n_sides(&self) -> uint {
        return 4;
    }
}

trait Rectangle : Quadrilateral {
    //only rectangles have equal-sided diagonals
    fn diagonal(&self) -> f32 {
        let sides = self.sides();
        return (sides[0] * sides[0] + sides[1] * sides[1]).sqrt();
    }
}

struct SquareImpl {
    side : f32
}

impl Quadrilateral for SquareImpl {
    fn sides(&self) -> ~[f32, ..4] {
        let sides : ~[f32, ..4] = ~([self.side, self.side, self.side, self.side]);
        return sides;
    }
}

impl Rectangle for SquareImpl {
    //Rectangle's diagonal is implemented by Rectangle
}

fn main() {}
