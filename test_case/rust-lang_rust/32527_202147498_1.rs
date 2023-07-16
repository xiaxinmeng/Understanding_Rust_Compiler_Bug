 rust
pub unsafe fn downgrade(&self) {
    self.read();
    self.unlock_write();
}
