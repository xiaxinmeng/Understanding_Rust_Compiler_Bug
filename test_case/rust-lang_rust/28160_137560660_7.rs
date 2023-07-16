 Rust
a.b().c().d()
// equiv
let t0 = a.b(); // this is an rvalue
let t1 = t0.c();
let t2 = t1.d();
