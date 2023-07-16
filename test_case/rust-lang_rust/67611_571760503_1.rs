rust
static X: T = ...;
static mut Y: S = ...;
const X_PTR: &T = &X; // Imagine that this could compile
const Y_PTR: *mut T = &raw mut Y; // Imagine that this doesn't need an unsafe block
// ...
use(*X_PTR);
use(*Y_PTR);
