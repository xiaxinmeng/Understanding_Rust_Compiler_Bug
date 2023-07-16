rust
    let mut x: usize = 42;
    let xref = &mut x;
    let xaref = AtomicUsize::from_mut(xref); // Shared ref
    // Send xaref around...
    *xref = 53;    // Undefined behavior!!!
