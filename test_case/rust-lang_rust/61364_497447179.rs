plain
travis_time:end:0e1597bd:start=1559242420714441063,finish=1559242421961996546,duration=1247555483
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:10:16]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:10:16]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:10:17]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:10:17]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:10:25] error[E0723]: can only call other `const fn` within a `const fn`, but `const num::<impl isize>::reverse_bits` is not stable as `const fn`
[00:10:25]     |
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 319 | |     ($($t:ty)*) => ($(
[00:10:25] 320 | |         impl Wrapping<$t> {
[00:10:25] 321 | |             doc_comment! {
[00:10:25] ...   |
[00:10:25] 528 | |                 Wrapping(self.0.reverse_bits())
[00:10:25] ...   |
[00:10:25] 677 | |     )*)
[00:10:25] 678 | | }
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25] 679 | 
[00:10:25] 680 |   wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:10:25]     |
[00:10:25]     = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[00:10:25]     = help: add #![feature(const_fn)] to the crate attributes to enable
[00:10:25] 
[00:10:25] 
[00:10:25] error[E0723]: can only call other `const fn` within a `const fn`, but `const num::<impl i8>::reverse_bits` is not stable as `const fn`
[00:10:25]     |
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 319 | |     ($($t:ty)*) => ($(
[00:10:25] 320 | |         impl Wrapping<$t> {
[00:10:25] 321 | |             doc_comment! {
[00:10:25] ...   |
[00:10:25] 528 | |                 Wrapping(self.0.reverse_bits())
[00:10:25] ...   |
[00:10:25] 677 | |     )*)
[00:10:25] 678 | | }
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25] 679 | 
[00:10:25] 680 |   wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:10:25]     |
[00:10:25]     = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[00:10:25]     = help: add #![feature(const_fn)] to the crate attributes to enable
[00:10:25] 
[00:10:25] 
[00:10:25] error[E0723]: can only call other `const fn` within a `const fn`, but `const num::<impl i16>::reverse_bits` is not stable as `const fn`
[00:10:25]     |
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 319 | |     ($($t:ty)*) => ($(
[00:10:25] 320 | |         impl Wrapping<$t> {
[00:10:25] 321 | |             doc_comment! {
[00:10:25] ...   |
[00:10:25] 528 | |                 Wrapping(self.0.reverse_bits())
[00:10:25] ...   |
[00:10:25] 677 | |     )*)
[00:10:25] 678 | | }
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25] 679 | 
[00:10:25] 680 |   wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:10:25]     |
[00:10:25]     = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[00:10:25]     = help: add #![feature(const_fn)] to the crate attributes to enable
[00:10:25] 
[00:10:25] 
[00:10:25] error[E0723]: can only call other `const fn` within a `const fn`, but `const num::<impl i32>::reverse_bits` is not stable as `const fn`
[00:10:25]     |
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 319 | |     ($($t:ty)*) => ($(
[00:10:25] 320 | |         impl Wrapping<$t> {
[00:10:25] 321 | |             doc_comment! {
[00:10:25] ...   |
[00:10:25] 528 | |                 Wrapping(self.0.reverse_bits())
[00:10:25] ...   |
[00:10:25] 677 | |     )*)
[00:10:25] 678 | | }
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25] 679 | 
[00:10:25] 680 |   wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:10:25]     |
[00:10:25]     = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[00:10:25]     = help: add #![feature(const_fn)] to the crate attributes to enable
[00:10:25] 
[00:10:25] 
[00:10:25] error[E0723]: can only call other `const fn` within a `const fn`, but `const num::<impl i64>::reverse_bits` is not stable as `const fn`
[00:10:25]     |
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 319 | |     ($($t:ty)*) => ($(
[00:10:25] 320 | |         impl Wrapping<$t> {
[00:10:25] 321 | |             doc_comment! {
[00:10:25] ...   |
[00:10:25] 528 | |                 Wrapping(self.0.reverse_bits())
[00:10:25] ...   |
[00:10:25] 677 | |     )*)
[00:10:25] 678 | | }
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25] 679 | 
[00:10:25] 680 |   wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:10:25]     |
[00:10:25]     = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[00:10:25]     = help: add #![feature(const_fn)] to the crate attributes to enable
[00:10:25] 
[00:10:25] 
[00:10:25] error[E0723]: can only call other `const fn` within a `const fn`, but `const num::<impl i128>::reverse_bits` is not stable as `const fn`
[00:10:25]     |
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 318 | / macro_rules! wrapping_int_impl {
[00:10:25] 319 | |     ($($t:ty)*) => ($(
[00:10:25] 320 | |         impl Wrapping<$t> {
[00:10:25] 321 | |             doc_comment! {
[00:10:25] ...   |
[00:10:25] 528 | |                 Wrapping(self.0.reverse_bits())
[00:10:25] ...   |
[00:10:25] 677 | |     )*)
[00:10:25] 678 | | }
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25]     | |_- in this expansion of `wrapping_int_impl!`
[00:10:25] 679 | 
[00:10:25] 680 |   wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:10:25]     |
[00:10:25]     = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[00:10:25]     = help: add #![feature(const_fn)] to the crate attributes to enable
[00:10:25] 
