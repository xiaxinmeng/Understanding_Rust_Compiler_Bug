plain
[01:03:43] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:43]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:36:17
[01:03:46]    |
[01:03:46] 36 |         assert!(INFINITY.mod_euc(a).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:36:34
[01:03:46]    |
[01:03:46] 36 |         assert!(INFINITY.mod_euc(a).is_nan());
[01:03:46]    |                                  ^ did you mean `A`?
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:37:20
[01:03:46]    |
[01:03:46] 37 |         assert_eq!(a.mod_euc(INFINITY), a);
[01:03:46]    |                    ^ did you mean `A`?
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:37:30
[01:03:46]    |
[01:03:46] 37 |         assert_eq!(a.mod_euc(INFINITY), a);
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:37:41
[01:03:46]    |
[01:03:46] 37 |         assert_eq!(a.mod_euc(INFINITY), a);
[01:03:46]    |                                         ^ did you mean `A`?
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:38:17
[01:03:46]    |
[01:03:46] 38 |         assert!(a.mod_euc(NAN).is_nan());
[01:03:46]    |                 ^ did you mean `A`?
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `NAN` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:38:27
[01:03:46]    |
[01:03:46] 38 |         assert!(a.mod_euc(NAN).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use core::f64::NAN;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use std::f64::NAN;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:39:17
[01:03:46]    |
[01:03:46] 39 |         assert!(INFINITY.mod_euc(INFINITY).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:39:34
[01:03:46]    |
[01:03:46] 39 |         assert!(INFINITY.mod_euc(INFINITY).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:40:17
[01:03:46]    |
[01:03:46] 40 |         assert!(INFINITY.mod_euc(NAN).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `NAN` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:40:34
[01:03:46]    |
[01:03:46] 40 |         assert!(INFINITY.mod_euc(NAN).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use core::f64::NAN;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use std::f64::NAN;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `NAN` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:41:17
[01:03:46]    |
[01:03:46] 41 |         assert!(NAN.mod_euc(INFINITY).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use core::f64::NAN;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use std::f64::NAN;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:41:29
[01:03:46]    |
[01:03:46] 41 |         assert!(NAN.mod_euc(INFINITY).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:46:20
[01:03:46]    |
[01:03:46] 46 |         assert_eq!(a.div_euc(INFINITY), 0.0);
[01:03:46]    |                    ^ did you mean `A`?
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:46:30
[01:03:46]    |
[01:03:46] 46 |         assert_eq!(a.div_euc(INFINITY), 0.0);
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:47:17
[01:03:46]    |
[01:03:46] 47 |         assert!(a.div_euc(NAN).is_nan());
[01:03:46]    |                 ^ did you mean `A`?
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `NAN` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:47:27
[01:03:46]    |
[01:03:46] 47 |         assert!(a.div_euc(NAN).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use core::f64::NAN;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use std::f64::NAN;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:48:17
[01:03:46]    |
[01:03:46] 48 |         assert!(INFINITY.div_euc(INFINITY).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:48:34
[01:03:46]    |
[01:03:46] 48 |         assert!(INFINITY.div_euc(INFINITY).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:49:17
[01:03:46]    |
[01:03:46] 49 |         assert!(INFINITY.div_euc(NAN).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `NAN` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:49:34
[01:03:46]    |
[01:03:46] 49 |         assert!(INFINITY.div_euc(NAN).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use core::f64::NAN;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use std::f64::NAN;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `NAN` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:50:17
[01:03:46]    |
[01:03:46] 50 |         assert!(NAN.div_euc(INFINITY).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use core::f64::NAN;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::NAN;
[01:03:46]    |
[01:03:46] 1  | use std::f64::NAN;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:50:29
[01:03:46]    |
[01:03:46] 50 |         assert!(NAN.div_euc(INFINITY).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i8.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i8, i8);
[01:03:46]    | -------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:36:17
[01:03:46]    |
[01:03:46] 36 |         assert!(INFINITY.mod_euc(a).is_nan());
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i16.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i16, i16);
[01:03:46]    | ---------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:36:34
[01:03:46]    |
[01:03:46] 36 |         assert!(INFINITY.mod_euc(a).is_nan());
[01:03:46]    |                                  ^ did you mean `A`?
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i16.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i16, i16);
[01:03:46]    | ---------------------- in this macro invocation
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:37:20
[01:03:46]    |
[01:03:46] 37 |         assert_eq!(a.mod_euc(INFINITY), a);
[01:03:46]    |                    ^ did you mean `A`?
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i16.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i16, i16);
[01:03:46]    | ---------------------- in this macro invocation
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `INFINITY` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:37:30
[01:03:46]    |
[01:03:46] 37 |         assert_eq!(a.mod_euc(INFINITY), a);
[01:03:46]    | 
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i16.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i16, i16);
[01:03:46]    | ---------------------- in this macro invocation
[01:03:46] help: possible candidates are found in other modules, you can import them into scope
[01:03:46] 1  | use core::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use core::f64::INFINITY;
[01:03:46]    |
[01:03:46]    |
[01:03:46] 1  | use std::f32::INFINITY;
[01:03:46]    |
[01:03:46] 1  | use std::f64::INFINITY;
[01:03:46]    |
[01:03:46] 
[01:03:46] error[E0425]: cannot find value `a` in this scope
[01:03:46]   --> libcore/../libcore/tests/num/int_macros.rs:37:41
[01:03:46]    |
[01:03:46] 37 |         assert_eq!(a.mod_euc(INFINITY), a);
[01:03:46]    |                                         ^ did you mean `A`?
[01:03:46]    | 
[01:03:46]   ::: libcore/../libcore/tests/num/i16.rs:11:1
[01:03:46]    |
[01:03:46] 11 | int_module!(i16, i16);
---
[01:03:58] 
[01:03:58] To learn more, run the command again with --verbose.
[01:03:58] 
[01:03:58] 
[01:03:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:03:58] 
[01:03:58] 
[01:03:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:58] Build completed unsuccessfully in 0:23:24
[01:03:58] Build completed unsuccessfully in 0:23:24
[01:03:58] make: *** [check] Error 1
[01:03:58] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:085ce750
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
