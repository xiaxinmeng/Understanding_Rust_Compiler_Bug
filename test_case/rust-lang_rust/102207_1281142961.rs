plain
   Compiling cc v1.0.73
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: to use a constant of type `alignment::Alignment` in a pattern, `alignment::Alignment` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
21 |     matches!(a, Alignment::MIN)

error: unreachable pattern
   --> library/core/src/macros/mod.rs:346:13
    |
    |
342 | / macro_rules! matches {
343 | |     ($expression:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
344 | |         match $expression {
345 | |             $( $pattern )|+ $( if $guard )? => true,
    | |             ^ unreachable pattern
347 | |         }
348 | |     };
349 | | }
349 | | }
    | |_- in this expansion of `matches!`
    |
   ::: library/core/src/ptr/alignment.rs:21:17
    |
21  |       matches!(a, Alignment::MIN)
    |       |           |
    |       |           matches any value
    |       in this macro invocation
    |
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:00:10
