rust
let x = unimplemented!();
// Move `let x` above this line to get this to compile
x.0; // E0282: cannot infer type for `_`
     // note: type must be known at this point
let x: (i32, i32) = x;
