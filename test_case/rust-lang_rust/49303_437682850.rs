rust
#![feature(self_struct_ctor)]
struct S([u8; weird(Self([]))]);
const fn weird(_: S) -> usize { 0 }
// This has a cycle I think
// Yeah...
// error[E0391]: cycle detected when const-evaluating + checking `S::0::{{constant}}`
