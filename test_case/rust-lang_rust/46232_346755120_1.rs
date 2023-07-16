
[01:08:18] failures:
[01:08:18] 
[01:08:18] ---- primitive_docs.rs - prim_never (line 114) stdout ----
[01:08:18] 	error[E0277]: the trait bound `Self: std::marker::Sized` is not satisfied
[01:08:18]  --> primitive_docs.rs:6:5
[01:08:18]   |
[01:08:18] 6 |     fn from_str(s: &str) -> Result<Self, Self::Error>;
[01:08:18]   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Self` does not have a constant size known at compile-time
[01:08:18]   |
[01:08:18]   = help: the trait `std::marker::Sized` is not implemented for `Self`
[01:08:18]   = help: consider adding a `where Self: std::marker::Sized` bound
[01:08:18]   = note: required by `std::result::Result`
[01:08:18] 
[01:08:18] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:290:12
[01:08:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:08:18] 
[01:08:18] ---- primitive_docs.rs - prim_never (line 127) stdout ----
[01:08:18] 	error[E0599]: no function or associated item named `from_str` found for type `std::string::String` in the current scope
[01:08:18]  --> primitive_docs.rs:4:13
[01:08:18]   |
[01:08:18] 4 | let Ok(s) = String::from_str("hello");
[01:08:18]   |             ^^^^^^^^^^^^^^^^ function or associated item not found in `std::string::String`
[01:08:18]   |
[01:08:18]   = help: items from traits can only be used if the trait is in scope
[01:08:18] help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:08:18]   |
[01:08:18] 3 | use std::str::FromStr;
[01:08:18]   |
[01:08:18] 
[01:08:18] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:290:12
[01:08:18] 
[01:08:18] ---- primitive_docs.rs - prim_never (line 144) stdout ----
[01:08:18] 	error[E0433]: failed to resolve. Use of undeclared type or module `fmt`
[01:08:18]  --> primitive_docs.rs:5:35
[01:08:18]   |
[01:08:18] 5 |     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
[01:08:18]   |                                   ^^^ Use of undeclared type or module `fmt`
[01:08:18] 
[01:08:18] error[E0433]: failed to resolve. Use of undeclared type or module `fmt`
[01:08:18]  --> primitive_docs.rs:5:54
[01:08:18]   |
[01:08:18] 5 |     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
[01:08:18]   |                                                      ^^^ Use of undeclared type or module `fmt`
[01:08:18] 
[01:08:18] error[E0405]: cannot find trait `Debug` in this scope
[01:08:18]  --> primitive_docs.rs:4:6
[01:08:18]   |
[01:08:18] 4 | impl Debug for ! {
[01:08:18]   |      ^^^^^ not found in this scope
[01:08:18] help: possible candidate is found in another module, you can import it into scope
[01:08:18]   |
[01:08:18] 3 | use std::fmt::Debug;
[01:08:18]   |
[01:08:18] 
[01:08:18] error: The `!` type is experimental (see issue #35121)
[01:08:18]  --> primitive_docs.rs:4:16
[01:08:18]   |
[01:08:18] 4 | impl Debug for ! {
[01:08:18]   |                ^
[01:08:18]   |
[01:08:18]   = help: add #![feature(never_type)] to the crate attributes to enable
[01:08:18] 
[01:08:18] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:290:12
[01:08:18] 
[01:08:18] ---- primitive_docs.rs - prim_never (line 81) stdout ----
[01:08:18] 	error: The `!` type is experimental (see issue #35121)
[01:08:18]  --> primitive_docs.rs:4:8
[01:08:18]   |
[01:08:18] 4 | let x: ! = {
[01:08:18]   |        ^
[01:08:18]   |
[01:08:18]   = help: add #![feature(never_type)] to the crate attributes to enable
[01:08:18] 
[01:08:18] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:290:12
[01:08:18] 
[01:08:18] ---- primitive_docs.rs - prim_never (line 95) stdout ----
[01:08:18] 	error: expected one of `.`, `;`, `?`, or an operator, found `}`
[01:08:18]  --> primitive_docs.rs:8:1
[01:08:18]   |
[01:08:18] 7 | }
[01:08:18]   |  - expected one of `.`, `;`, `?`, or an operator here
[01:08:18] 8 | }
[01:08:18]   | ^ unexpected token
[01:08:18] 
[01:08:18] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:290:12
