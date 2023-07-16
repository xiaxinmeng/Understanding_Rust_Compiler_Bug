
[01:20:19]    |     ^
[01:20:19] 
[01:20:19] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::signum (line 209)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:20:19] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::signum (line 209)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:20:19] 
[01:20:19] ---- num/mod.rs - num::wrapping::Wrapping<i32>::pow (line 214) stdout ----
[01:20:19]  error[E0425]: cannot find function `Wrapping` in this scope
[01:20:19]  --> num/mod.rs:216:12
[01:20:19]   |
[01:20:19] 5 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:20:19]   |            ^^^^^^^^ not found in this scope
[01:20:19] help: possible candidate is found in another module, you can import it into scope
[01:20:19] 4 | use std::num::Wrapping;
[01:20:19]   |
[01:20:19] 
[01:20:19] 
[01:20:19] error[E0425]: cannot find function `Wrapping` in this scope
[01:20:19]  --> num/mod.rs:217:12
[01:20:19]   |
[01:20:19] 6 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:20:19]   |            ^^^^^^^^ not found in this scope
[01:20:19] help: possible candidate is found in another module, you can import it into scope
[01:20:19] 4 | use std::num::Wrapping;
[01:20:19]   |
[01:20:19] 
[01:20:19] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:20:19] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:20:19] 
[01:20:19] ---- num/mod.rs - num::wrapping::Wrapping<i32>::to_le (line 208) stdout ----
[01:20:19]  error: this file contains an un-closed delimiter
[01:20:19]   --> num/mod.rs:219:2
[01:20:19] 14 | }
[01:20:19]    |  ^
[01:20:19]    |
[01:20:19] help: did you mean to close this delimiter?
[01:20:19] help: did you mean to close this delimiter?
[01:20:19]   --> num/mod.rs:208:3
[01:20:19]    |
[01:20:19] 3  | #![feature(wrapping_int_impl)
[01:20:19]    |   ^
[01:20:19] 
[01:20:19] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:20:19]   --> num/mod.rs:208:1
[01:20:19]    |
[01:20:19] 3  | / #![feature(wrapping_int_impl)
[01:20:19] 4  | | fn main() {
[01:20:19] 5  | | use std::num::Wrapping;
[01:20:19] ...  |
[01:20:19] 13 | | }
[01:20:19] 14 | | }
[01:20:19]    | |_^
[01:20:19]    | |_^
[01:20:19] 
[01:20:19] error: unexpected token: `fn`
[01:20:19]  --> num/mod.rs:209:1
[01:20:19]   |
[01:20:19] 3 | #![feature(wrapping_int_impl)
[01:20:19]   |                             - unexpected token after this
[01:20:19] 4 | fn main() {
[01:20:19]   | ^^ unexpected token
[01:20:19] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:20:19] 
[01:20:19] ---- num/mod.rs - num::wrapping::Wrapping<i64>::from_le (line 208) stdout ----
[01:20:19] ---- num/mod.rs - num::wrapping::Wrapping<i64>::from_le (line 208) stdout ----
[01:20:19]  error: expected one of `,`, `.`, `?`, or an operator, found `$`
[01:20:19]  --> num/mod.rs:212:22
[01:20:19]   |
[01:20:19] 7 | let n = Wrapping(0x1A$ SelfT);
[01:20:19]   |                      ^ expected one of `,`, `.`, `?`, or an operator here
[01:20:19] 
[01:20:19] error: expected one of `.`, `;`, `<`, `?`, `break`, `continue`, `false`, `for`, `if`, `loop`, `match`, `move`, `return`, `static`, `true`, `unsafe`, `while`, `yield`, or an operator, found `$`
[01:20:19]  --> num/mod.rs:212:22
[01:20:19]   |
[01:20:19] 7 | let n = Wrapping(0x1A$ SelfT);
[01:20:19]   |                      ^ expected one of 19 possible tokens here
[01:20:19] error: unused import: `std::num::Wrapping`
[01:20:19]  --> num/mod.rs:210:5
[01:20:19]   |
[01:20:19] 5 | use std::num::Wrapping;
[01:20:19] 5 | use std::num::Wrapping;
[01:20:19]   |     ^^^^^^^^^^^^^^^^^^
[01:20:19]   |
[01:20:19] note: lint level defined here
[01:20:19]  --> num/mod.rs:206:9
[01:20:19]   |
[01:20:19] 1 | #![deny(warnings)]
[01:20:19]   |         ^^^^^^^^
[01:20:19]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:20:19] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::from_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:20:19] 
[01:20:19] ---- num/mod.rs - num::wrapping::Wrapping<i64>::from_be (line 208) stdout ----
[01:20:19] ---- num/mod.rs - num::wrapping::Wrapping<i64>::from_be (line 208) stdout ----
[01:20:19]  error[E0034]: multiple applicable items in scope
[01:20:19]   --> num/mod.rs:215:16
[01:20:19]    |
[01:20:19] 10 |     assert_eq!(Wrapping::from_be(n), n)
[01:20:19]    |                ^^^^^^^^^^^^^^^^^ multiple `from_be` found
[01:20:19]    |
[01:20:19]    = note: candidate #1 is defined in an impl for the type `std::num::Wrapping<usize>`
[01:20:19]    = note: candidate #2 is defined in an impl for the type `std::num::Wrapping<u8>`
[01:20:19]    = note: candidate #3 is defined in an impl for the type `std::num::Wrapping<u16>`
[01:20:19]    = note: candidate #4 is defined in an impl for the type `std::num::Wrapping<u32>`
[01:20:19]    = note: and 8 others
[01:20:19] 
[01:20:19] error[E0034]: multiple applicable items in scope
[01:20:19]   --> num/mod.rs:217:16
[01:20:19]    |
[01:20:19] 12 |     assert_eq!(Wrapping::from_be(n), n.swap_bytes())
[01:20:19]    |                ^^^^^^^^^^^^^^^^^ multiple `from_be` found
[01:20:19]    |
[01:20:19]    = note: candidate #1 is defined in an impl for the type `std::num::Wrapping<usize>`
[01:20:19]    = note: candidate #2 is defined in an impl for the type `std::num::Wrapping<u8>`
[01:20:19]    = note: candidate #3 is defined in an impl for the type `std::num::Wrapping<u16>`
[01:20:19]    = note: candidate #4 is defined in an impl for the type `std::num::Wrapping<u32>`
[01:20:19]    = note: and 8 others
[01:20:19] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::from_be (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:20:19] 
[01:20:19] ---- num/mod.rs - num::wrapping::Wrapping<i64>::count_zeros (line 205) stdout ----
[01:20:19]  thread 'num/mod.rs - num::wrapping::Wrapping<i64>::count_zeros (line 205)' panicked at 'test executable failed:
---
[01:20:19]    |
[01:20:19] 10 |     