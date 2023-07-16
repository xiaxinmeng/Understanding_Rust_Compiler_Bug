console
error: implementation of `std::iter::Iterator` is not general enough
    --> src/main.rs:14:31
     |
14   |   fn foo<'a>(t: &'a [Thing]) -> impl Send + 'a {
     |                                 ^^^^^^^^^^^^^^ implementation of `std::iter::Iterator` is not general enough
     |
     = note: `std::iter::Iterator` would have to be implemented for the type `std::slice::Iter<'0, Thing>`, for any lifetime `'0`...
     = note: ...but `std::iter::Iterator` is actually implemented for the type `std::slice::Iter<'1, Thing>`, for some specific lifetime `'1`
