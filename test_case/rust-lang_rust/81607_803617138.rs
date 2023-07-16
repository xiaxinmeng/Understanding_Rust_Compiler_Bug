plain
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
[RUSTC-TIMING] build_script_build test:false 0.385
[RUSTC-TIMING] build_script_build test:false 0.592
error[E0407]: method `may_have_side_effect` is not a member of trait `TrustedRandomAccess`
    |
    |
207 | /     fn may_have_side_effect() -> bool {
209 | |     }
    | |_____^ not a member of trait `TrustedRandomAccess`


error[E0407]: method `may_have_side_effect` is not a member of trait `TrustedRandomAccess`
    |
    |
498 |  / macro_rules! unsafe_range_trusted_random_access_impl {
499 |  |     ($($t:ty)*) => ($(
500 |  |         #[doc(hidden)]
501 |  |         #[unstable(feature = "trusted_random_access", issue = "none")]
502 |  |         unsafe impl TrustedRandomAccess for ops::Range<$t> {
503 | /|             fn may_have_side_effect() -> bool {
505 | ||             }
    | ||_____________^ not a member of trait `TrustedRandomAccess`
506 |  |         }
507 |  |     )*)
507 |  |     )*)
508 |  | }
    |  |_- in this expansion of `unsafe_range_trusted_random_access_impl!`
...
604 | /  unsafe_range_trusted_random_access_impl! {
605 | |      usize u8 u16
606 | |      isize i8 i16
607 | |  }


error[E0407]: method `may_have_side_effect` is not a member of trait `TrustedRandomAccess`
    |
    |
498 |  / macro_rules! unsafe_range_trusted_random_access_impl {
499 |  |     ($($t:ty)*) => ($(
500 |  |         #[doc(hidden)]
501 |  |         #[unstable(feature = "trusted_random_access", issue = "none")]
502 |  |         unsafe impl TrustedRandomAccess for ops::Range<$t> {
503 | /|             fn may_have_side_effect() -> bool {
505 | ||             }
    | ||_____________^ not a member of trait `TrustedRandomAccess`
506 |  |         }
507 |  |     )*)
507 |  |     )*)
508 |  | }
    |  |_- in this expansion of `unsafe_range_trusted_random_access_impl!`
...
615 | /  unsafe_range_trusted_random_access_impl! {
616 | |      u32 i32
617 | |      u64 i64
618 | |  }


error[E0046]: not all trait items implemented, missing: `MAY_HAVE_SIDE_EFFECT`
    |
    |
203 | / unsafe impl<T, const N: usize> TrustedRandomAccess for IntoIter<T, N>
204 | | where
205 | |     T: Copy,
206 | | {
209 | |     }
210 | | }
210 | | }
    | |_^ missing `MAY_HAVE_SIDE_EFFECT` in implementation
    | 
   ::: library/core/src/iter/adapters/zip.rs:429:5
    |
429 |       const MAY_HAVE_SIDE_EFFECT: bool;
    |       --------------------------------- `MAY_HAVE_SIDE_EFFECT` from trait

error[E0046]: not all trait items implemented, missing: `MAY_HAVE_SIDE_EFFECT`
    |
    |
498 | / macro_rules! unsafe_range_trusted_random_access_impl {
499 | |     ($($t:ty)*) => ($(
500 | |         #[doc(hidden)]
501 | |         #[unstable(feature = "trusted_random_access", issue = "none")]
502 | |         unsafe impl TrustedRandomAccess for ops::Range<$t> {
    | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `MAY_HAVE_SIDE_EFFECT` in implementation
507 | |     )*)
508 | | }
508 | | }
    | |_- in this expansion of `unsafe_range_trusted_random_access_impl!`
...
604 | / unsafe_range_trusted_random_access_impl! {
605 | |     usize u8 u16
606 | |     isize i8 i16
    | |_- in this macro invocation
    | 
    | 
   ::: library/core/src/iter/adapters/zip.rs:429:5
    |
429 |       const MAY_HAVE_SIDE_EFFECT: bool;
    |       --------------------------------- `MAY_HAVE_SIDE_EFFECT` from trait

error[E0046]: not all trait items implemented, missing: `MAY_HAVE_SIDE_EFFECT`
    |
    |
498 | / macro_rules! unsafe_range_trusted_random_access_impl {
499 | |     ($($t:ty)*) => ($(
500 | |         #[doc(hidden)]
501 | |         #[unstable(feature = "trusted_random_access", issue = "none")]
502 | |         unsafe impl TrustedRandomAccess for ops::Range<$t> {
    | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `MAY_HAVE_SIDE_EFFECT` in implementation
507 | |     )*)
508 | | }
508 | | }
    | |_- in this expansion of `unsafe_range_trusted_random_access_impl!`
...
615 | / unsafe_range_trusted_random_access_impl! {
616 | |     u32 i32
617 | |     u64 i64
    | |_- in this macro invocation
    | 
    | 
   ::: library/core/src/iter/adapters/zip.rs:429:5
    |
429 |       const MAY_HAVE_SIDE_EFFECT: bool;
    |       --------------------------------- `MAY_HAVE_SIDE_EFFECT` from trait
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0046, E0407.
For more information about an error, try `rustc --explain E0046`.
