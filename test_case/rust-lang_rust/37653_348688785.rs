rust
struct S;
struct Z;
default unsafe impl Send for S {}
default impl !Send for Z {}
