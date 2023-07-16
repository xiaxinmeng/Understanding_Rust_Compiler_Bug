rust
trait Hasher {
    // Option 1
    fn should_add_prefix(&self) -> bool { false }
    
    // Option 2
    fn write_prefix(&mut self, prefix: usize) { self.write_usize(prefix); }
}
