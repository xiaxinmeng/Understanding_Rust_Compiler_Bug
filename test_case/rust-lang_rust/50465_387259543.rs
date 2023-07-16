plain
[01:10:28] ....................................................................................................
[01:10:40] ....................................................................................................
[01:10:53] ....................................................................................................
[01:11:05] ....................................................................................................
[01:11:17] ........................................F....F..F........F...F...F.......F...F..F........F...F...F..
[01:11:29] .....F...F..F........F...F..F.....F....F....F.........F...F........F....F........F....F........F...F
[01:11:41] ..........F...F.....................................................................................
[01:12:08] ....................................................................................................
[01:12:22] ....................................................................................................
[01:12:37] ....................................................................................................
[01:12:56] ....................................................................................................
[01:12:56] ....................................................................................................
6] 
[01:13:06] error: unexpected token: `fn`
[01:13:06]  --> num/mod.rs:209:1
[01:13:06]   |
[01:13:06] 3 | #![feature(wrapping_int_impl)
[01:13:06]   |                             - unexpected token after this
[01:13:06] 4 | fn main() {
[01:13:06]   | ^^ unexpected token
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<i128>::pow (line 214) stdout ----
[01:13:06]  error[E0308]: mismatched types
[01:13:06]  error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:218:1
[01:13:06]   |
[01:13:06] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:219:1
[01:13:06]   |
[01:13:06]   |
[01:13:06] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<i16>::leading_zeros (line 205) stdout ----
---
[01:13:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:13:06] 
[01:13:06] ', librustdoc/test.rs:356:17
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<i16>::to_le (line 208) stdout ----
[01:13:06]  error: this file contains an un-closed delimiter
[01:13:06]   --> num/mod.rs:219:2
[01:13:06] 14 | }
[01:13:06]    |  ^
[01:13:06]    |
[01:13:06] help: did you mean to close this delimiter?
[01:13:06] help: did you mean to close this delimiter?
[01:13:06]   --> num/mod.rs:208:3
[01:13:06]    |
[01:13:06] 3  | #![feature(wrapping_int_impl)
[01:13:06]    |   ^
[01:13:06] 
[01:13:06] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:13:06]   --> num/mod.rs:208:1
[01:13:06]    |
[01:13:06] 3  | / #![feature(wrapping_int_impl)
[01:13:06] 4  | | fn main() {
[01:13:06] 5  | | use std::num::Wrapping;
[01:13:06try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<i32>::leading_zeros (line 205) stdout ----
---
[01:13:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:13:06] 
[01:13:06] ', librustdoc/test.rs:356:17
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<i32>::to_le (line 208) stdout ----
[01:13:06]  error: this file contains an un-closed delimiter
[01:13:06]   --> num/mod.rs:219:2
[01:13:06] 14 | }
[01:13:06]    |  ^
[01:13:06]    |
[01:13:06] help: did you mean to close this delimiter?
[01:13:06] help: did you mean to close this delimiter?
[01:13:06]   --> num/mod.rs:208:3
[01:13:06]    |
[01:13:06] 3  | #![feature(wrapping_int_impl)
[01:13:06]    |   ^
[01:13:06] 
[01:13:06] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:13:06]   --> num/mod.rs:208:1
[01:13:06]    |
[01:13:06] 3  | / #![feature(wrapping_int_impl)
[01:13:06] 4  | | fn main() {
[01:13:06] 5  | | use std::num::Wrapping;
[01:13:06] ...  |
[01:13:06] 13 | | }
[01:13:06] 14 | | }
[01:13:06]    | |_^
[01:13:06]    | |_^
[01:13:06] 
[01:13:06] error: unexpected token: `fn`
[01:13:06]  --> num/mod.rs:209:1
[01:13:06]   |
[01:13:06] 3 | #![feature(wrapping_int_impl)
[01:13:06]   |                             - unexpected token after this
[01:13:06] 4 | fn main() {
[01:13:06]   | ^^ unexpected token
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<i32>::pow (line 214) stdout ----
[01:13:06]  error[E0308]: mismatched types
[01:13:06]  error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:218:1
[01:13:06]   |
[01:13:06] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:219:1
[01:13:06]   |
[01:13:06]   |
[01:13:06] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:13:06]   | |
[0d types
[01:13:06]  --> num/mod.rs:219:1
[01:13:06]   |
[01:13:06]   |
[01:13:06] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<i8>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<isize>::leading_zeros (line 205) stdout ----
---
[01:13:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:13:06] 
[01:13:06] ', librustdoc/test.rs:356:17
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<isize>::to_le (line 208) stdout ----
[01:13:06]  error: this file contains an un-closed delimiter
[01:13:06]   --> num/mod.rs:219:2
[01:13:06] 14 | }
[01:13:06]    |  ^
[01:13:06]    |
[01:13:06] help: did you mean to close this delimiter?
[01:13:06] help: did you mean to close this delimiter?
[01:13:06]   --> num/mod.rs:208:3
[01:13:06]    |
[01:13:06] 3  | #![feature(wrapping_int_impl)
[01:13:06]    |   ^
[01:13:06] 
[01:13:06] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:13:06]   --> num/mod.rs:208:1
[01:13:06]    |
[01:13:06] 3  | / #![feature(wrapping_int_impl)
[01:13:06] 4  | | fn main() {
[01:13:06] 5  | | use std::num::Wrapping;
[01:13:06] ...  |
[01:13:06] 13 | | }
[01:13:06] 14 | | }
[01:13:06]    | |_^
[01:13:06]    | |_^
[01:13:06] 
[01:13:06] error: unexpected token: `fn`
[01:13:06]  --> num/mod.rs:209:1
[01:13:06]   |
[01:13:06] 3 | #![feature(wrapping_int_impl)
[01:13:06]   |                             - unexpected token after this
[01:13:06] 4 | fn main() {
[01:13:06]   | ^^ unexpected token
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<isize>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<isize>::pow (line 214) stdout ----
[01:13:06]  error[E0308]: mismatched types
[01:13:06]  error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:218:1
[01:13:06]   |
[01:13:06] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:219:1
[01:13:06]   |
[01:13:06]   |
[01:13:06] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<isize>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<u128>::leading_zeros (line 205) stdout ----
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<u128>::leading_zeros (line 205) stdout ----
[01:13:06]  error[E0369]: binary operation `>>` cannot be applied to type `std::num::Wrapping<u128>`
[01:13:06]  --> num/mod.rs:209:9
[01:13:06]   |
[01:13:06] 7 | let n = Wrapping(!0u128) >> 2usize;
[01:13:06]   |
[01:13:06]   |
[01:13:06]   = note: an implementation of `std::ops::Shr` might be missing for `std::num::Wrapping<u128>`
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<u128>::leading_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<u128>::to_le (lined::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:219:1
[01:13:06]   |
[01:13:06]   |
[01:13:06] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<u128>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<u16>::to_le (line 208) stdout ----
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<u16>::to_le (line 208) stdout ----
[01:13:06]  error: this file contains an un-closed delimiter
[01:13:06]   --> num/mod.rs:219:2
[01:13:06] 14 | }
[01:13:06]    |  ^
[01:13:06]    |
[01:13:06] help: did you mean to close this delimiter?
[01:13:06] help: did you mean to close this delimiter?
[01:13:06]   --> num/mod.rs:208:3
[01:13:06]    |
[01:13:06] 3  | #![feature(wrapping_int_impl)
[01:13:06]    |   ^
[01:13:06] 
[01:13:06] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:13:06]   --> num/mod.rs:208:1
[01:13:06]    |
[01:13:06] 3  | / #![feature(wrapping_int_impl)
[01:13:06] 4  | | fn main() {
[01:13:06] 5  | | use std::num::Wrapping;
[01:13:06] ...  |
[01:13:06] 13 | | }
[01:13:06] 14 | | }
[01:13:06]    | |_^
[01:13:06]    | |_^
[01:13:06] 
[01:13:06] error: unexpected token: `fn`
[01:13:06]  --> num/mod.rs:209:1
[01:13:06]   |
[01:13:06] 3 | #![feature(wrapping_int_impl)
[01:13:06]   |                             - unexpected token after this
[01:13:06] 4 | fn main() {
[01:13:06]   | ^^ unexpected token
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<u16>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<u16>::pow (line 214) stdout ----
[01:13:06]  error[E0308]: mismatched types
[01:13:06]  error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:218:1
[01:13:06]   |
[01:13:06] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:219:1
[01:13:0to_le (line 208) stdout ----
[01:13:0to_le (line 208) stdout ----
[01:13:06]  error: this file contains an un-closed delimiter
[01:13:06]   --> num/mod.rs:219:2
[01:13:06] 14 | }
[01:13:06]    |  ^
[01:13:06]    |
[01:13:06] help: did you mean to close this delimiter?
[01:13:06] help: did you mean to close this delimiter?
[01:13:06]   --> num/mod.rs:208:3
[01:13:06]    |
[01:13:06] 3  | #![feature(wrapping_int_impl)
[01:13:06]    |   ^
[01:13:06] 
[01:13:06] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:13:06]   --> num/mod.rs:208:1
[01:13:06]    |
[01:13:06] 3  | / #![feature(wrapping_int_impl)
[01:13:06] 4  | | fn main() {
[01:13:06] 5  | | use std::num::Wrapping;
[01:13:06] ...  |
[01:13:06] 13 | | }
[01:13:06] 14 | | }
[01:13:06]    | |_^
[01:13:06]    | |_^
[01:13:06] 
[01:13:06] error: unexpected token: `fn`
[01:13:06]  --> num/mod.rs:209:1
[01:13:06]   |
[01:13:06] 3 | #![feature(wrapping_int_impl)
[01:13:06]   |                             - unexpected token after this
[01:13:06] 4 | fn main() {
[01:13:06]   | ^^ unexpected token
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<u8>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<u8>::pow (line 214) stdout ----
[01:13:06]  error[E0308]: mismatched types
[01:13:06]  error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:218:1
[01:13:06]   |
[01:13:06] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] error[E0308]: mismatched types
[01:13:06]  --> num/mod.rs:219:1
[01:13:06]   |
[01:13:06]   |
[01:13:06] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<u8>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<usize>::to_le (line 208) stdout ----
[01:13:06] ---- num/mod.rs - num::wrapping::Wrapping<usize>::to_le (line 208) stdout ----
[01:13:06]  error: this file contains an un-closed delimiter
[01:13:06]   --> num/mod.rs:219:2
[01:13:06] 14 | }
[01:13:06]    |  ^
[01:13:06]    |
[01:13:06] help: did you mean to close this delimiter?
[01:13:06] help: did you mean to close this delimiter?
[01:13:06]   --> num/mod.rs:208:3
[01:13:06]    |
[01:13:06] 3  | #![feature(wrapping_int_impl)
[01:13:06]    |   ^
[01:13:06] error[E0555]1
[01:13:06]   |
[01:13:06]   |
[01:13:06] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:13:06]   | |
[01:13:06]   | |
[01:13:06]   | expected struct `std::num::Wrapping`, found integral variable
[01:13:06]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]   = note: expected type `std::num::Wrapping<i8>`
[01:13:06]              found type `{integer}`
[01:13:06] 
[01:13:06] thread 'num/mod.rs - num::wrapping::Wrapping<usize>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:06] 
[01:13:06] 
