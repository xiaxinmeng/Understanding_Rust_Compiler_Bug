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
   --> library/core/src/cmp.rs:775:16
    |
775 | pub trait Ord: ~const Eq + ~const PartialOrd<Self> {
    |                ^^^^^^^^^ required by this bound in `cmp::Ord`

error[E0277]: can't compare `alignment::Alignment` with `alignment::Alignment` in const contexts
     |
183  | impl const cmp::PartialOrd for Alignment {
     |            ^^^^^^^^^^^^^^^ no implementation for `alignment::Alignment == alignment::Alignment`
     |
     |
     = help: the trait `~const cmp::PartialEq` is not implemented for `alignment::Alignment`
note: the trait `cmp::PartialEq` is implemented for `alignment::Alignment`, but that implementation is not `const`
     |
183  | impl const cmp::PartialOrd for Alignment {
     |            ^^^^^^^^^^^^^^^
note: required by a bound in `cmp::PartialOrd`
note: required by a bound in `cmp::PartialOrd`
    --> library/core/src/cmp.rs:1083:43
     |
1083 | pub trait PartialOrd<Rhs: ?Sized = Self>: ~const PartialEq<Rhs> {
     |                                           ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `cmp::PartialOrd`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:00:06
