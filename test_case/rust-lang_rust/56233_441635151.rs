plain
travis_time:end:01e387b0:start=1543237076849265517,finish=1543237077852153167,duration=1002887650
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:17:58]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:18:04]    |
[00:18:04] 15 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:18:04]    |
[00:18:04] 3  | / simd_i_ty! {
[00:18:04] 4  | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:18:04] 5  | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:18:04] 6  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 7  | |     /// A 128-bit vector with 16 `i8` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:18:04]    |
[00:18:04] 35 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:18:04]    |
[00:18:04] 3  | / simd_i_ty! {
[00:18:04] 4  | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:18:04] 5  | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:18:04] 6  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 7  | |     /// A 128-bit vector with 16 `i8` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:18:04]    |
[00:18:04] 53 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:18:04]    |
[00:18:04] 3  | / simd_i_ty! {
[00:18:04] 4  | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:18:04] 5  | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:18:04] 6  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 7  | |     /// A 128-bit vector with 16 `i8` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:18:04]    |
[00:18:04] 67 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:18:04]    |
[00:18:04] 3  | / simd_i_ty! {
[00:18:04] 4  | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:18:04] 5  | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:18:04] 6  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 7  | |     /// A 128-bit vector with 16 `i8` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:18:04]    |
[00:18:04] 87 |                       use slice::SliceExt;
[00:18:04]    |                           ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:18:04]    |
[00:18:04] 3  | / simd_i_ty! {
[00:18:04] 4  | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:18:04] 5  | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:18:04] 6  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 7  | |     /// A 128-bit vector with 16 `i8` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:18:04]     |
[00:18:04] 105 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:18:04]     |
[00:18:04] 3   | / simd_i_ty! {
[00:18:04] 4   | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:18:04] 5   | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:18:04] 6   | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 7   | |     /// A 128-bit vector with 16 `i8` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:18:04]     |
[00:18:04] 120 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:18:04]     |
[00:18:04] 3   | / simd_i_ty! {
[00:18:04] 4   | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:18:04] 5   | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:18:04] 6   | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 7   | |     /// A 128-bit vector with 16 `i8` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:18:04]     |
[00:18:04] 134 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:18:04]     |
[00:18:04] 3   | / simd_i_ty! {
[00:18:04] 4   | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:18:04] 5   | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:18:04] 6   | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 7   | |     /// A 128-bit vector with 16 `i8` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:18:04]    |
[00:18:04] 15 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:18:04]    |
[00:18:04] 10 | / simd_u_ty! {
[00:18:04] 11 | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:18:04] 12 | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:18:04] 13 | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 14 | |     /// A 128-bit vector with 16 `u8` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:18:04]    |
[00:18:04] 35 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:18:04]    |
[00:18:04] 10 | / simd_u_ty! {
[00:18:04] 11 | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:18:04] 12 | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:18:04] 13 | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 14 | |     /// A 128-bit vector with 16 `u8` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:18:04]    |
[00:18:04] 53 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:18:04]    |
[00:18:04] 10 | / simd_u_ty! {
[00:18:04] 11 | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:18:04] 12 | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:18:04] 13 | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 14 | |     /// A 128-bit vector with 16 `u8` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:18:04]    |
[00:18:04] 67 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:18:04]    |
[00:18:04] 10 | / simd_u_ty! {
[00:18:04] 11 | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:18:04] 12 | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:18:04] 13 | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 14 | |     /// A 128-bit vector with 16 `u8` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:18:04]    |
[00:18:04] 87 |                       use slice::SliceExt;
[00:18:04]    |                           ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:18:04]    |
[00:18:04] 10 | / simd_u_ty! {
[00:18:04] 11 | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:18:04] 12 | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:18:04] 13 | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 14 | |     /// A 128-bit vector with 16 `u8` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:18:04]     |
[00:18:04] 105 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:18:04]     |
[00:18:04] 10  | / simd_u_ty! {
[00:18:04] 11  | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:18:04] 12  | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:18:04] 13  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 14  | |     /// A 128-bit vector with 16 `u8` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:18:04]     |
[00:18:04] 120 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:18:04]     |
[00:18:04] 10  | / simd_u_ty! {
[00:18:04] 11  | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:18:04] 12  | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:18:04] 13  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 14  | |     /// A 128-bit vector with 16 `u8` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:18:04]     |
[00:18:04] 134 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:18:04]     |
[00:18:04] 10  | / simd_u_ty! {
[00:18:04] 11  | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:18:04] 12  | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:18:04] 13  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:18:04] 14  | |     /// A 128-bit vector with 16 `u8` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:18:04]    |
[00:18:04] 15 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:18:04]    |
[00:18:04] 24 | / simd_i_ty! {
[00:18:04] 25 | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:18:04] 26 | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:18:04] 27 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 28 | |     /// A 128-bit vector with 8 `i16` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:18:04]    |
[00:18:04] 35 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:18:04]    |
[00:18:04] 24 | / simd_i_ty! {
[00:18:04] 25 | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:18:04] 26 | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:18:04] 27 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 28 | |     /// A 128-bit vector with 8 `i16` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:18:04]    |
[00:18:04] 53 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:18:04]    |
[00:18:04] 24 | / simd_i_ty! {
[00:18:04] 25 | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:18:04] 26 | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:18:04] 27 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 28 | |     /// A 128-bit vector with 8 `i16` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:18:04]    |
[00:18:04] 67 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:18:04]    |
[00:18:04] 24 | / simd_i_ty! {
[00:18:04] 25 | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:18:04] 26 | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:18:04] 27 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 28 | |     /// A 128-bit vector with 8 `i16` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:18:04]    |
[00:18:04] 87 |                       use slice::SliceExt;
[00:18:04]    |                           ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:18:04]    |
[00:18:04] 24 | / simd_i_ty! {
[00:18:04] 25 | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:18:04] 26 | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:18:04] 27 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 28 | |     /// A 128-bit vector with 8 `i16` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:18:04]     |
[00:18:04] 105 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:18:04]     |
[00:18:04] 24  | / simd_i_ty! {
[00:18:04] 25  | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:18:04] 26  | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:18:04] 27  | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 28  | |     /// A 128-bit vector with 8 `i16` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:18:04]     |
[00:18:04] 120 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:18:04]     |
[00:18:04] 24  | / simd_i_ty! {
[00:18:04] 25  | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:18:04] 26  | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:18:04] 27  | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 28  | |     /// A 128-bit vector with 8 `i16` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:18:04]     |
[00:18:04] 134 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:18:04]     |
[00:18:04] 24  | / simd_i_ty! {
[00:18:04] 25  | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:18:04] 26  | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:18:04] 27  | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 28  | |     /// A 128-bit vector with 8 `i16` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:18:04]    |
[00:18:04] 15 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:18:04]    |
[00:18:04] 31 | / simd_u_ty! {
[00:18:04] 32 | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:18:04] 33 | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:18:04] 34 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 35 | |     /// A 128-bit vector with 8 `u16` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:18:04]    |
[00:18:04] 35 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:18:04]    |
[00:18:04] 31 | / simd_u_ty! {
[00:18:04] 32 | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:18:04] 33 | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:18:04] 34 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 35 | |     /// A 128-bit vector with 8 `u16` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:18:04]    |
[00:18:04] 53 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:18:04]    |
[00:18:04] 31 | / simd_u_ty! {
[00:18:04] 32 | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:18:04] 33 | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:18:04] 34 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 35 | |     /// A 128-bit vector with 8 `u16` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:18:04]    |
[00:18:04] 67 |                   use slice::SliceExt;
[00:18:04]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:18:04]    |
[00:18:04] 31 | / simd_u_ty! {
[00:18:04] 32 | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:18:04] 33 | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:18:04] 34 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 35 | |     /// A 128-bit vector with 8 `u16` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:18:04]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:18:04]    |
[00:18:04] 87 |                       use slice::SliceExt;
[00:18:04]    |                           ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]    | 
[00:18:04]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:18:04]    |
[00:18:04] 31 | / simd_u_ty! {
[00:18:04] 32 | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:18:04] 33 | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:18:04] 34 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 35 | |     /// A 128-bit vector with 8 `u16` lanes.
[00:18:04]    | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:18:04]     |
[00:18:04] 105 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:18:04]     |
[00:18:04] 31  | / simd_u_ty! {
[00:18:04] 32  | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:18:04] 33  | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:18:04] 34  | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 35  | |     /// A 128-bit vector with 8 `u16` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:18:04]     |
[00:18:04] 120 |                   use slice::SliceExt;
[00:18:04]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:18:04]     | 
[00:18:04]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:18:04]     |
[00:18:04] 31  | / simd_u_ty! {
[00:18:04] 32  | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:18:04] 33  | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:18:04] 34  | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:18:04] 35  | |     /// A 128-bit vector with 8 `u16` lanes.
[00:18:04]     | |_- in this macro invocation
[00:18:04] 
[00:18:04] error[E0432]: unresolved import `slice::SliceExt`
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:18:04]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:18:04]     |
