plain
    Checking rustc-demangle v0.1.21
error[E0433]: failed to resolve: use of undeclared crate or module `std`
  --> library/alloc/src/str.rs:98:10
   |
98 | where S: std::fmt::Display {
   |          ^^^ use of undeclared crate or module `std`
help: there is a crate or module with a similar name
   |
   |
98 | where S: str::fmt::Display {

error[E0412]: cannot find type `Str` in this scope
  --> library/alloc/src/str.rs:97:21
   |
   |
97 | impl<S> Join<Borrow<Str>> for [S] 
   |
help: a builtin type with a similar name exists
   |
   |
97 | impl<S> Join<Borrow<str>> for [S] 
help: you might be missing a type parameter
   |
   |
97 | impl<S, Str> Join<Borrow<Str>> for [S] 


error[E0782]: trait objects must include the `dyn` keyword
   |
   |
97 | impl<S> Join<Borrow<Str>> for [S] 
   |
   |
help: add `dyn` keyword before this trait
   |
97 | impl<S> Join<dyn Borrow<Str>> for [S] 

Some errors have detailed explanations: E0412, E0433, E0782.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `alloc` due to 3 previous errors
