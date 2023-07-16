rust
fn foo(a: (LogDrop, LogDrop), b: (LogDrop, LogDrop)) {
let (_x, ) = a;
let (, _y) = b;
