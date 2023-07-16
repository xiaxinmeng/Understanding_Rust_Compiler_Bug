rs
impl IndexMut<usize> for TupleStruct {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.0
        }
        else if index == 1 {
            &mut self.1
        }
        else {
            panic!("index out of range")
        }
    }
}
