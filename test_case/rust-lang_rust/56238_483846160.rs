rust
existential type T<'a, 'b>; // = (&'a (), &'b ())
fn foo<'a, 'b>(x: &'a (), y: &'b ()) -> T<'a, 'b> { (x, y) }
