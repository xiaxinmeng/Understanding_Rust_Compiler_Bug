 rust
macro_rules! make_method {
     ($e: expr) => { fn method(&self) { $e } }
}
