rust
let x = Cell::new(42);
let y = &x;
some_function(y);
let z = x.get();
