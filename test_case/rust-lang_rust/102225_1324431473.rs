plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.84
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: the trait bound `alignment::Alignment: ~const cmp::Eq` is not satisfied
    |
174 | impl const cmp::Ord for Alignment {
174 | impl const cmp::Ord for Alignment {
    |            ^^^^^^^^ the trait `~const cmp::Eq` is not implemented for `alignment::Alignment`
    |
note: the trait `cmp::Eq` is implemented for `alignment::Alignment`, but that implementation is not `const`
    |
174 | impl const cmp::Ord for Alignment {
    |            ^^^^^^^^
note: required by a bound in `cmp::Ord`
note: required by a bound in `cmp::Ord`
   --> library/core/src/cmp.rs:767:16
    |
767 | pub trait Ord: ~const Eq + ~const PartialOrd<Self> {
    |                ^^^^^^^^^ required by this bound in `cmp::Ord`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:02:04
