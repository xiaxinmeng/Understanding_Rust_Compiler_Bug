
// error: cannot apply unary operator `!` to type `core::option::Option<<generic #0>>`
// error: internal compiler error: unexpected failure
assert!(None, None);

// error: mismatched types: expected `bool` but found `<generic integer #0>`
// error: internal compiler error: unexpected failure
assert!(1, 1);

// error: cannot apply unary operator `!` to type `core::option::Option<<generic #5>>`
// but no internal compiler error
assert!(None);

// Works
assert!(true, 1i);
