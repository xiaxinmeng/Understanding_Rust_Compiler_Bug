
let b = box ...;
let x = &mut *box;
let y = *x; // move out of *x
*x = ...; // replace
