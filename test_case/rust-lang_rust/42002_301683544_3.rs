rust
unsafe trait UninitializedBuffer<T> {
    fn get(&mut self) -> *mut [T];
    unsafe fn did_fill(&mut self, size: usize);
}
