rust
pub struct Cache {
    data: Vec<i32>,
}

impl Cache {
    pub fn list_data(&mut self, key: &usize) {
        for reference in vec![1, 2, 3] {
            if /*let*/ Some(reference) = self.data.get(*key) {
                unimplemented!()
            }
        }
    }
}
