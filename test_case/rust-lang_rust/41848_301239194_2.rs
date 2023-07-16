rust
S { field, ..base, field } // FAIL
S { field, .., field } // FAIL
fn f(arg, ..., arg); // FAIL
