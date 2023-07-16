
[00:06:59] error[E0308]: mismatched types
[00:06:59]    --> librustc/traits/select.rs:457:42
[00:06:59]     |
[00:06:59] 457 |             intercrate_ambiguity_causes: Vec::new(),
[00:06:59]     |                                          ^^^^^^^^^^
[00:06:59]     |                                          |
[00:06:59]     |                                          expected enum `std::option::Option`, found struct `std::vec::Vec`
[00:06:59]     |                                          help: try using a variant of the expected type: `Some(<Vec>::new())`
[00:06:59]     |
[00:06:59]     = note: expected type `std::option::Option<std::vec::Vec<traits::select::IntercrateAmbiguityCause>>`
[00:06:59]                found type `std::vec::Vec<_>`
[00:06:59]     = help: here are some functions which might fulfill your needs:
[00:06:59]             - .pop()
[00:06:59] 
[00:07:10] error: aborting due to previous error
[00:07:10] 
[00:07:10] error: Could not compile `rustc`.
