plain
    |
472 | ///
    | ^^^
    |
    = help: doc comments must come before what they document, maybe a comment was intended with `//`?
   Compiling compiler_builtins v0.1.47
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0432]: unresolved import `crate::iter::adapters::zip::try_get_unchecked`
 --> library/core/src/iter/adapters/cloned.rs:2:5
 --> library/core/src/iter/adapters/cloned.rs:2:5
  |
2 |     zip::try_get_unchecked, TrustedRandomAccess, TrustedRandomAccessNoCoerce,
  |     ^^^^^^^^^^^^^^^^^^^^^^ no `try_get_unchecked` in `iter::adapters::zip`
error[E0432]: unresolved import `crate::iter::adapters::zip::try_get_unchecked`
 --> library/core/src/iter/adapters/copied.rs:2:5
  |
  |
2 |     zip::try_get_unchecked, TrustedRandomAccess, TrustedRandomAccessNoCoerce,
  |     ^^^^^^^^^^^^^^^^^^^^^^ no `try_get_unchecked` in `iter::adapters::zip`
error[E0432]: unresolved import `crate::iter::adapters::zip::try_get_unchecked`
 --> library/core/src/iter/adapters/enumerate.rs:2:5
  |
  |
2 |     zip::try_get_unchecked, SourceIter, TrustedRandomAccess, TrustedRandomAccessNoCoerce,
  |     ^^^^^^^^^^^^^^^^^^^^^^ no `try_get_unchecked` in `iter::adapters::zip`
error[E0432]: unresolved import `crate::iter::adapters::zip::try_get_unchecked`
 --> library/core/src/iter/adapters/fuse.rs:2:5
  |
2 | use crate::iter::adapters::zip::try_get_unchecked;
2 | use crate::iter::adapters::zip::try_get_unchecked;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `try_get_unchecked` in `iter::adapters::zip`
error[E0432]: unresolved import `crate::iter::adapters::zip::try_get_unchecked`
 --> library/core/src/iter/adapters/map.rs:3:5
  |
  |
3 |     zip::try_get_unchecked, SourceIter, TrustedRandomAccess, TrustedRandomAccessNoCoerce,
  |     ^^^^^^^^^^^^^^^^^^^^^^ no `try_get_unchecked` in `iter::adapters::zip`
error[E0432]: unresolved import `self::zip::Zip`
  --> library/core/src/iter/adapters/mod.rs:30:87
   |
   |
30 |     scan::Scan, skip::Skip, skip_while::SkipWhile, take::Take, take_while::TakeWhile, zip::Zip,
   |                                                                                       ^^^^^^^^ no `Zip` in `iter::adapters::zip`
error[E0432]: unresolved import `self::zip::TrustedRandomAccess`
  --> library/core/src/iter/adapters/mod.rs:52:9
   |
52 | pub use self::zip::TrustedRandomAccess;
52 | pub use self::zip::TrustedRandomAccess;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TrustedRandomAccess` in `iter::adapters::zip`

error[E0432]: unresolved import `self::zip::TrustedRandomAccessNoCoerce`
   |
   |
55 | pub use self::zip::TrustedRandomAccessNoCoerce;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TrustedRandomAccessNoCoerce` in `iter::adapters::zip`
error[E0432]: unresolved import `self::zip::zip`
  --> library/core/src/iter/adapters/mod.rs:58:9
   |
58 | pub use self::zip::zip;
58 | pub use self::zip::zip;
   |         ^^^^^^^^^^^^^^ no `zip` in `iter::adapters::zip`

error[E0432]: unresolved imports `super::TrustedRandomAccess`, `super::TrustedRandomAccessNoCoerce`
  |
  |
7 |     FusedIterator, TrustedLen, TrustedRandomAccess, TrustedRandomAccessNoCoerce, TrustedStep,
  |                                ^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TrustedRandomAccessNoCoerce` in `iter`
  |                                |
  |                                no `TrustedRandomAccess` in `iter`

error[E0432]: unresolved import `super::super::TrustedRandomAccessNoCoerce`
  |
  |
8 | use super::super::TrustedRandomAccessNoCoerce;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TrustedRandomAccessNoCoerce` in `iter`

error[E0432]: unresolved import `super::super::Zip`
   |
   |
11 | use super::super::{FromIterator, Intersperse, IntersperseWith, Product, Sum, Zip};
   |                                                                              ^^^ no `Zip` in `iter`

error[E0432]: unresolved imports `crate::iter::TrustedRandomAccess`, `crate::iter::TrustedRandomAccessNoCoerce`
   |
   |
11 | use crate::iter::{FusedIterator, TrustedLen, TrustedRandomAccess, TrustedRandomAccessNoCoerce};
   |                                              ^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TrustedRandomAccessNoCoerce` in `iter`
   |                                              |
   |                                              no `TrustedRandomAccess` in `iter`

error[E0432]: unresolved imports `crate::iter::TrustedRandomAccess`, `crate::iter::TrustedRandomAccessNoCoerce`
  |
  |
7 | use crate::iter::{TrustedRandomAccess, TrustedRandomAccessNoCoerce};
  |                   ^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TrustedRandomAccessNoCoerce` in `iter`
  |                   |
  |                   no `TrustedRandomAccess` in `iter`

error[E0603]: module `zip` is private
    |
395 | pub use self::adapters::zip;
    |                         ^^^ private module
    |
    |
note: the module `zip` is defined here
    |
    |
25  | mod zip;


error: an `#[unstable]` annotation here has no effect
   --> library/core/src/iter/adapters/cloned.rs:125:1
    |
125 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    |
    = note: `#[deny(ineffective_unstable_trait_impl)]` on by default


error: an `#[unstable]` annotation here has no effect
   --> library/core/src/iter/adapters/cloned.rs:129:1
    |
129 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
   --> library/core/src/iter/adapters/copied.rs:141:1
    |
141 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
   --> library/core/src/iter/adapters/copied.rs:145:1
    |
145 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
   --> library/core/src/iter/adapters/enumerate.rs:211:1
    |
211 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
   --> library/core/src/iter/adapters/enumerate.rs:215:1
    |
215 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
    |
    |
219 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
    |
    |
228 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
    |
    |
191 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
    |
    |
195 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
    |
    |
496 | / macro_rules! unsafe_range_trusted_random_access_impl {
497 | |     ($($t:ty)*) => ($(
498 | |         #[doc(hidden)]
499 | |         #[unstable(feature = "trusted_random_access", issue = "none")]
...   |
507 | |     )*)
508 | | }
508 | | }
    | |_- in this expansion of `unsafe_range_trusted_random_access_impl!`
...
709 | / unsafe_range_trusted_random_access_impl! {
710 | |     usize u8 u16
711 | |     isize i8 i16
    | |_- in this macro invocation
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
    |
    |
496 | / macro_rules! unsafe_range_trusted_random_access_impl {
497 | |     ($($t:ty)*) => ($(
498 | |         #[doc(hidden)]
499 | |         #[unstable(feature = "trusted_random_access", issue = "none")]
...   |
503 | |         #[unstable(feature = "trusted_random_access", issue = "none")]
...   |
507 | |     )*)
508 | | }
508 | | }
    | |_- in this expansion of `unsafe_range_trusted_random_access_impl!`
...
709 | / unsafe_range_trusted_random_access_impl! {
710 | |     usize u8 u16
711 | |     isize i8 i16
    | |_- in this macro invocation
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
    |
    |
496 | / macro_rules! unsafe_range_trusted_random_access_impl {
497 | |     ($($t:ty)*) => ($(
498 | |         #[doc(hidden)]
499 | |         #[unstable(feature = "trusted_random_access", issue = "none")]
...   |
507 | |     )*)
508 | | }
508 | | }
    | |_- in this expansion of `unsafe_range_trusted_random_access_impl!`
...
720 | / unsafe_range_trusted_random_access_impl! {
721 | |     u32 i32
722 | |     u64 i64
    | |_- in this macro invocation
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
    |
    |
496 | / macro_rules! unsafe_range_trusted_random_access_impl {
497 | |     ($($t:ty)*) => ($(
498 | |         #[doc(hidden)]
499 | |         #[unstable(feature = "trusted_random_access", issue = "none")]
...   |
503 | |         #[unstable(feature = "trusted_random_access", issue = "none")]
...   |
507 | |     )*)
508 | | }
508 | | }
    | |_- in this expansion of `unsafe_range_trusted_random_access_impl!`
...
720 | / unsafe_range_trusted_random_access_impl! {
721 | |     u32 i32
722 | |     u64 i64
    | |_- in this macro invocation
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1314 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1318 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1483 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1487 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1649 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1653 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1807 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1811 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1962 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1966 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2487 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2491 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2652 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2656 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2814 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2818 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2973 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2977 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2983 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2987 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2993 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
2997 | #[unstable(feature = "trusted_random_access", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
    |
    |
347 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
    |
    |
351 | #[unstable(feature = "trusted_random_access", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information

error: aborting due to 54 previous errors
