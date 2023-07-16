plain
    Checking chalk-ir v0.75.0
    Checking tracing v0.1.28
    Checking tracing-subscriber v0.3.3
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0277]: can't compare `MyIdx` with `MyIdx`
    |
69  |     / macro_rules! newtype_index {
70  |     |     // ---- public rules ----
71  |     |
---
94  | |   |                           $($tokens)+);
    | |___|______________________________________- in this macro invocation (#2)
...       |
208 |     |         impl ::std::iter::Step for $type {
    |     |              ^^^^^^^^^^^^^^^^^ no implementation for `MyIdx < MyIdx` and `MyIdx > MyIdx`
386 | /   |         $crate::newtype_index!(
387 | |   |             @derives      []
388 | |   |             @attrs        [$(#[$attrs])*]
389 | |   |             @type         [$type]
389 | |   |             @type         [$type]
...   |   |
392 | |   |             @debug_format [$debug_format]
393 | |   |                           $($tokens)*);
    | |___|______________________________________- in this macro invocation (#3)
...       |
418 | /   |         $crate::newtype_index!(
419 | |   |             @derives      [$($derives,)*]
420 | |   |             @attrs        [$(#[$attrs])*]
421 | |   |             @type         [$type]
...   |   |
424 | |   |             @debug_format [$debug_format]
425 | |   |                           $name = $constant,);
    | |___|_____________________________________________- in this macro invocation (#4)
456 | /   |         $crate::newtype_index!(
457 | |   |             @derives      [$($derives,)*]
458 | |   |             @attrs        [$(#[$attrs])*]
459 | |   |             @type         [$type]
---
    |       in this expansion of `$crate::newtype_index!` (#5)
    |
   ::: compiler/rustc_index/src/vec/tests.rs:2:1
    |
2   |       newtype_index!(struct MyIdx { MAX = 0xFFFF_FFFA });
    |
    |
    = help: the trait `PartialOrd` is not implemented for `MyIdx`
note: required by a bound in `Step`
    |
    |
24  | pub trait Step: Clone + PartialOrd + Sized {
    |                         ^^^^^^^^^^ required by this bound in `Step`

error[E0277]: can't compare `MyIdx` with `MyIdx`
    |
69  |     / macro_rules! newtype_index {
70  |     |     // ---- public rules ----
71  |     |
---
93  | |   |             @debug_format ["{}"]
94  | |   |                           $($tokens)+);
    | |___|______________________________________- in this macro invocation (#2)
...       |
229 |     |         unsafe impl ::std::iter::TrustedStep for $type {}
    |     |                     ^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `MyIdx < MyIdx` and `MyIdx > MyIdx`
386 | /   |         $crate::newtype_index!(
387 | |   |             @derives      []
388 | |   |             @attrs        [$(#[$attrs])*]
389 | |   |             @type         [$type]
389 | |   |             @type         [$type]
...   |   |
392 | |   |             @debug_format [$debug_format]
393 | |   |                           $($tokens)*);
    | |___|______________________________________- in this macro invocation (#3)
...       |
418 | /   |         $crate::newtype_index!(
419 | |   |             @derives      [$($derives,)*]
420 | |   |             @attrs        [$(#[$attrs])*]
421 | |   |             @type         [$type]
...   |   |
424 | |   |             @debug_format [$debug_format]
425 | |   |                           $name = $constant,);
    | |___|_____________________________________________- in this macro invocation (#4)
456 | /   |         $crate::newtype_index!(
457 | |   |             @derives      [$($derives,)*]
458 | |   |             @attrs        [$(#[$attrs])*]
459 | |   |             @type         [$type]
---
    |       in this expansion of `$crate::newtype_index!` (#5)
    |
   ::: compiler/rustc_index/src/vec/tests.rs:2:1
    |
2   |       newtype_index!(struct MyIdx { MAX = 0xFFFF_FFFA });
    |
    |
    = help: the trait `PartialOrd` is not implemented for `MyIdx`
note: required by a bound in `TrustedStep`
    |
    |
74  | pub unsafe trait TrustedStep: Step {}
    |                               ^^^^ required by this bound in `TrustedStep`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_index` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
