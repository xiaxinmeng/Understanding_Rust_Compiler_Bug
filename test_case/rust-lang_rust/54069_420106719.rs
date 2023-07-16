rust
#[this_is_both_bang_and_attr]
macro mac() { ... }

#[mac] // OK
struct S;

mac!(); // OK
