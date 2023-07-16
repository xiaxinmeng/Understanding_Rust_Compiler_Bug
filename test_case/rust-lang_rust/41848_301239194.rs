rust
S { field, ..base } // Struct expression
S { field, .. } // Struct pattern
S(field, .., field) // Tuple struct pattern
(field, .., field) // Tuple pattern
[elem, elems.., elem] // Slice pattern
[elem, .., elem] // Slice pattern (sugar for [elem, _.., elem])
fn f(arg, ...); // Variadic foreign function
