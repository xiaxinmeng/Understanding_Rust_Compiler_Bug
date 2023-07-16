 rust
#[deriving(PartialEq, PartialOrd)]
struct Foo(&'static int);
//~^ error: mismatched types: expected `&int`, found `&&'static int` (expected int, found &-ptr)
