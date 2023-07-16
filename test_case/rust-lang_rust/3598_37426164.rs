 rust
fn shorthen<'a, 'b>(x: &'b Option<&'a int>) -> Option<&'b int> {*x}
