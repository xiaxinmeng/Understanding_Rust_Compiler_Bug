
[00:55:10]   |     ^
[00:55:10] 
[00:55:10] 
[00:55:10] warning: Backing out of syntax highlighting
[00:55:10]   |
[00:55:10]   = note: You probably did not intend to render this as a rust code-block
[00:55:13]     Finished release [optimized] target(s) in 114.77 secs
[00:55:13] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:55:14]    Compiling term v0.0.0 (file:///checkout/src/libterm)
[00:55:14]    Compiling getopts v0.2.17
---
[01:37:25] ....................................................................................................
[01:37:42] ....................................................................................................
[01:37:58] ....................................................................................................
[01:38:14] ....................................................................................................
[01:38:23] .................................FF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[01:38:27] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[01:38:43] FFFFFFFFFFFFFF......................................................................................
[01:39:00] ....................................................................................................
[01:39:18] ............F..F...F...F...F...F...F...F....F..F...F...F............................................
[01:39:58] ....................................................................................................
[01:40:27] ....................................................................................................
[01:40:27] ....................................................................................................
tion `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:210:9
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0x1Ai128);
[01:40:41]   |         ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::from_be (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::from_be (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i128>::is_negative (line 206) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:208:9
[01:40:41]   |
[01:40:41] 5 | assert!(Wrapping(-10i128).is_negative());
[01:40:41]   |         ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:209:10
[01:40:41]   |
[01:40:41] 6 | assert!(!Wrapping(10i128).is_negative());
[01:40:41]   |          ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::is_negative (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::is_negative (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - [01:40:41]   |     ^
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::signum (line 209)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i128>::max_value (line 205) stdout ----
[01:40:41]  error[E0412]: cannot find type `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:13
[01:40:41]   |
[01:40:41] 5 | assert_eq!(<Wrapping<i128>>::max_value(), Wrapping(i128::max_value()));
[01:40:41]   |             ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:43
[01:40:41]   |
[01:40:41] 5 | assert_eq!(<Wrapping<i128>>::max_value(), Wrapping(i128::max_value()));
[01:40:41]   |                                           ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::max_value (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::max_value (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i128>::pow (line 212) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:214:12
[01:40:41]   |
[01:40:41] 5 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:215:12
[01:40:41]   |
[01:40:41] 6 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::pow (line 212)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::pow (line 212)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i128>::min_value (line 205) stdout ----
[01:40:41]  error[E0412]: cannot find type `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:13
[01:40:41]   |
[01:40:41] 5 | assert_eq!(<Wrapping<i128>>::min_value(), Wrapping(i128::min_value()));
[01:40:41]   |             ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:43
[01:40:41]   |
[01:40:41] 5 | assert_eq!(<Wrapping<i128>>::min_value(), Wrapping(i128::min_value()));
[01:40:41]   |                                           ^^^^^^^^ not found in thisle candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::count_ones (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::count_ones (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i16>::count_zeros (line 205) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:12
[01:40:41]   |
[01:40:41] 5 | assert_eq!(Wrapping(i16::max_value()).count_zeros(), 0);
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::count_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::count_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i16>::abs (line 210) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:212:12
[01:40:41]   |
[01:40:41] 5 | assert_eq!(Wrapping(100i16).abs(), Wrapping(100));
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:212:36
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:214:46
[01:40:41]   |
[01:40:41] 7 | assert_eq!(Wrapping(i16::min_value()).abs(), Wrapping(i16::min_value()));
[01:40:41]   |                                              ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:215:12
[01:40:41]   |
[01:40:41] 8 | assert_eq!(Wrapping(-128i8).abs().0 as u8, 128u8);
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::abs (line 210)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i16>::from_le (line 208) stdout ----
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i16>::from_le (line 208) stdout ----
[01:40:41]  error: expected one of `,`, `.`, `?`, or an operator, found `$`
[01:40:41]  --> num/mod.rs:210:22
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0x1A$ SelfT);
[01:40:41]   |                      ^ expected one of `,`, `.`, `?`, or an operator here
[01:40:41] 
[01:40:41] error: expected one of `.`, `;`, `<`, `?`, `break`, `continue`, `false`, `for`, `if`, `loop`, `match`, `move`, `return`, `static`, `true`, `unsafe`, `while`, `yield`, or an opera01:40:41]  --> num/mod.rs:209:10
[01:40:41]   |
[01:40:41] 6 | assert!(!Wrapping(-10i16).is_positive());
[01:40:41]   |          ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::is_positive (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::is_positive (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i16>::leading_zeros (line 205) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:9
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(i16::max_value()) >> 2;
[01:40:41]   |         ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::leading_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i16>::leading_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i16>::pow (line 205) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:12
[01:40:41]   |
[01:40:41] 5 | assert_eq!(Wrapping(3i16).pow(4), Wrapping(81));
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 10:22
[01:40:41]   |
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0x1A$ SelfT);
[01:40:41]   |                      ^ expected one of `,`, `.`, `?`, or an operator here
[01:40:41] 
[01:40:41] error: expected one of `.`, `;`, `<`, `?`, `break`, `continue`, `false`, `for`, `if`, `loop`, `match`, `move`, `return`, `static`, `true`, `unsafe`, `while`, `yield`, or an operator, found `$`
[01:40:41]  --> num/mod.rs:210:22
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0x1A$ SelfT);
[01:40:41]   |                      ^ expected one of 19 possible tokens here
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::from_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i32>::count_zeros (line 205) stdout ----
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i32>::count_zeros (line 205) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:12
[01:40:41]   |
[01:40:41] 5 | assert_eq!(Wrapping(i32::max_value()).count_zeros(), 0);
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::count_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::count_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i32>::from_be (line 208) stdout ----
[01:40:41]  error[E0433]: failed to resolve. Use of undeclared type or module `Wrapping`
[01:40:41]  --> num/mod.rs:213:16
[01:40:4[01:40:41]   |
[01:40:41] 6 | assert!(!Wrapping(10i32).is_negative());
[01:40:41]   |          ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::is_negative (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::is_negative (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i32>::is_positive (line 206) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:208:9
[01:40:41]   |
[01:40:41] 5 | assert!(Wrapping(10i32).is_positive());
[01:40:41]   |         ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:209:10
[01:40:41]   |
[01:40:41] 6 | assert!(!Wrapping(-10i32).is_positive());
[01:40:41]   |          ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::is_positive (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::is_positive (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i32>::leading_zeros (line 20use std::num::Wrapping;
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::max_value (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i32>::pow (line 212) stdout ----
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i32>::pow (line 212) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:214:12
[01:40:41]   |
[01:40:41] 5 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:215:12
[01:40:41]   |
[01:40:41] 6 | assert_eq!(Wrapping(3i8).pow(6), -39);
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::pow (line 212)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i32>::pow (line 212)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i32>::min_value (line 205) stdout ----
[01:40:41]  error[E0412]: cannot find type `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:13
[01:40:41]   |
[01:40:41] 5 | assert_eq!(<Wrapping<i32>>::min_value(), Wrapping(i32::min_value()));
[01:40:41]   |             ^^^^^^^^ not found in this [01:40:41]  error: expected one of `,`, `.`, `?`, or an operator, found `$`
[01:40:41]  --> num/mod.rs:210:22
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0x1A$ SelfT);
[01:40:41]   |                      ^ expected one of `,`, `.`, `?`, or an operator here
[01:40:41] 
[01:40:41] error: expected one of `.`, `;`, `<`, `?`, `break`, `continue`, `false`, `for`, `if`, `loop`, `match`, `move`, `return`, `static`, `true`, `unsafe`, `while`, `yield`, or an operator, found `$`
[01:40:41]  --> num/mod.rs:210:22
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0x1A$ SelfT);
[01:40:41]   |                      ^ expected one of 19 possible tokens here
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::from_le (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i64>::from_be (line 208) stdout ----
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i64>::from_be (line 208) stdout ----
[01:40:41]  error[E0433]: failed to resolve. Use of undeclared type or module `Wrapping`
[01:40:41]  --> num/mod.rs:213:16
[01:40:41]   |
[01:40:41] 8 |     assert_eq!(Wrapping::from_be(n), n)
[01:40:41]   |                ^^^^^^^^ Use of undeclared type or module `Wrapping`
[01:40:41] 
[01:40:41] error[E0433]: failed to resolve. Use of undeclared type or module `Wrapping`
[01:40:41]   --> num/mod.rs:215:16
[01:40:41]    |
[01:40:41] 10 |     assert_eq!(Wrapping::from_be(n), n.swap_bytes())
[01:40:41]    |                ^^^^^^^^ Use of undeclared type or module `Wrapping`
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:210:9
num (line 209)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i64>::max_value (line 205) stdout ----
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i64>::max_value (line 205) stdout ----
[01:40:41]  error[E0412]: cannot find type `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:13
[01:40:41]   |
[01:40:41] 5 | assert_eq!(<Wrapping<i64>>::max_value(), Wrapping(i64::max_value()));
[01:40:41]   |             ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:42
[01:40:41]   |
[01:40:41] 5 | assert_eq!(<Wrapping<i64>>::max_value(), Wrapping(i64::max_value()));
[01:40:41]   |                                          ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::max_value (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::max_value (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i64>::pow (line 212) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:214:12
[01:40:41]   |
[01:40:41] 5 | assert_eq!(Wrapping(3i8).pow(5), -13);
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in  in this scope
[01:40:41]  --> num/mod.rs:210:9
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0x1Ai64);
[01:40:41]   |         ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::to_be (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::to_be (line 208)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i64>::trailing_zeros (line 206) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:208:9
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0b0101000i64);
[01:40:41]   |         ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::trailing_zeros (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i64>::trailing_zeros (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i8>::count_ones (line 205) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:9
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0b01001100i8);
[01:40:41]   |         ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[scope
[01:40:41]  --> num/mod.rs:213:36
[01:40:41]   |
[01:40:41]   |
[01:40:41] 6 | assert_eq!(Wrapping(-100i8).abs(), Wrapping(100));
[01:40:41]   |                                    ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:214:12
[01:40:41]   |
[01:40:41] 7 | assert_eq!(Wrapping(i8::min_value()).abs(), Wrapping(i8::min_value()));
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:214:45
[01:40:41]   |
[01:40:41] 7 | assert_eq!(Wrapping(i8::min_value()).abs(), Wrapping(i8::min_value()));
[01:40:41]   |                                             ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:215:12
[01:40:41]   |
[01:40:41] 8 | assert_eq!(Wrapping(-128i8).abs().0 as u8, 128u8);
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i8>::abs (line 210)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i8>::count_zeros (line 205) stdout ----
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i8>::count_zeros (line 205) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:12
[01:40:41]   |
[01:40:41] 5 | assert_eq!(Wrapping(i8::max_value()).count_zeros(), 0);
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i8>::count_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i8>::count_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i8>::from_le (line 208) stdout ----
[01:40:41]  error: expected one of `,`, `.`, `?`, or an operator, found `$`
[01:40:41]  --> num/mod.rs:210:22
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0x1A$ SelfT);
[01:40:41]   |                      ^ expected one of `,`, `.`, `?`, or an operator here
[01:40:41] 
[01:40:41] error: expected one of `.`, `;`, `<`, `?`, `break`, `continue`, `false`, `for`, `if`, `loop`, `match`, `move`, `return`, `static`, `true`, `unsafe`, `while`, `yield`, or an operator, found `$`
[01:40:41]  --> num/mod.rs:210:22
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(0x1A$ SelfT);
[01:40:41]   |                  s scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i8>::is_positive (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i8>::is_positive (line 206)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i8>::leading_zeros (line 205) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:9
[01:40:41]   |
[01:40:41] 5 | let n = Wrapping(i8::max_value()) >> 2;
[01:40:41]   |         ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i8>::leading_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] thread 'num/mod.rs - num::wrapping::Wrapping<i8>::leading_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:40:41] 
[01:40:41] ---- num/mod.rs - num::wrapping::Wrapping<i8>::pow (line 205) stdout ----
[01:40:41]  error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:12
[01:40:41]   |
[01:40:41] 5 | assert_eq!(Wrapping(3i8).pow(4), Wrapping(81));
[01:40:41]   |            ^^^^^^^^ not found in this scope
[01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
[01:40:41]   |
[01:40:41] 
[01:40:41] 
[01:40:41] error[E0425]: cannot find function `Wrapping` in this scope
[01:40:41]  --> num/mod.rs:207:34
01:40:41] help: possible candidate is found in another module, you can import it into scope
[01:40:41] 4 | use std::num::Wrapping;
---
[01:40:41] 
[01:40:41] error: test failed, to rerun pass '--doc'
[01:40:41] 
[01:40:41] 
[01:40:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:40:41] 
[01:40:41] 
[01:40:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:41] Build completed unsuccessfully in 0:44:25
[01:40:41] Build completed unsuccessfully in 0:44:25
[01:40:41] make: *** [check] Error 1
[01:40:41] Makefile:58: recipe for target 'check' failed
