
struct Circle { center: Point, radius: float }
struct Rectangle { top_left: Point, bottom_right: Point }

enum Shape {
    Circle,
    Rectangle
}

impl Circle {
    fn area(&self) -> float {
        self.radius * self.radius * float::consts::pi
    }
}
