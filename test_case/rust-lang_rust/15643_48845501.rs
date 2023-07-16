 Rust
fn with_c_fd(&self, close_after: bool, block: |libc::c_int| -> IoResult) { match block(self.fd()) { ... } }
