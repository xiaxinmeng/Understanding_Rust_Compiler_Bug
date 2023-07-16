rust
let mut x: usize = 42;
let xref = &mut x;
let xaref = AtomicUsize::from_mut(xref);

// Send xaref around...

*xref = 53;

dbg!(xaref);
