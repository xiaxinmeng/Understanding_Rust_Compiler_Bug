rust
S { field, ..base, } // FAIL
S { field, .., } // FAIL
S(field, ..,) // FAIL
(field, ..,) // FAIL
[elem, elems..,] // FAIL
[elem, ..,] // FAIL
fn f(arg, ...,); // FAIL
