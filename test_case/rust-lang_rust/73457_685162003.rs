
error: implementation of `std::ops::FnOnce` is not general enough
   --> src/main.rs:10:5
    |
10  |       parse_header(make_it)
    |       ^^^^^^^^^^^^ implementation of `std::ops::FnOnce` is not general enough
    |
    = note: `std::ops::FnOnce<(&'0 MyType,)>` would have to be implemented for the type `for<'r> fn(&'r MyType) -> &'r MyType {make_it}`, for some specific lifetime `'0`...
    = note: ...but `std::ops::FnOnce<(&MyType,)>` is actually implemented for the type `for<'r> fn(&'r MyType) -> &'r MyType {make_it}`
