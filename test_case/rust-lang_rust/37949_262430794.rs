rust
match *self.0 {
    X::A(ref mut i) => { *i += 1; }
    X::B => (),
}
if let X::B = *self.0 {
    self.fail();
}
