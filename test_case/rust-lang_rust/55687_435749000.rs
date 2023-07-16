plain
travis_time:end:21206dd8:start=1541389148107204406,finish=1541389205107126493,duration=56999922087
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:21:28]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:21:28]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:21:29]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:21:29]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:21:31] error[E0284]: type annotations required: cannot resolve `<T as ops::arith::Mul>::Output == T`
[00:21:31]   --> libcore/num/flt2dec/decoder.rs:69:1
[00:21:31]    |
[00:21:31] 69 | / pub fn decode<T: DecodableFloat>(v: T) -> (/*negative?*/ bool, FullDecoded) {
[00:21:31] 70 | |     let (mant, exp, sign) = v.integer_decode();
[00:21:31] 71 | |     let even = (mant & 1) == 0;
[00:21:31] 72 | |     let decoded = match v.classify() {
[00:21:31] ...  |
[00:21:31] 97 | |     (sign < 0, decoded)
[00:21:31]    | |_^
[00:21:31]    |
[00:21:31]    |
[00:21:31] note: required by `num::flt2dec::decoder::DecodableFloat`
[00:21:31]   --> libcore/num/flt2dec/decoder.rs:54:1
[00:21:31]    |
[00:21:31] 54 | pub trait DecodableFloat: RawFloat + Copy {
[00:21:31] 
[00:21:31] error[E0277]: the size for values of type `Self` cannot be known at compilation time
[00:21:31] error[E0277]: the size for values of type `Self` cannot be known at compilation time
[00:21:31]   --> libcore/num/dec2flt/num.rs:49:1
[00:21:31]    |
[00:21:31] 49 | / pub fn from_str_unchecked<'a, T>(bytes: T) -> u64 where T : IntoIterator<Item=&'a u8> {
[00:21:31] 50 | |     let mut result = 0;
[00:21:31] 51 | |     for &c in bytes {
[00:21:31] 52 | |         result = result * 10 + (c - b'0') as u64;
[00:21:31] 54 | |     result
[00:21:31] 55 | | }
[00:21:31]    | |_^ doesn't have a size known at compile-time
[00:21:31]    |
[00:21:31]    |
[00:21:31]    = help: the trait `marker::Sized` is not implemented for `Self`
[00:21:31]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:21:31]    = help: consider adding a `where Self: marker::Sized` bound
[00:21:31]    = note: required because of the requirements on the impl of `iter::traits::IntoIterator` for `Self`
[00:21:31] error[E0277]: `Self` is not an iterator
[00:21:31] error[E0277]: `Self` is not an iterator
[00:21:31]   --> libcore/num/dec2flt/num.rs:49:1
[00:21:31]    |
[00:21:31] 49 | / pub fn from_str_unchecked<'a, T>(bytes: T) -> u64 where T : IntoIterator<Item=&'a u8> {
[00:21:31] 50 | |     let mut result = 0;
[00:21:31] 51 | |     for &c in bytes {
[00:21:31] 52 | |         result = result * 10 + (c - b'0') as u64;
[00:21:31] 54 | |     result
[00:21:3] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:21:31] 
[00:21:31] 
[00:21:31] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:21:31] note: some of the compiler flags provided by cargo are hidden
[00:21:31] 
[00:21:31] error: Could not compile `core`.
[00:21:31] warning: build failed, waiting for other jobs to finish...
---
151412 ./src/tools/clang
150256 ./obj/build/bootstrap/debug/incremental
149112 ./src/llvm-emscripten/test
134672 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6dkhs26j5-dz49aw-1ixyktdm53mj
107892 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
94832 ./obj/build/x86_64-unknown-linux-gnu/stage1
94812 ./
