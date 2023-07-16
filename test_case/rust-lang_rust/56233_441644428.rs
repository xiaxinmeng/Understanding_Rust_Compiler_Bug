plain
travis_time:end:0cb28db6:start=1543238767752384310,finish=1543238824646921054,duration=56894536744
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:19:53]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:19:53]    |
[00:19:53] 15 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:19:53]    |
[00:19:53] 3  | / simd_i_ty! {
[00:19:53] 4  | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:19:53] 5  | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:19:53] 6  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 7  | |     /// A 128-bit vector with 16 `i8` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:19:53]    |
[00:19:53] 35 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:19:53]    |
[00:19:53] 3  | / simd_i_ty! {
[00:19:53] 4  | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:19:53] 5  | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:19:53] 6  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 7  | |     /// A 128-bit vector with 16 `i8` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:19:53]    |
[00:19:53] 53 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:19:53]    |
[00:19:53] 3  | / simd_i_ty! {
[00:19:53] 4  | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:19:53] 5  | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:19:53] 6  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 7  | |     /// A 128-bit vector with 16 `i8` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:19:53]    |
[00:19:53] 67 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:19:53]    |
[00:19:53] 3  | / simd_i_ty! {
[00:19:53] 4  | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:19:53] 5  | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:19:53] 6  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 7  | |     /// A 128-bit vector with 16 `i8` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:19:53]    |
[00:19:53] 87 |                       use slice::SliceExt;
[00:19:53]    |                           ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:19:53]    |
[00:19:53] 3  | / simd_i_ty! {
[00:19:53] 4  | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:19:53] 5  | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:19:53] 6  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 7  | |     /// A 128-bit vector with 16 `i8` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:19:53]     |
[00:19:53] 105 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:19:53]     |
[00:19:53] 3   | / simd_i_ty! {
[00:19:53] 4   | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:19:53] 5   | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:19:53] 6   | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 7   | |     /// A 128-bit vector with 16 `i8` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:19:53]     |
[00:19:53] 120 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:19:53]     |
[00:19:53] 3   | / simd_i_ty! {
[00:19:53] 4   | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:19:53] 5   | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:19:53] 6   | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 7   | |     /// A 128-bit vector with 16 `i8` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:19:53]     |
[00:19:53] 134 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:3:1
[00:19:53]     |
[00:19:53] 3   | / simd_i_ty! {
[00:19:53] 4   | |     i8x16: 16, i8, m8x16, i8x16_tests, test_v128 |
[00:19:53] 5   | |     i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8  |
[00:19:53] 6   | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 7   | |     /// A 128-bit vector with 16 `i8` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:19:53]    |
[00:19:53] 15 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:19:53]    |
[00:19:53] 10 | / simd_u_ty! {
[00:19:53] 11 | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:19:53] 12 | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:19:53] 13 | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 14 | |     /// A 128-bit vector with 16 `u8` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:19:53]    |
[00:19:53] 35 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:19:53]    |
[00:19:53] 10 | / simd_u_ty! {
[00:19:53] 11 | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:19:53] 12 | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:19:53] 13 | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 14 | |     /// A 128-bit vector with 16 `u8` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:19:53]    |
[00:19:53] 53 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:19:53]    |
[00:19:53] 10 | / simd_u_ty! {
[00:19:53] 11 | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:19:53] 12 | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:19:53] 13 | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 14 | |     /// A 128-bit vector with 16 `u8` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:19:53]    |
[00:19:53] 67 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:19:53]    |
[00:19:53] 10 | / simd_u_ty! {
[00:19:53] 11 | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:19:53] 12 | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:19:53] 13 | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 14 | |     /// A 128-bit vector with 16 `u8` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:19:53]    |
[00:19:53] 87 |                       use slice::SliceExt;
[00:19:53]    |                           ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:19:53]    |
[00:19:53] 10 | / simd_u_ty! {
[00:19:53] 11 | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:19:53] 12 | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:19:53] 13 | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 14 | |     /// A 128-bit vector with 16 `u8` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:19:53]     |
[00:19:53] 105 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:19:53]     |
[00:19:53] 10  | / simd_u_ty! {
[00:19:53] 11  | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:19:53] 12  | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:19:53] 13  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 14  | |     /// A 128-bit vector with 16 `u8` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:19:53]     |
[00:19:53] 120 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:19:53]     |
[00:19:53] 10  | / simd_u_ty! {
[00:19:53] 11  | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:19:53] 12  | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:19:53] 13  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 14  | |     /// A 128-bit vector with 16 `u8` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:19:53]     |
[00:19:53] 134 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:10:1
[00:19:53]     |
[00:19:53] 10  | / simd_u_ty! {
[00:19:53] 11  | |     u8x16: 16, u8, m8x16, u8x16_tests, test_v128 |
[00:19:53] 12  | |     u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8 |
[00:19:53] 13  | |     x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
[00:19:53] 14  | |     /// A 128-bit vector with 16 `u8` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:19:53]    |
[00:19:53] 15 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:19:53]    |
[00:19:53] 24 | / simd_i_ty! {
[00:19:53] 25 | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:19:53] 26 | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:19:53] 27 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 28 | |     /// A 128-bit vector with 8 `i16` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:19:53]    |
[00:19:53] 35 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:19:53]    |
[00:19:53] 24 | / simd_i_ty! {
[00:19:53] 25 | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:19:53] 26 | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:19:53] 27 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 28 | |     /// A 128-bit vector with 8 `i16` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:19:53]    |
[00:19:53] 53 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:19:53]    |
[00:19:53] 24 | / simd_i_ty! {
[00:19:53] 25 | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:19:53] 26 | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:19:53] 27 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 28 | |     /// A 128-bit vector with 8 `i16` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:19:53]    |
[00:19:53] 67 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:19:53]    |
[00:19:53] 24 | / simd_i_ty! {
[00:19:53] 25 | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:19:53] 26 | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:19:53] 27 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 28 | |     /// A 128-bit vector with 8 `i16` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:19:53]    |
[00:19:53] 87 |                       use slice::SliceExt;
[00:19:53]    |                           ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:19:53]    |
[00:19:53] 24 | / simd_i_ty! {
[00:19:53] 25 | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:19:53] 26 | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:19:53] 27 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 28 | |     /// A 128-bit vector with 8 `i16` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:19:53]     |
[00:19:53] 105 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:19:53]     |
[00:19:53] 24  | / simd_i_ty! {
[00:19:53] 25  | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:19:53] 26  | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:19:53] 27  | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 28  | |     /// A 128-bit vector with 8 `i16` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:19:53]     |
[00:19:53] 120 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:19:53]     |
[00:19:53] 24  | / simd_i_ty! {
[00:19:53] 25  | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:19:53] 26  | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:19:53] 27  | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 28  | |     /// A 128-bit vector with 8 `i16` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:19:53]     |
[00:19:53] 134 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:24:1
[00:19:53]     |
[00:19:53] 24  | / simd_i_ty! {
[00:19:53] 25  | |     i16x8: 8, i16, m16x8, i16x8_tests, test_v128 |
[00:19:53] 26  | |     i16, i16, i16, i16, i16, i16, i16, i16 |
[00:19:53] 27  | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 28  | |     /// A 128-bit vector with 8 `i16` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:15:21
[00:19:53]    |
[00:19:53] 15 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:19:53]    |
[00:19:53] 31 | / simd_u_ty! {
[00:19:53] 32 | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:19:53] 33 | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:19:53] 34 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 35 | |     /// A 128-bit vector with 8 `u16` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:35:21
[00:19:53]    |
[00:19:53] 35 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:19:53]    |
[00:19:53] 31 | / simd_u_ty! {
[00:19:53] 32 | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:19:53] 33 | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:19:53] 34 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 35 | |     /// A 128-bit vector with 8 `u16` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:53:21
[00:19:53]    |
[00:19:53] 53 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:19:53]    |
[00:19:53] 31 | / simd_u_ty! {
[00:19:53] 32 | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:19:53] 33 | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:19:53] 34 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 35 | |     /// A 128-bit vector with 8 `u16` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:67:21
[00:19:53]    |
[00:19:53] 67 |                   use slice::SliceExt;
[00:19:53]    |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:19:53]    |
[00:19:53] 31 | / simd_u_ty! {
[00:19:53] 32 | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:19:53] 33 | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:19:53] 34 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 35 | |     /// A 128-bit vector with 8 `u16` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:19:53]   --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:87:25
[00:19:53]    |
[00:19:53] 87 |                       use slice::SliceExt;
[00:19:53]    |                           ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]    | 
[00:19:53]   ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:19:53]    |
[00:19:53] 31 | / simd_u_ty! {
[00:19:53] 32 | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:19:53] 33 | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:19:53] 34 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 35 | |     /// A 128-bit vector with 8 `u16` lanes.
[00:19:53]    | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:105:21
[00:19:53]     |
[00:19:53] 105 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:19:53]     |
[00:19:53] 31  | / simd_u_ty! {
[00:19:53] 32  | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:19:53] 33  | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:19:53] 34  | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 35  | |     /// A 128-bit vector with 8 `u16` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:120:21
[00:19:53]     |
[00:19:53] 120 |                   use slice::SliceExt;
[00:19:53]     |                       ^^^^^^^^^^^^^^^ no `SliceExt` in `slice`
[00:19:53]     | 
[00:19:53]    ::: src/libcore/../stdsimd/coresimd/ppsv/v128.rs:31:1
[00:19:53]     |
[00:19:53] 31  | / simd_u_ty! {
[00:19:53] 32  | |     u16x8: 8, u16, m16x8, u16x8_tests, test_v128 |
[00:19:53] 33  | |     u16, u16, u16, u16, u16, u16, u16, u16 |
[00:19:53] 34  | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:19:53] 35  | |     /// A 128-bit vector with 8 `u16` lanes.
[00:19:53]     | |_- in this macro invocation
[00:19:53] 
[00:19:53] error[E0432]: unresolved import `slice::SliceExt`
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:19:53]    --> src/libcore/../stdsimd/coresimd/ppsv/api/load_store.rs:134:21
[00:19:53]     |
