rs
impl IndexMut<usize> for TupleStruct {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self[index]
    }
}
