 rust
impl<'a> ObjRef<'a> {
    pub fn get_obj<'b>(&'b mut self) -> &'b int {
        self.r
    }
}
