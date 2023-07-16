 rust
impl fmt::Show for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "({}, {})", self.x, self.y)
    }
}
