 Rust
const X: uint = Y[0];
const Y: [uint * X] = [3, 2, 1];
---
error: expected constant expr for vector length: Unsupported constant expr
const Y: [uint * X] = [3, 2, 1];
         ^~~~~~~~~~
