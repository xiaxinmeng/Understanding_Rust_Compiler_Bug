rust
let x: (u32, !);
x.0 = 42;
x.1 = return x.0;
foo(&x);
