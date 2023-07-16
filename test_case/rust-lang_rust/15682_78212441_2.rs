 rust
fn bar(&self) {
     struct Inner;
     impl Inner {
         make_method!(self.qux()) // meant to be the outer `self`
     }
}
