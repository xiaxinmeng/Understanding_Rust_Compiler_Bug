 rust
pub unsafe fn downgrade(&self) {
    self.unlock_write();
    self.read();
}
