 Rust
a.b.f(a.b.g(), a.b.h())
// equiv
let arg0 = a.b.g();
let arg1 = a.b.h();
a.b.f(arg0, arg1); // potentially with overloaded autoderef
