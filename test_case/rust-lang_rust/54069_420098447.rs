rust
#[this_is_attr]
macro mmm() { ... }

#[mmm] // OK
struct S;
