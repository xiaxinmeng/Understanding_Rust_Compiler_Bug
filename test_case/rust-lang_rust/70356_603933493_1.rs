rust
static X: &'static Cell<i32> = &Cell::new(42);
let y = X;
let z = y.get();
