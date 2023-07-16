rust
pub struct S {
    pad: usize,
    arr: [u8; 3],
}

impl S {
    pub fn f(&mut self) {
        self.arr[1] = 0;
        self.arr[2] = 0;
    }
}
