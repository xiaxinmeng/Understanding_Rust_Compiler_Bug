rust
fn foo<'a, 'b, 'c, 'd>(x: &'a usize, y: &'b usize) -> (&'c usize, &'d usize) {
  (x, y)
}
