rust
const TUPLE_STRUCT_MAX_INDEX: usize = 1;

impl IndexMut<usize> for TupleStruct {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index & TUPLE_STRUCT_MAX_INDEX {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("This cannot possibly happen")
        }
    }
}
