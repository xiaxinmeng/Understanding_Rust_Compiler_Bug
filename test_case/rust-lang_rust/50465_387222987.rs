plain
[01:10:18] ....................................................................................................
[01:10:30] ....................................................................................................
[01:10:42] ....................................................................................................
[01:10:55] ....................................................................................................
[01:11:06] ....................................F....F..F.FF....F....F..F.F..F..F....F.F.F...F..F.....FF..F...F.
[01:11:18] F....F..F.F...F.F.....F.F.F...F...F.....F...F........F...F..........F...F.........F...F........F....
[01:11:30] F........F...F......................................................................................
[01:11:57] ....................................................................................................
[01:12:11] ....................................................................................................
[01:12:26] ....................................................................................................
[01:12:45] ....................................................................................................
---
[01:12:56] 
[01:12:56] ', librustdoc/test.rs:356:17
[01:12:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i128>::leading_zeros (line 205) stdout ----
[01:12:56]  error[E0369]: binary operation `>>` cannot be applied to type `std::num::Wrapping<i128>`
[01:12:56]  --> num/mod.rs:209:9
[01:12:56]   |
[01:12:56] 7 | let n = Wrapping(i128::max_value()) >> 2;
[01:12:56]   |
[01:12:56]   |
[01:12:56]   = note: an implementation of `std::ops::Shr` might be missing for `std::num::Wrapping<i128>`
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::leading_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i128>::signum (line 209) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i128>::signum (line 209) stdout ----
[01:12:56]  error: incorrect close delimiter: `}`
[01:12:56]   --> num/mod.rs:216:1
[01:12:56] 10 | }
[01:12:56]    | ^
[01:12:56]    |
[01:12:56] note: unclosed delimiter
[01:12:56] note: unclosed delimiter
[01:12:56]   --> num/mod.rs:214:11
[01:12:56]    |
[01:12:56] 8  | assert_eq!(Wrapping(0i128.signum(), Wrapping(0));
[01:12:56] 
[01:12:56] 
[01:12:56] error: no rules expected the token `;`
[01:12:56]  --> num/mod.rs:214:49
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(0i128.signum(), Wrapping(0));
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::signum (line 209)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i128>::to_le (line 208) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i128>::to_le (line 208) stdout ----
[01:12:56]  error: this file contains an un-closed delimiter
[01:12:56]   --> num/mod.rs:219:2
[01:12:56] 14 | }
[01:12:56]    |  ^
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i128>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:218:1
[01:12:56]   |
[01:12:56] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i16>::count_zeros (line 205) stdout ----
---
[01:12:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:56] 
[01:12:56] ', librustdoc/test.rs:356:17
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i16>::signum (line 209) stdout ----
[01:12:56]  error: incorrect close delimiter: `}`
[01:12:56]   --> num/mod.rs:216:1
[01:12:56] 10 | }
[01:12:56]    | ^
[01:12:56]    |
[01:12:56] note: unclosed delimiter
[01:12:56] note: unclosed delimiter
[01:12:56]   --> num/mod.rs:214:11
[01:12:56]    |
[01:12:56] 8  | assert_eq!(Wrapping(0i16.signum(), Wrapping(0));
[01:12:56] 
[01:12:56] 
[01:12:56] error: no rules expected the token `;`
[01:12:56]  --> num/mod.rs:214:48
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(0i16.signum(), Wrapping(0));
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::signum (line 209)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i16>::to_le (line 208) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i16>::to_le (line 208) stdout ----
[01:12:56]  error: this file contains an un-closed delimiter
[01:12:56]   --> num/mod.rs:219:2
[01:12:56] 14 | }
[01:12:56]    |  ^
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i16>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:218:1
[01:12:56]   |
[01:12:56] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i32>::count_zeros (line 205) stdout ----
---
[01:12:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:56] 
[01:12:56] ', librustdoc/test.rs:356:17
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i32>::signum (line 209) stdout ----
[01:12:56]  error: incorrect close delimiter: `}`
[01:12:56]   --> num/mod.rs:216:1
[01:12:56] 10 | }
[01:12:56]    | ^
[01:12:56]    |
[01:12:56] note: unclosed delimiter
[01:12:56] note: unclosed delimiter
[01:12:56]   --> num/mod.rs:214:11
[01:12:56]    |
[01:12:56] 8  | assert_eq!(Wrapping(0i32.signum(), Wrapping(0));
[01:12:56] 
[01:12:56] 
[01:12:56] error: no rules expected the token `;`
[01:12:56]  --> num/mod.rs:214:48
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(0i32.signum(), Wrapping(0));
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::signum (line 209)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i32>::to_le (line 208) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i32>::to_le (line 208) stdout ----
[01:12:56]  error: this file contains an un-closed delimiter
[01:12:56]   --> num/mod.rs:219:2
[01:12:56] 14 | }
[01:12:56]    |  ^
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i32>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:218:1
[01:12:56]   |
[01:12:56] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i64>::count_zeros (line 205) stdout ----
---
[01:12:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:56] 
[01:12:56] ', librustdoc/test.rs:356:17
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i64>::signum (line 209) stdout ----
[01:12:56]  error: incorrect close delimiter: `}`
[01:12:56]   --> num/mod.rs:216:1
[01:12:56] 10 | }
[01:12:56]    | ^
[01:12:56]    |
[01:12:56] note: unclosed delimiter
[01:12:56] note: unclosed delimiter
[01:12:56]   --> num/mod.rs:214:11
[01:12:56]    |
[01:12:56] 8  | assert_eq!(Wrapping(0i64.signum(), Wrapping(0));
[01:12:56] 
[01:12:56] 
[01:12:56] error: no rules expected the token `;`
[01:12:56]  --> num/mod.rs:214:48
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(0i64.signum(), Wrapping(0));
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::signum (line 209)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i64>::to_le (line 208) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i64>::to_le (line 208) stdout ----
[01:12:56]  error: this file contains an un-closed delimiter
[01:12:56]   --> num/mod.rs:219:2
[01:12:56] 14 | }
[01:12:56]    |  ^
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i64>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:218:1
[01:12:56]   |
[01:12:56] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i8>::count_zeros (line 205) stdout ----
---
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i8>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<i8>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:218:1
[01:12:56]   |
[01:12:56] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<i8>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<isize>::count_zeros (line 205) stdout ----
---
[01:12:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:56] 
[01:12:56] ', librustdoc/test.rs:356:17
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<isize>::signum (line 209) stdout ----
[01:12:56]  error: incorrect close delimiter: `}`
[01:12:56]   --> num/mod.rs:216:1
[01:12:56] 10 | }
[01:12:56]    | ^
[01:12:56]    |
[01:12:56] note: unclosed delimiter
[01:12:56] note: unclosed delimiter
[01:12:56]   --> num/mod.rs:214:11
[01:12:56]    |
[01:12:56] 8  | assert_eq!(Wrapping(0isize.signum(), Wrapping(0));
[01:12:56] 
[01:12:56] 
[01:12:56] error: no rules expected the token `;`
[01:12:56]  --> num/mod.rs:214:50
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(0isize.signum(), Wrapping(0));
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<isize>::signum (line 209)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<isize>::to_le (line 208) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<isize>::to_le (line 208) stdout ----
[01:12:56]  error: this file contains an un-closed delimiter
[01:12:56]   --> num/mod.rs:219:2
[01:12:56] 14 | }
[01:12:56]    |  ^
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<isize>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<isize>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:218:1
[01:12:56]   |
[01:12:56] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<isize>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u128>::leading_zeros (line 205) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u128>::leading_zeros (line 205) stdout ----
[01:12:56]  error[E0369]: binary operation `>>` cannot be applied to type `std::num::Wrapping<u128>`
[01:12:56]  --> num/mod.rs:209:9
[01:12:56]   |
[01:12:56] 7 | let n = Wrapping(u128::max_value()) >> 2;
[01:12:56]   |
[01:12:56]   |
[01:12:56]   = note: an implementation of `std::ops::Shr` might be missing for `std::num::Wrapping<u128>`
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<u128>::leading_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u128>::to_le (line 208) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u128>::to_le (line 208) stdout ----
[01:12:56]  error: this file contains an un-closed delimiter
[01:12:56]   --> num/mod.rs:219:2
[01:12:56] 14 | }
[01:12:56]    |  ^
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<u128>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u128>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:218:1
[01:12:56]   |
[01:12:56] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<u128>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u16>::to_le (line 208) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u16>::to_le (line 208) stdout ----
[01:12:56]  error: this file contains an un-closed delimiter
[01:12:56]   --> num/mod.rs:219:2
[01:12:56] 14 | }
[01:12:56]    |  ^
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<u16>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u16>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:218:1
[01:12:56]   |
[01:12:56] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<u16>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u32>::to_le (line 208) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u32>::to_le (line 208) stdout ----
[01:12:56]  error: this file contains an un-closed delimiter
[01:12:56]   --> num/mod.rs:219:2
[01:12:56] 14 | }
[01:12:56]    |  ^
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<u32>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u32>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:218:1
[01:12:56]   |
[01:12:56] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<u32>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u64>::to_le (line 208) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u64>::to_le (line 208) stdout ----
[01:12:56]  error: this file contains an un-closed delimiter
[01:12:56]   --> num/mod.rs:219:2
[01:12:56] 14 | }
[01:12:56]    |  ^
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<u64>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u64>::pow (line 214) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<u64>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismype `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<u8>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<usize>::to_le (line 208) stdout ----
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<usize>::to_le (line 208) stdout ----
[01:12:56]  error: this file contains an un-closed delimiter
[01:12:56]   --> num/mod.rs:219:2
[01:12:56] 14 | }
[01:12:56]    |  ^
[01:12:56]    |
[01:12:56] help: did you mean to close this delimiter?
[01:12:56] help: did you mean to close this delimiter?
[01:12:56]   --> num/mod.rs:208:3
[01:12:56]    |
[01:12:56] 3  | #![feature(wrapping_int_impl)
[01:12:56]    |   ^
[01:12:56] 
[01:12:56] error[E0555]: malformed feature attribute, expected #![feature(...)]
[01:12:56]   --> num/mod.rs:208:1
[01:12:56]    |
[01:12:56] 3  | / #![feature(wrapping_int_impl)
[01:12:56] 4  | | fn main() {
[01:12:56] 5  | | use std::num::Wrapping;
[01:12:56] ...  |
[01:12:56] 13 | | }
[01:12:56] 14 | | }
[01:12:56]    | |_^
[01:12:56]    | |_^
[01:12:56] 
[01:12:56] error: unexpected token: `fn`
[01:12:56]  --> num/mod.rs:209:1
[01:12:56]   |
[01:12:56] 3 | #![feature(wrapping_int_impl)
[01:12:56]   |                             - unexpected token after this
[01:12:56] 4 | fn main() {
[01:12:56]   | ^^ unexpected token
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<usize>::to_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] ---- num/mod.rs - num::wrapping::Wrapping<usize>::pow (line 214) stdout ----
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:218:1
[01:12:56]   |
[01:12:56] 7 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] error[E0308]: mismatched types
[01:12:56]  --> num/mod.rs:219:1
[01:12:56]   |
[01:12:56]   |
[01:12:56] 8 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:12:56]   | |
[01:12:56]   | |
[01:12:56]   | expected struct `std::num::Wrapping`, found integral variable
[01:12:56]   | help: try using a variant of the expected type: `std::num::Wrapping(*right_val)`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]   = note: expected type `std::num::Wrapping<i8>`
[01:12:56]              found type `{integer}`
[01:12:56] 
[01:12:56] thread 'num/mod.rs - num::wrapping::Wrapping<usize>::pow (line 214)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:12:56] 
[01:12:56] 
---
[01:12:56] 
[01:12:56] error: test failed, to rerun pass '--doc'
[01:12:56] 
[01:12:56] 
[01:12:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:12:56] 
[01:12:56] 
[01:12:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:56] Build completed unsuccessfully in 0:32:10
[01:12:56] Build completed unsuccessfully in 0:32:10
[01:12:56] make: *** [check] Error 1
[01:12:56] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07db24f4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
