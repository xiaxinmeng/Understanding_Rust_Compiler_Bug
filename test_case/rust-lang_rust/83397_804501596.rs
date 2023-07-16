plain
   |
22 | use crate::intrinsics;
   | ^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: constant is private but has a stability attribute
  --> library/core/src/num/bignum.rs:94:1
   |
   |
94 | const SMALL_POW5: [(u64, usize); 3] = [(125, 3), (15625, 6), (1_220_703_125, 13)];
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:87:1
   |
87 | use crate::fmt;
87 | use crate::fmt;
   | ^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:88:1
   |
88 | use crate::str::FromStr;
88 | use crate::str::FromStr;
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:90:1
   |
90 | use self::num::digits_to_big;
90 | use self::num::digits_to_big;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:91:1
   |
   |
91 | use self::parse::{parse_decimal, Decimal, ParseResult, Sign};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:91:19
   |
   |
91 | use self::parse::{parse_decimal, Decimal, ParseResult, Sign};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:91:34
   |
   |
91 | use self::parse::{parse_decimal, Decimal, ParseResult, Sign};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:91:43
   |
   |
91 | use self::parse::{parse_decimal, Decimal, ParseResult, Sign};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:91:56
   |
   |
91 | use self::parse::{parse_decimal, Decimal, ParseResult, Sign};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:92:1
   |
92 | use self::rawfp::RawFloat;
92 | use self::rawfp::RawFloat;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: module is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:94:1
   |
   |
94 | mod algorithm;
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:3:1
  |
3 | use crate::cmp::min;
3 | use crate::cmp::min;
  | ^^^^^^^^^^^^^^^^^^^^
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:4:1
  |
  |
4 | use crate::cmp::Ordering::{Equal, Greater, Less};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:4:28
  |
  |
4 | use crate::cmp::Ordering::{Equal, Greater, Less};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:4:35
  |
  |
4 | use crate::cmp::Ordering::{Equal, Greater, Less};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:4:44
  |
  |
4 | use crate::cmp::Ordering::{Equal, Greater, Less};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:5:1
  |
  |
5 | use crate::num::dec2flt::num::{self, Big};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:5:32
  |
  |
5 | use crate::num::dec2flt::num::{self, Big};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:5:38
  |
  |
5 | use crate::num::dec2flt::num::{self, Big};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:6:1
  |
  |
6 | use crate::num::dec2flt::rawfp::{self, fp_to_float, next_float, prev_float, RawFloat, Unpacked};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:6:34
  |
  |
6 | use crate::num::dec2flt::rawfp::{self, fp_to_float, next_float, prev_float, RawFloat, Unpacked};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:6:40
  |
  |
6 | use crate::num::dec2flt::rawfp::{self, fp_to_float, next_float, prev_float, RawFloat, Unpacked};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:6:53
  |
  |
6 | use crate::num::dec2flt::rawfp::{self, fp_to_float, next_float, prev_float, RawFloat, Unpacked};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:6:65
  |
  |
6 | use crate::num::dec2flt::rawfp::{self, fp_to_float, next_float, prev_float, RawFloat, Unpacked};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:6:77
  |
  |
6 | use crate::num::dec2flt::rawfp::{self, fp_to_float, next_float, prev_float, RawFloat, Unpacked};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:6:87
  |
  |
6 | use crate::num::dec2flt::rawfp::{self, fp_to_float, next_float, prev_float, RawFloat, Unpacked};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:7:1
  |
7 | use crate::num::dec2flt::table;
7 | use crate::num::dec2flt::table;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/algorithm.rs:8:1
  |
  |
8 | use crate::num::diy_float::Fp;
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: constant is private but has a stability attribute
  --> library/core/src/num/dec2flt/algorithm.rs:11:1
   |
   |
11 | const P: u32 = 64;
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
  --> library/core/src/num/dec2flt/algorithm.rs:16:1
   |
   |
16 | / fn power_of_ten(e: i16) -> Fp {
17 | |     assert!(e >= table::MIN_E);
18 | |     let i = e - table::MIN_E;
19 | |     let sig = table::POWERS.0[i as usize];
20 | |     let exp = table::POWERS.1[i as usize];
21 | |     Fp { f: sig, e: exp }
   | |_^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: module is private but has a stability attribute
  --> library/core/src/num/dec2flt/algorithm.rs:27:1
   |
   |
27 | / mod fpu_precision {
28 | |     pub fn set_precision<T>() {}
   | |_^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
  --> library/core/src/num/dec2flt/algorithm.rs:28:5
   |
28 |     pub fn set_precision<T>() {}
28 |     pub fn set_precision<T>() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/algorithm.rs:115:1
    |
    |
115 | / pub fn fast_path<T: RawFloat>(integral: &[u8], fractional: &[u8], e: i64) -> Option<T> {
116 | |     let num_digits = integral.len() + fractional.len();
117 | |     // log_10(f64::MAX_SIG) ~ 15.95. We compare the exact value to MAX_SIG near the end,
118 | |     // this is just a quick, cheap rejection (and also frees the rest of the code from
145 | |     }
146 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/algorithm.rs:163:1
    |
    |
163 | / pub fn bellerophon<T: RawFloat>(f: &Big, e: i16) -> T {
164 | |     let slop = if f <= &Big::from_u64(T::MAX_SIG) {
165 | |         // The cases abs(e) < log5(2^N) are in fast_path()
166 | |         if e >= 0 { 0 } else { 3 }
179 | |     }
180 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/algorithm.rs:187:1
    |
    |
187 | / fn algorithm_r<T: RawFloat>(f: &Big, e: i16, z0: T) -> T {
188 | |     let mut z = z0;
189 | |     loop {
190 | |         let raw = z.unpack();
242 | |     }
243 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/algorithm.rs:248:1
    |
    |
248 | / fn make_ratio(x: &mut Big, y: &mut Big, e: i16, k: i16) {
249 | |     let (e_abs, k_abs) = (e.abs() as usize, k.abs() as usize);
250 | |     if e >= 0 {
251 | |         if k >= 0 {
274 | |     }
275 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/algorithm.rs:294:1
    |
    |
294 | / pub fn algorithm_m<T: RawFloat>(f: &Big, e: i16) -> T {
295 | |     let mut u;
296 | |     let mut v;
297 | |     let e_abs = e.abs() as usize;
...   |
343 | |     round_by_remainder(v, rem, q, z)
    | |_^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/algorithm.rs:347:1
    |
    |
347 | / fn quick_start<T: RawFloat>(u: &mut Big, v: &mut Big, k: &mut i16) {
348 | |     // The bit length is an estimate of the base two logarithm, and log(u / v) = log(u) - log(v).
349 | |     // The estimate is off by at most 1, but always an under-estimate, so the error on log(u)
350 | |     // and log(v) are of the same sign and cancel out (if both are large). Therefore the error
...   |
382 | |     v.mul_pow2(v_shift as usize);
    | |_^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/algorithm.rs:385:1
    |
    |
385 | / fn underflow<T: RawFloat>(x: Big, v: Big, rem: Big) -> T {
386 | |     if x < Big::from_u64(T::MIN_SIG) {
387 | |         let q = num::to_u64(&x);
388 | |         let z = rawfp::encode_subnormal(q);
415 | |     }
416 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/algorithm.rs:419:1
    |
    |
419 | / fn round_by_remainder<T: RawFloat>(v: Big, r: Big, q: u64, z: T) -> T {
420 | |     let mut v_minus_r = v;
421 | |     v_minus_r.sub(&r);
422 | |     if r < v_minus_r {
430 | |     }
431 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: module is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:95:1
   |
   |
95 | mod num;
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/num.rs:5:1
  |
  |
5 | use crate::cmp::Ordering::{self, Equal, Greater, Less};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/num.rs:5:28
  |
  |
5 | use crate::cmp::Ordering::{self, Equal, Greater, Less};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/num.rs:5:34
  |
  |
5 | use crate::cmp::Ordering::{self, Equal, Greater, Less};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/num.rs:5:41
  |
  |
5 | use crate::cmp::Ordering::{self, Equal, Greater, Less};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/num.rs:5:50
  |
  |
5 | use crate::cmp::Ordering::{self, Equal, Greater, Less};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/num.rs:7:1
  |
  |
7 | pub use crate::num::bignum::Big32x40 as Big;
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
  --> library/core/src/num/dec2flt/num.rs:11:1
   |
   |
11 | / pub fn compare_with_half_ulp(f: &Big, ones_place: usize) -> Ordering {
12 | |     if ones_place == 0 {
13 | |         return Less;
...  |
27 | |     Equal
28 | | }
   | |_^
   | |_^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
