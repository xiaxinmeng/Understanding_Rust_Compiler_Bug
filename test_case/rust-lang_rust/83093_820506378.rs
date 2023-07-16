rust
let a = Instant::now();
// some work
let b = Instant::now();
let duration = b - a; // panic: supplied instant is later than self
