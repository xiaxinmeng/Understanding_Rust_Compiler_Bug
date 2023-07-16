plain
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> compiler/rustc_serialize/tests/leb128.rs:33:44
   |
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
33 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                                            |
   | |                                            expected 2 arguments
...  |
39 | |     };
39 | |     };
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
41 | 
42 |   impl_test_unsigned_leb128!(test_u16_leb128, write_u16_leb128, read_u16_leb128, u16);
   |
note: function defined here
  --> /checkout/compiler/rustc_serialize/src/leb128.rs:87:28
   |
   |
87 | impl_read_unsigned_leb128!(read_u16_leb128, u16);
   |                            ^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:33:21
   |
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
33 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                     ^^^^^^^^^^^^^^^^^^^^   ---------------------------------- this expression has type `u16`
   | |                     expected `u16`, found tuple
...  |
39 | |     };
40 | | }
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
41 | 
42 |   impl_test_unsigned_leb128!(test_u16_leb128, write_u16_leb128, read_u16_leb128, u16);
   |
   = note: expected type `u16`
             found tuple `(_, _)`


error[E0277]: the trait bound `u16: Neg` is not satisfied
  --> compiler/rustc_serialize/tests/leb128.rs:21:18
7  | / macro_rules! impl_test_unsigned_leb128 {
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
21 | |                 (-500..500).map(|i| (i as $int_ty).wrapping_mul(0x12345789ABCDEFu64 as $int_ty)),
   | |                  ^^^^ the trait `Neg` is not implemented for `u16`
39 | |     };
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
41 | 
41 | 
42 |   impl_test_unsigned_leb128!(test_u16_leb128, write_u16_leb128, read_u16_leb128, u16);

error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> compiler/rustc_serialize/tests/leb128.rs:33:44
   |
   |
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
33 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                                            |
   | |                                            expected 2 arguments
...  |
39 | |     };
39 | |     };
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
43 |   impl_test_unsigned_leb128!(test_u32_leb128, write_u32_leb128, read_u32_leb128, u32);
   |
note: function defined here
  --> /checkout/compiler/rustc_serialize/src/leb128.rs:88:28
   |
   |
88 | impl_read_unsigned_leb128!(read_u32_leb128, u32);
   |                            ^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:33:21
   |
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
33 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                     ^^^^^^^^^^^^^^^^^^^^   ---------------------------------- this expression has type `u32`
   | |                     expected `u32`, found tuple
...  |
39 | |     };
40 | | }
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
43 |   impl_test_unsigned_leb128!(test_u32_leb128, write_u32_leb128, read_u32_leb128, u32);
   |
   = note: expected type `u32`
             found tuple `(_, _)`


error[E0277]: the trait bound `u32: Neg` is not satisfied
  --> compiler/rustc_serialize/tests/leb128.rs:21:18
7  | / macro_rules! impl_test_unsigned_leb128 {
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
21 | |                 (-500..500).map(|i| (i as $int_ty).wrapping_mul(0x12345789ABCDEFu64 as $int_ty)),
   | |                  ^^^^ the trait `Neg` is not implemented for `u32`
39 | |     };
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
...
43 |   impl_test_unsigned_leb128!(test_u32_leb128, write_u32_leb128, read_u32_leb128, u32);

error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> compiler/rustc_serialize/tests/leb128.rs:33:44
   |
   |
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
33 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                                            |
   | |                                            expected 2 arguments
...  |
39 | |     };
39 | |     };
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
44 |   impl_test_unsigned_leb128!(test_u64_leb128, write_u64_leb128, read_u64_leb128, u64);
   |
note: function defined here
  --> /checkout/compiler/rustc_serialize/src/leb128.rs:89:28
   |
   |
89 | impl_read_unsigned_leb128!(read_u64_leb128, u64);
   |                            ^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:33:21
   |
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
33 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                     ^^^^^^^^^^^^^^^^^^^^   ---------------------------------- this expression has type `u64`
   | |                     expected `u64`, found tuple
...  |
39 | |     };
40 | | }
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
44 |   impl_test_unsigned_leb128!(test_u64_leb128, write_u64_leb128, read_u64_leb128, u64);
   |
   = note: expected type `u64`
             found tuple `(_, _)`


error[E0277]: the trait bound `u64: Neg` is not satisfied
  --> compiler/rustc_serialize/tests/leb128.rs:21:18
7  | / macro_rules! impl_test_unsigned_leb128 {
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
21 | |                 (-500..500).map(|i| (i as $int_ty).wrapping_mul(0x12345789ABCDEFu64 as $int_ty)),
   | |                  ^^^^ the trait `Neg` is not implemented for `u64`
39 | |     };
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
...
44 |   impl_test_unsigned_leb128!(test_u64_leb128, write_u64_leb128, read_u64_leb128, u64);

error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> compiler/rustc_serialize/tests/leb128.rs:33:44
   |
   |
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
33 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                                            |
   | |                                            expected 2 arguments
...  |
39 | |     };
39 | |     };
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
45 |   impl_test_unsigned_leb128!(test_u128_leb128, write_u128_leb128, read_u128_leb128, u128);
   |
note: function defined here
  --> /checkout/compiler/rustc_serialize/src/leb128.rs:90:28
   |
   |
90 | impl_read_unsigned_leb128!(read_u128_leb128, u128);
   |                            ^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:33:21
   |
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
33 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                     ^^^^^^^^^^^^^^^^^^^^   ---------------------------------- this expression has type `u128`
   | |                     expected `u128`, found tuple
...  |
39 | |     };
40 | | }
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
45 |   impl_test_unsigned_leb128!(test_u128_leb128, write_u128_leb128, read_u128_leb128, u128);
   |
   = note: expected type `u128`
             found tuple `(_, _)`


error[E0277]: the trait bound `u128: Neg` is not satisfied
  --> compiler/rustc_serialize/tests/leb128.rs:21:18
7  | / macro_rules! impl_test_unsigned_leb128 {
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
21 | |                 (-500..500).map(|i| (i as $int_ty).wrapping_mul(0x12345789ABCDEFu64 as $int_ty)),
   | |                  ^^^^ the trait `Neg` is not implemented for `u128`
39 | |     };
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
...
45 |   impl_test_unsigned_leb128!(test_u128_leb128, write_u128_leb128, read_u128_leb128, u128);

error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> compiler/rustc_serialize/tests/leb128.rs:33:44
   |
   |
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
33 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                                            |
   | |                                            expected 2 arguments
...  |
39 | |     };
39 | |     };
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
46 |   impl_test_unsigned_leb128!(test_usize_leb128, write_usize_leb128, read_usize_leb128, usize);
   |
note: function defined here
  --> /checkout/compiler/rustc_serialize/src/leb128.rs:91:28
   |
   |
91 | impl_read_unsigned_leb128!(read_usize_leb128, usize);
   |                            ^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:33:21
   |
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
33 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                     ^^^^^^^^^^^^^^^^^^^^   ---------------------------------- this expression has type `usize`
   | |                     expected `usize`, found tuple
...  |
39 | |     };
40 | | }
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
46 |   impl_test_unsigned_leb128!(test_usize_leb128, write_usize_leb128, read_usize_leb128, usize);
   |
   = note: expected type `usize`
             found tuple `(_, _)`


error[E0277]: the trait bound `usize: Neg` is not satisfied
  --> compiler/rustc_serialize/tests/leb128.rs:21:18
7  | / macro_rules! impl_test_unsigned_leb128 {
7  | / macro_rules! impl_test_unsigned_leb128 {
8  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
9  | |         #[test]
10 | |         fn $test_name() {
...  |
21 | |                 (-500..500).map(|i| (i as $int_ty).wrapping_mul(0x12345789ABCDEFu64 as $int_ty)),
   | |                  ^^^^ the trait `Neg` is not implemented for `usize`
39 | |     };
40 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
...
46 |   impl_test_unsigned_leb128!(test_usize_leb128, write_usize_leb128, read_usize_leb128, usize);

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_serialize/tests/leb128.rs:80:44
    |
    |
48  | / macro_rules! impl_test_signed_leb128 {
49  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
50  | |         #[test]
51  | |         fn $test_name() {
...   |
80  | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
    | |                                            |
    | |                                            expected 2 arguments
...   |
86  | |     };
86  | |     };
87  | | }
    | |_- in this expansion of `impl_test_signed_leb128!`
88  | 
89  |   impl_test_signed_leb128!(test_i16_leb128, write_i16_leb128, read_i16_leb128, i16);
    |
note: function defined here
   --> /checkout/compiler/rustc_serialize/src/leb128.rs:163:26
    |
    |
163 | impl_read_signed_leb128!(read_i16_leb128, i16);
    |                          ^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:80:21
   |
48 | / macro_rules! impl_test_signed_leb128 {
49 | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
50 | |         #[test]
51 | |         fn $test_name() {
...  |
80 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                     ^^^^^^^^^^^^^^^^^^^^   ---------------------------------- this expression has type `i16`
   | |                     expected `i16`, found tuple
...  |
86 | |     };
87 | | }
87 | | }
   | |_- in this expansion of `impl_test_signed_leb128!`
88 | 
89 |   impl_test_signed_leb128!(test_i16_leb128, write_i16_leb128, read_i16_leb128, i16);
   |
   = note: expected type `i16`
             found tuple `(_, _)`


error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_serialize/tests/leb128.rs:80:44
    |
48  | / macro_rules! impl_test_signed_leb128 {
49  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
50  | |         #[test]
51  | |         fn $test_name() {
...   |
80  | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
    | |                                            |
    | |                                            expected 2 arguments
...   |
86  | |     };
86  | |     };
87  | | }
    | |_- in this expansion of `impl_test_signed_leb128!`
...
90  |   impl_test_signed_leb128!(test_i32_leb128, write_i32_leb128, read_i32_leb128, i32);
    |
note: function defined here
   --> /checkout/compiler/rustc_serialize/src/leb128.rs:164:26
    |
    |
164 | impl_read_signed_leb128!(read_i32_leb128, i32);
    |                          ^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:80:21
   |
48 | / macro_rules! impl_test_signed_leb128 {
49 | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
50 | |         #[test]
51 | |         fn $test_name() {
...  |
80 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                     ^^^^^^^^^^^^^^^^^^^^   ---------------------------------- this expression has type `i32`
   | |                     expected `i32`, found tuple
...  |
86 | |     };
87 | | }
87 | | }
   | |_- in this expansion of `impl_test_signed_leb128!`
...
90 |   impl_test_signed_leb128!(test_i32_leb128, write_i32_leb128, read_i32_leb128, i32);
   |
   = note: expected type `i32`
             found tuple `(_, _)`


error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_serialize/tests/leb128.rs:80:44
    |
48  | / macro_rules! impl_test_signed_leb128 {
49  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
50  | |         #[test]
51  | |         fn $test_name() {
...   |
80  | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
    | |                                            |
    | |                                            expected 2 arguments
...   |
86  | |     };
86  | |     };
87  | | }
    | |_- in this expansion of `impl_test_signed_leb128!`
...
91  |   impl_test_signed_leb128!(test_i64_leb128, write_i64_leb128, read_i64_leb128, i64);
    |
note: function defined here
   --> /checkout/compiler/rustc_serialize/src/leb128.rs:165:26
    |
    |
165 | impl_read_signed_leb128!(read_i64_leb128, i64);
    |                          ^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:80:21
   |
48 | / macro_rules! impl_test_signed_leb128 {
49 | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
50 | |         #[test]
51 | |         fn $test_name() {
...  |
80 | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
   | |                     ^^^^^^^^^^^^^^^^^^^^   ---------------------------------- this expression has type `i64`
   | |                     expected `i64`, found tuple
...  |
86 | |     };
87 | | }
87 | | }
   | |_- in this expansion of `impl_test_signed_leb128!`
...
91 |   impl_test_signed_leb128!(test_i64_leb128, write_i64_leb128, read_i64_leb128, i64);
   |
   = note: expected type `i64`
             found tuple `(_, _)`


error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_serialize/tests/leb128.rs:80:44
    |
48  | / macro_rules! impl_test_signed_leb128 {
49  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
50  | |         #[test]
51  | |         fn $test_name() {
...   |
80  | |                 let (actual, bytes_read) = $read_fn_name(&stream[position..]);
    | |                                            |
    | |                                            expected 2 arguments
...   |
86  | |     };
86  | |     };
87  | | }
    | |_- in this expansion of `impl_test_signed_leb128!`
...
