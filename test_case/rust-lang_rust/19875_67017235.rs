 rust
// Calling `next` on an `Iterator` never freezes it, because of the signature of `next()`:
//    `next(&mut self) -> T`  // no lifetime "glue" between `self` and `T`
let head_mut: &mut T = iter.next();
let another_head_mut: &mut T = iter.next();
