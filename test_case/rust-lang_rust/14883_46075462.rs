 rust
match self.try_borrow() {
    Some(_) => (*self.borrow()).fmt(f),
    None => Err(WriteError)
}
