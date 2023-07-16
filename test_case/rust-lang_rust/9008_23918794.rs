
impl<T:Quadrilateral> Shape for T {
    fn n_sides(&self) -> uint {
        return 4;
    }
}
