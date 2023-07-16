 rust
impl <T: Add<T> + Copy> Add for DenseMatrix<T> {
    type Output = DenseMatrix<T>;  // <-- Add this

    fn add(self, rhs: DenseMatrix<T>) -> DenseMatrix<T> {
        self.__add(rhs)
    }
}
