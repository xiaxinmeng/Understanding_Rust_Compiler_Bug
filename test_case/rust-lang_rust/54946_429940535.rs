plain
[00:07:25]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:49] error[E0061]: this function takes 1 parameter but 0 parameters were supplied
[00:07:49]    --> librustc/traits/error_reporting.rs:428:28
[00:07:49]     |
[00:07:49] 428 |                     scalar.to_usize().ok()
[00:07:49]     | 
[00:07:49]    ::: librustc/mir/interpret/value.rs:312:5
[00:07:49]     |
[00:07:49]     |
[00:07:49] 312 |     pub fn to_usize(self, cx: impl HasDataLayout) -> EvalResult<'static, u64> {
[00:07:49]     |     ------------------------------------------------------------------------- defined here
known-linux-gnu/stage0-rustc
72532 ./src/llvm/lib
70500 ./obj/build/x86_64-unknown-linux-gnu/native
70300 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
