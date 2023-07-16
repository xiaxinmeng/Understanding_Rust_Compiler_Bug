plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.79
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: no rules expected the token `start`
    --> library/core/src/ops/index_range.rs:23:46
     |
23   |         unsafe { assert_unsafe_precondition!(start <= end) };
     |                                              ^^^^^ no rules expected this token in macro call
    ::: library/core/src/intrinsics.rs:2180:1
     |
2180 | macro_rules! assert_unsafe_precondition {
     | --------------------------------------- when calling this macro
     | --------------------------------------- when calling this macro

error: no rules expected the token `self`
     |
     |
296  |             assert_unsafe_precondition!(self.end() <= slice.len());
     |                                         ^^^^ no rules expected this token in macro call
    ::: library/core/src/intrinsics.rs:2180:1
     |
2180 | macro_rules! assert_unsafe_precondition {
     | --------------------------------------- when calling this macro
     | --------------------------------------- when calling this macro

error: no rules expected the token `self`
     |
     |
305  |             assert_unsafe_precondition!(self.end() <= slice.len());
     |                                         ^^^^ no rules expected this token in macro call
    ::: library/core/src/intrinsics.rs:2180:1
     |
2180 | macro_rules! assert_unsafe_precondition {
     | --------------------------------------- when calling this macro
