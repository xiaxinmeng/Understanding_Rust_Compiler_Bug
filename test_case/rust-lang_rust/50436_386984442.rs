rust
impl Alloc {
    pub fn alloc_one<T>(&mut self) -> Result<Unique<T>, AllocErr> { ... }
}
