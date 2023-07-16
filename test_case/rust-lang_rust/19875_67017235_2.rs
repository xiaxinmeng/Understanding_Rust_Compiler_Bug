 rust
let head_mut: &mut T = iter.next();  // `iter` gets frozen here
let another_head_mut: &mut T = iter.next();  //~ error: `iter` is already mutably borrowed
