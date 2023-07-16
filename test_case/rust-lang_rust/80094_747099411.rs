rust
struct Vector([T; N]);

impl Add<Vector> for Vector {
    fn add(self, rhs: Vector) -> Vector {
        self.0.zip(rhs.0).map(|(lhs, rhs)| lhs + rhs)
    }
}
