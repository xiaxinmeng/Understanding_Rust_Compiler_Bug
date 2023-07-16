plain
   Compiling cc v1.0.73
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: no rules expected the token `!`
     |
     |
257  |             core::intrinsics::assert_unsafe_precondition!(!T::input_is_zero(&v));
     |                                                           ^ no rules expected this token in macro call
    ::: library/core/src/intrinsics.rs:2205:1
     |
2205 | macro_rules! assert_unsafe_precondition {
     | --------------------------------------- when calling this macro
