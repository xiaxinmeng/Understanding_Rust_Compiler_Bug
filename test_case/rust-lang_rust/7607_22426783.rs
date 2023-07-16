
enum Shape {
    Circle { center: Point, radius: float },
    Rectangle { top_left: Point, bottom_right: Point }
}

impl Circle {
    fn area(&self) -> float {
        self.radius * self.radius * float::consts::pi
    }
}
