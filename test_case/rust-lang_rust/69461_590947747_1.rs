rust
let (a, ref mut b) = &mut (1, 2);
*b = *a;
