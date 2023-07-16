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
  --> library/core/src/num/dec2flt/mod.rs:78:1
   |
78 | use crate::fmt;
78 | use crate::fmt;
   | ^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:79:1
   |
79 | use crate::str::FromStr;
79 | use crate::str::FromStr;
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:81:1
   |
   |
81 | use self::common::{BiasedFp, ByteSlice};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:81:20
   |
   |
81 | use self::common::{BiasedFp, ByteSlice};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:81:30
   |
   |
81 | use self::common::{BiasedFp, ByteSlice};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:82:1
   |
82 | use self::float::RawFloat;
82 | use self::float::RawFloat;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:83:1
   |
83 | use self::lemire::compute_float;
83 | use self::lemire::compute_float;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:84:1
   |
   |
84 | use self::parse::{parse_inf_nan, parse_number};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:84:19
   |
   |
84 | use self::parse::{parse_inf_nan, parse_number};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:84:34
   |
   |
84 | use self::parse::{parse_inf_nan, parse_number};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:85:1
   |
85 | use self::slow::parse_long_mantissa;
85 | use self::slow::parse_long_mantissa;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: module is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:87:1
   |
   |
87 | mod common;
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/common.rs:3:1
  |
3 | use crate::ptr;
3 | use crate::ptr;
  | ^^^^^^^^^^^^^^^
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: trait is private but has a stability attribute
  --> library/core/src/num/dec2flt/common.rs:6:1
   |
   |
6  | / pub(crate) trait ByteSlice: AsRef<[u8]> {
7  | |     unsafe fn first_unchecked(&self) -> u8 {
8  | |         debug_assert!(!self.is_empty());
9  | |         // SAFETY: safe as long as self is not empty
90 | |     }
91 | | }
   | |_^
   |
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: implementation is private but has a stability attribute
  --> library/core/src/num/dec2flt/common.rs:93:1
   |
   |
93 | impl ByteSlice for [u8] {}
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: trait is private but has a stability attribute
   --> library/core/src/num/dec2flt/common.rs:96:1
    |
    |
96  | / pub(crate) trait ByteSliceMut: AsMut<[u8]> {
97  | |     /// Write a 64-bit integer as 8 bytes in little-endian order.
98  | |     unsafe fn write_u64_unchecked(&mut self, value: u64) {
99  | |         debug_assert!(self.as_mut().len() >= 8);
108 | |     }
109 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: implementation is private but has a stability attribute
   --> library/core/src/num/dec2flt/common.rs:111:1
    |
    |
111 | impl ByteSliceMut for [u8] {}
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: implementation is private but has a stability attribute
   --> library/core/src/num/dec2flt/common.rs:119:1
    |
    |
119 | / impl<'a> AsciiStr<'a> {
120 | |     pub fn new(slc: &'a [u8]) -> Self {
121 | |         Self { slc }
...   |
151 | |     }
152 | | }
    | |_^
    | |_^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: implementation is private but has a stability attribute
   --> library/core/src/num/dec2flt/common.rs:154:1
    |
    |
154 | / impl<'a> AsRef<[u8]> for AsciiStr<'a> {
155 | |     #[inline]
156 | |     fn as_ref(&self) -> &[u8] {
157 | |         self.slc
159 | | }
    | |_^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: implementation is private but has a stability attribute
   --> library/core/src/num/dec2flt/common.rs:161:1
    |
    |
161 | impl<'a> ByteSlice for AsciiStr<'a> {}
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/common.rs:165:1
    |
    |
165 | / pub(crate) fn is_8digits(v: u64) -> bool {
166 | |     let a = v.wrapping_add(0x4646_4646_4646_4646);
167 | |     let b = v.wrapping_sub(0x3030_3030_3030_3030);
168 | |     (a | b) & 0x8080_8080_8080_8080 == 0
    | |_^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/common.rs:172:1
    |
    |
172 | / pub(crate) fn parse_digits(s: &mut &[u8], mut f: impl FnMut(u8)) {
173 | |     while let Some(&c) = s.get(0) {
174 | |         let c = c.wrapping_sub(b'0');
175 | |         if c < 10 {
181 | |     }
182 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: module is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:88:1
   |
88 | mod decimal;
88 | mod decimal;
   | ^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/decimal.rs:12:1
   |
   |
12 | use crate::num::dec2flt::common::{is_8digits, parse_digits, ByteSlice, ByteSliceMut};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/decimal.rs:12:35
   |
   |
12 | use crate::num::dec2flt::common::{is_8digits, parse_digits, ByteSlice, ByteSliceMut};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/decimal.rs:12:47
   |
   |
12 | use crate::num::dec2flt::common::{is_8digits, parse_digits, ByteSlice, ByteSliceMut};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/decimal.rs:12:61
   |
   |
12 | use crate::num::dec2flt::common::{is_8digits, parse_digits, ByteSlice, ByteSliceMut};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
  --> library/core/src/num/dec2flt/decimal.rs:12:72
   |
   |
12 | use crate::num::dec2flt::common::{is_8digits, parse_digits, ByteSlice, ByteSliceMut};
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: implementation is private but has a stability attribute
  --> library/core/src/num/dec2flt/decimal.rs:26:1
   |
   |
26 | / impl Default for Decimal {
27 | |     fn default() -> Self {
28 | |         Self { num_digits: 0, decimal_point: 0, truncated: false, digits: [0; Self::MAX_DIGITS] }
30 | | }
   | |_^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: implementation is private but has a stability attribute
   --> library/core/src/num/dec2flt/decimal.rs:32:1
    |
32  | / impl Decimal {
32  | / impl Decimal {
33  | |     /// The maximum number of digits required to unambiguously round a float.
34  | |     ///
35  | |     /// For a double-precision IEEE-754 float, this required 767 digits,
201 | |     }
202 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/decimal.rs:205:1
    |
    |
205 | / pub fn parse_decimal(mut s: &[u8]) -> Decimal {
206 | |     let mut d = Decimal::default();
207 | |     let start = s;
208 | |     s = s.skip_chars(b'0');
271 | |     d
272 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/decimal.rs:274:1
    |
    |
274 | / fn number_of_digits_decimal_left_shift(d: &Decimal, mut shift: usize) -> usize {
275 | |     #[rustfmt::skip]
276 | |     const TABLE: [u16; 65] = [
277 | |         0x0000, 0x0800, 0x0801, 0x0803, 0x1006, 0x1009, 0x100D, 0x1812, 0x1817, 0x181D, 0x2024,
350 | |     num_new_digits
351 | | }
    | |_^
    |
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: module is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:89:1
   |
   |
89 | mod fpu;
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/fpu.rs:4:1
  |
4 | pub use fpu_precision::set_precision;
4 | pub use fpu_precision::set_precision;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: module is private but has a stability attribute
  --> library/core/src/num/dec2flt/fpu.rs:87:1
   |
   |
87 | / mod fpu_precision {
88 | |     pub fn set_precision<T>() {}
   | |_^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
  --> library/core/src/num/dec2flt/fpu.rs:88:5
   |
88 |     pub fn set_precision<T>() {}
88 |     pub fn set_precision<T>() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: module is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:90:1
   |
   |
90 | mod slow;
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/slow.rs:3:1
  |
  |
3 | use crate::num::dec2flt::common::BiasedFp;
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/slow.rs:4:1
  |
  |
4 | use crate::num::dec2flt::decimal::{parse_decimal, Decimal};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/slow.rs:4:36
  |
  |
4 | use crate::num::dec2flt::decimal::{parse_decimal, Decimal};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/slow.rs:4:51
  |
  |
4 | use crate::num::dec2flt::decimal::{parse_decimal, Decimal};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/num/dec2flt/slow.rs:5:1
  |
5 | use crate::num::dec2flt::float::RawFloat;
5 | use crate::num::dec2flt::float::RawFloat;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
   --> library/core/src/num/dec2flt/slow.rs:26:1
    |
    |
26  | / pub(crate) fn parse_long_mantissa<F: RawFloat>(s: &[u8]) -> BiasedFp {
27  | |     const MAX_SHIFT: usize = 60;
28  | |     const NUM_POWERS: usize = 19;
29  | |     const POWERS: [u8; 19] =
...   |
108 | |     BiasedFp { f: mantissa, e: power2 }
    | |_^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: constant is private but has a stability attribute
  --> library/core/src/num/dec2flt/slow.rs:27:5
   |
   |
27 |     const MAX_SHIFT: usize = 60;
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: constant is private but has a stability attribute
  --> library/core/src/num/dec2flt/slow.rs:28:5
   |
28 |     const NUM_POWERS: usize = 19;
28 |     const NUM_POWERS: usize = 19;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: constant is private but has a stability attribute
  --> library/core/src/num/dec2flt/slow.rs:29:5
   |
   |
29 | /     const POWERS: [u8; 19] =
30 | |         [0, 3, 6, 9, 13, 16, 19, 23, 26, 29, 33, 36, 39, 43, 46, 49, 53, 56, 59];
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: module is private but has a stability attribute
  --> library/core/src/num/dec2flt/mod.rs:91:1
   |
91 | mod table;
91 | mod table;
   | ^^^^^^^^^^
   |
---
166 | |     }
167 | | }
    | |_^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: module is private but has a stability attribute
   --> library/core/src/stream/mod.rs:125:1
    |
125 | mod stream;
125 | mod stream;
    | ^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/stream/stream/mod.rs:1:1
  |
1 | use crate::ops::DerefMut;
1 | use crate::ops::DerefMut;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/stream/stream/mod.rs:2:1
  |
2 | use crate::pin::Pin;
2 | use crate::pin::Pin;
  | ^^^^^^^^^^^^^^^^^^^^
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/stream/stream/mod.rs:3:1
  |
  |
3 | use crate::task::{Context, Poll};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/stream/stream/mod.rs:3:19
  |
  |
3 | use crate::task::{Context, Poll};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/stream/stream/mod.rs:3:28
  |
  |
3 | use crate::task::{Context, Poll};
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/sync/atomic.rs:135:5
    |
135 |     v: UnsafeCell<u8>,
135 |     v: UnsafeCell<u8>,
    |     ^^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/sync/atomic.rs:165:5
    |
    |
165 |     p: UnsafeCell<*mut T>,
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
   --> library/core/src/sync/atomic.rs:933:9
    |
933 |         use crate::mem::align_of;
933 |         use crate::mem::align_of;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
 --> library/core/src/fmt/builders.rs:6:5
  |
  |
6 |     buf: &'buf mut (dyn fmt::Write + 'buf),
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
 --> library/core/src/fmt/builders.rs:7:5
  |
  |
7 |     state: &'state mut PadAdapterState,
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
  --> library/core/src/fmt/builders.rs:11:5
   |
11 |     on_newline: bool,
11 |     on_newline: bool,
   |     ^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
  --> library/core/src/fmt/builders.rs:93:5
   |
   |
93 |     fmt: &'a mut fmt::Formatter<'b>,
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
  --> library/core/src/fmt/builders.rs:94:5
   |
94 |     result: fmt::Result,
94 |     result: fmt::Result,
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
  --> library/core/src/fmt/builders.rs:95:5
   |
95 |     has_fields: bool,
95 |     has_fields: bool,
   |     ^^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:282:5
    |
    |
282 |     fmt: &'a mut fmt::Formatter<'b>,
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:283:5
    |
283 |     result: fmt::Result,
283 |     result: fmt::Result,
    |     ^^^^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:284:5
    |
284 |     fields: usize,
284 |     fields: usize,
    |     ^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:285:5
    |
285 |     empty_name: bool,
285 |     empty_name: bool,
    |     ^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:386:5
    |
    |
386 |     fmt: &'a mut fmt::Formatter<'b>,
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:387:5
    |
387 |     result: fmt::Result,
387 |     result: fmt::Result,
    |     ^^^^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:388:5
    |
388 |     has_fields: bool,
388 |     has_fields: bool,
    |     ^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:448:5
    |
    |
448 |     inner: DebugInner<'a, 'b>,
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:578:5
    |
    |
578 |     inner: DebugInner<'a, 'b>,
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:708:5
    |
    |
708 |     fmt: &'a mut fmt::Formatter<'b>,
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:709:5
    |
709 |     result: fmt::Result,
709 |     result: fmt::Result,
    |     ^^^^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:710:5
    |
710 |     has_fields: bool,
710 |     has_fields: bool,
    |     ^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:711:5
    |
711 |     has_key: bool,
711 |     has_key: bool,
    |     ^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/builders.rs:713:5
    |
713 |     state: PadAdapterState,
713 |     state: PadAdapterState,
    |     ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/mod.rs:218:5
    |
218 |     flags: u32,
218 |     flags: u32,
    |     ^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/mod.rs:219:5
    |
219 |     fill: char,
219 |     fill: char,
    |     ^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/mod.rs:220:5
    |
    |
220 |     align: rt::v1::Alignment,
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/mod.rs:221:5
    |
221 |     width: Option<usize>,
221 |     width: Option<usize>,
    |     ^^^^^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/mod.rs:222:5
    |
222 |     precision: Option<usize>,
222 |     precision: Option<usize>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/fmt/mod.rs:224:5
    |
    |
224 |     buf: &'a mut (dyn Write + 'a),
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
    --> library/core/src/fmt/mod.rs:1179:5
     |
1179 |     fill: char,
1179 |     fill: char,
     |     ^^^^^^^^^^
     |
     = help: if it is meant to be private, remove the stability attribute
     = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
    --> library/core/src/fmt/mod.rs:1180:5
     |
1180 |     padding: usize,
1180 |     padding: usize,
     |     ^^^^^^^^^^^^^^
     |
     = help: if it is meant to be private, remove the stability attribute
     = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/hash/mod.rs:576:34
    |
    |
576 | pub struct BuildHasherDefault<H>(marker::PhantomData<H>);
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/slice/memchr.rs:4:1
  |
4 | use crate::cmp;
4 | use crate::cmp;
  | ^^^^^^^^^^^^^^^
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: import is private but has a stability attribute
 --> library/core/src/slice/memchr.rs:5:1
  |
5 | use crate::mem;
5 | use crate::mem;
  | ^^^^^^^^^^^^^^^
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: constant is private but has a stability attribute
 --> library/core/src/slice/memchr.rs:7:1
  |
  |
7 | const LO_U64: u64 = 0x0101010101010101;
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: constant is private but has a stability attribute
 --> library/core/src/slice/memchr.rs:8:1
  |
  |
8 | const HI_U64: u64 = 0x8080808080808080;
  |
  = help: if it is meant to be private, remove the stability attribute
  = help: if it is meant to be private, remove the stability attribute
  = help: or, if it is meant to be public, make it public
error: constant is private but has a stability attribute
  --> library/core/src/slice/memchr.rs:11:1
   |
   |
11 | const LO_USIZE: usize = LO_U64 as usize;
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: constant is private but has a stability attribute
  --> library/core/src/slice/memchr.rs:12:1
   |
   |
12 | const HI_USIZE: usize = HI_U64 as usize;
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: constant is private but has a stability attribute
  --> library/core/src/slice/memchr.rs:13:1
   |
   |
13 | const USIZE_BYTES: usize = mem::size_of::<usize>();
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
  --> library/core/src/slice/memchr.rs:23:1
   |
   |
23 | / fn contains_zero_byte(x: usize) -> bool {
24 | |     x.wrapping_sub(LO_USIZE) & !x & HI_USIZE != 0
   | |_^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
  --> library/core/src/slice/memchr.rs:35:1
   |
   |
35 | / fn repeat_byte(b: u8) -> usize {
36 | |     (b as usize) * (usize::MAX / 255)
   | |_^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: function is private but has a stability attribute
  --> library/core/src/slice/memchr.rs:50:1
   |
   |
50 | / fn memchr_general_case(x: u8, text: &[u8]) -> Option<usize> {
51 | |     // Scan for a single byte value by reading two `usize` words at a time.
52 | |     //
53 | |     // Split `text` in three parts
...  |
90 | |     text[offset..].iter().position(|elt| *elt == x).map(|i| offset + i)
   | |_^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: type alias is private but has a stability attribute
   --> library/core/src/slice/memchr.rs:103:5
    |
    |
103 |     type Chunk = usize;
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
  --> library/core/src/slice/iter.rs:67:5
   |
67 |     ptr: NonNull<T>,
67 |     ptr: NonNull<T>,
   |     ^^^^^^^^^^^^^^^
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
  --> library/core/src/slice/iter.rs:68:5
   |
   |
68 |     end: *const T, // If T is a ZST, this is actually ptr+len.  This encoding is picked so that
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
  --> library/core/src/slice/iter.rs:71:5
   |
   |
71 |     _marker: PhantomData<&'a T>,
   |
   = help: if it is meant to be private, remove the stability attribute
   = help: if it is meant to be private, remove the stability attribute
   = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/slice/iter.rs:186:5
    |
186 |     ptr: NonNull<T>,
186 |     ptr: NonNull<T>,
    |     ^^^^^^^^^^^^^^^
    |
    = help: if it is meant to be private, remove the stability attribute
    = help: or, if it is meant to be public, make it public
error: field is private but has a stability attribute
   --> library/core/src/slice/iter.rs:187:5
    |
    |
187 |     end: *mut T, // If T is a ZST, this is actually ptr+len.  This encoding is picked so that
    |
    = help: if it is meant to be private, remove the stability attribute
