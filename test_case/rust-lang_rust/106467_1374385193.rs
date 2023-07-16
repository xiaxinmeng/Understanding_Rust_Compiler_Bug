plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 84f22e44c588be9c9058d6d6ed02a21aa32ad843 and 5a4d9e8289364cdab44e060afcb1afe7d657a9c4
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
error[E0425]: cannot find function `co_alloc_metadata_num_slots_with_preference_specific` in module `core::alloc`
    --> library/alloc/src/collections/vec_deque/macros.rs:7:31
     |
1    | / macro_rules! __impl_slice_eq1 {
2    | |     ([$($vars:tt)*] $lhs:ty, $rhs:ty, $($constraints:tt)*) => {
3    | |         #[stable(feature = "vec_deque_partial_eq_slice", since = "1.17.0")]
4    | |         impl<T, U, A: Allocator, const COOP_PREFERRED: bool, $($vars)*> PartialEq<$rhs> for $lhs
...    |
7    | |             [(); core::alloc::co_alloc_metadata_num_slots_with_preference_specific::<A>(COOP_PREFERRED)]:,
     | |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `co_alloc_metadata_num_slots_with_preference`
19   | |     }
20   | | }
     | |_- in this expansion of `__impl_slice_eq1!`
     |
     |
    ::: library/alloc/src/collections/vec_deque/mod.rs:2725:1
     |
2725 |   __impl_slice_eq1! { [] VecDeque<T, A, COOP_PREFERRED>, Vec<U, A, COOP_PREFERRED>, }
     |
    ::: /checkout/library/core/src/alloc/mod.rs:107:1
     |
     |
107  | / pub const fn co_alloc_metadata_num_slots_with_preference<A: Allocator>(
108  | |     coop_preferred: bool,
109  | | ) -> usize {
     | |__________- similarly named function `co_alloc_metadata_num_slots_with_preference` defined here

error[E0425]: cannot find function `co_alloc_metadata_num_slots_with_preference_specific` in module `core::alloc`
    --> library/alloc/src/collections/vec_deque/macros.rs:7:31
     |
1    | / macro_rules! __impl_slice_eq1 {
2    | |     ([$($vars:tt)*] $lhs:ty, $rhs:ty, $($constraints:tt)*) => {
3    | |         #[stable(feature = "vec_deque_partial_eq_slice", since = "1.17.0")]
4    | |         impl<T, U, A: Allocator, const COOP_PREFERRED: bool, $($vars)*> PartialEq<$rhs> for $lhs
...    |
7    | |             [(); core::alloc::co_alloc_metadata_num_slots_with_preference_specific::<A>(COOP_PREFERRED)]:,
     | |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `co_alloc_metadata_num_slots_with_preference`
19   | |     }
20   | | }
     | |_- in this expansion of `__impl_slice_eq1!`
     |
     |
    ::: library/alloc/src/collections/vec_deque/mod.rs:2726:1
     |
2726 |   __impl_slice_eq1! { [] VecDeque<T, A, COOP_PREFERRED>, &[U], }
     |
    ::: /checkout/library/core/src/alloc/mod.rs:107:1
     |
     |
107  | / pub const fn co_alloc_metadata_num_slots_with_preference<A: Allocator>(
108  | |     coop_preferred: bool,
109  | | ) -> usize {
     | |__________- similarly named function `co_alloc_metadata_num_slots_with_preference` defined here

error[E0425]: cannot find function `co_alloc_metadata_num_slots_with_preference_specific` in module `core::alloc`
    --> library/alloc/src/collections/vec_deque/macros.rs:7:31
     |
1    | / macro_rules! __impl_slice_eq1 {
2    | |     ([$($vars:tt)*] $lhs:ty, $rhs:ty, $($constraints:tt)*) => {
3    | |         #[stable(feature = "vec_deque_partial_eq_slice", since = "1.17.0")]
4    | |         impl<T, U, A: Allocator, const COOP_PREFERRED: bool, $($vars)*> PartialEq<$rhs> for $lhs
...    |
7    | |             [(); core::alloc::co_alloc_metadata_num_slots_with_preference_specific::<A>(COOP_PREFERRED)]:,
     | |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `co_alloc_metadata_num_slots_with_preference`
19   | |     }
20   | | }
     | |_- in this expansion of `__impl_slice_eq1!`
     |
     |
    ::: library/alloc/src/collections/vec_deque/mod.rs:2727:1
     |
2727 |   __impl_slice_eq1! { [] VecDeque<T, A, COOP_PREFERRED>, &mut [U], }
     |
    ::: /checkout/library/core/src/alloc/mod.rs:107:1
     |
     |
107  | / pub const fn co_alloc_metadata_num_slots_with_preference<A: Allocator>(
108  | |     coop_preferred: bool,
109  | | ) -> usize {
     | |__________- similarly named function `co_alloc_metadata_num_slots_with_preference` defined here

error[E0425]: cannot find function `co_alloc_metadata_num_slots_with_preference_specific` in module `core::alloc`
    --> library/alloc/src/collections/vec_deque/macros.rs:7:31
     |
1    | / macro_rules! __impl_slice_eq1 {
2    | |     ([$($vars:tt)*] $lhs:ty, $rhs:ty, $($constraints:tt)*) => {
3    | |         #[stable(feature = "vec_deque_partial_eq_slice", since = "1.17.0")]
4    | |         impl<T, U, A: Allocator, const COOP_PREFERRED: bool, $($vars)*> PartialEq<$rhs> for $lhs
...    |
7    | |             [(); core::alloc::co_alloc_metadata_num_slots_with_preference_specific::<A>(COOP_PREFERRED)]:,
     | |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `co_alloc_metadata_num_slots_with_preference`
19   | |     }
20   | | }
     | |_- in this expansion of `__impl_slice_eq1!`
     |
     |
    ::: library/alloc/src/collections/vec_deque/mod.rs:2728:1
     |
2728 |   __impl_slice_eq1! { [const N: usize] VecDeque<T, A, COOP_PREFERRED>, [U; N], }
     |
    ::: /checkout/library/core/src/alloc/mod.rs:107:1
     |
     |
107  | / pub const fn co_alloc_metadata_num_slots_with_preference<A: Allocator>(
108  | |     coop_preferred: bool,
109  | | ) -> usize {
     | |__________- similarly named function `co_alloc_metadata_num_slots_with_preference` defined here

error[E0425]: cannot find function `co_alloc_metadata_num_slots_with_preference_specific` in module `core::alloc`
    --> library/alloc/src/collections/vec_deque/macros.rs:7:31
     |
1    | / macro_rules! __impl_slice_eq1 {
2    | |     ([$($vars:tt)*] $lhs:ty, $rhs:ty, $($constraints:tt)*) => {
3    | |         #[stable(feature = "vec_deque_partial_eq_slice", since = "1.17.0")]
4    | |         impl<T, U, A: Allocator, const COOP_PREFERRED: bool, $($vars)*> PartialEq<$rhs> for $lhs
...    |
7    | |             [(); core::alloc::co_alloc_metadata_num_slots_with_preference_specific::<A>(COOP_PREFERRED)]:,
     | |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `co_alloc_metadata_num_slots_with_preference`
19   | |     }
20   | | }
     | |_- in this expansion of `__impl_slice_eq1!`
     |
     |
    ::: library/alloc/src/collections/vec_deque/mod.rs:2729:1
     |
2729 |   __impl_slice_eq1! { [const N: usize] VecDeque<T, A, COOP_PREFERRED>, &[U; N], }
     |
    ::: /checkout/library/core/src/alloc/mod.rs:107:1
     |
     |
107  | / pub const fn co_alloc_metadata_num_slots_with_preference<A: Allocator>(
108  | |     coop_preferred: bool,
109  | | ) -> usize {
     | |__________- similarly named function `co_alloc_metadata_num_slots_with_preference` defined here

error[E0425]: cannot find function `co_alloc_metadata_num_slots_with_preference_specific` in module `core::alloc`
    --> library/alloc/src/collections/vec_deque/macros.rs:7:31
     |
1    | / macro_rules! __impl_slice_eq1 {
2    | |     ([$($vars:tt)*] $lhs:ty, $rhs:ty, $($constraints:tt)*) => {
3    | |         #[stable(feature = "vec_deque_partial_eq_slice", since = "1.17.0")]
4    | |         impl<T, U, A: Allocator, const COOP_PREFERRED: bool, $($vars)*> PartialEq<$rhs> for $lhs
...    |
7    | |             [(); core::alloc::co_alloc_metadata_num_slots_with_preference_specific::<A>(COOP_PREFERRED)]:,
     | |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `co_alloc_metadata_num_slots_with_preference`
19   | |     }
20   | | }
     | |_- in this expansion of `__impl_slice_eq1!`
     |
     |
    ::: library/alloc/src/collections/vec_deque/mod.rs:2730:1
     |
2730 |   __impl_slice_eq1! { [const N: usize] VecDeque<T, A, COOP_PREFERRED>, &mut [U; N], }
     |
    ::: /checkout/library/core/src/alloc/mod.rs:107:1
     |
     |
107  | / pub const fn co_alloc_metadata_num_slots_with_preference<A: Allocator>(
108  | |     coop_preferred: bool,
109  | | ) -> usize {
     | |__________- similarly named function `co_alloc_metadata_num_slots_with_preference` defined here
error: unconstrained generic constant
    --> library/alloc/src/boxed.rs:1687:53
     |
     |
1687 | impl<T, const N: usize, const COOP_PREFERRED: bool> TryFrom<Vec<T, Global, COOP_PREFERRED>>
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); core::alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `Vec`
     |
409  | pub struct Vec<
     |            --- required by a bound in this
...
...
415  |     [(); core::alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Vec`
error: unconstrained generic constant
    --> library/alloc/src/boxed.rs:1692:18
     |
     |
1692 |     type Error = Vec<T, Global, COOP_PREFERRED>;
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); core::alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `Vec`
     |
409  | pub struct Vec<
     |            --- required by a bound in this
...
...
415  |     [(); core::alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Vec`
error: unconstrained generic constant
    --> library/alloc/src/boxed.rs:1712:22
     |
     |
1712 |     fn try_from(vec: Vec<T, Global, COOP_PREFERRED>) -> Result<Self, Self::Error> {
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); core::alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `Vec`
     |
409  | pub struct Vec<
     |            --- required by a bound in this
...
...
415  |     [(); core::alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Vec`
error: unconstrained generic constant
    --> library/alloc/src/collections/binary_heap.rs:1487:11
     |
     |
1487 |     iter: vec::Drain<'a, T, Global, COOP_PREFERRED>,
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `vec::drain::Drain`
     |
23   | pub struct Drain<
     |            ----- required by a bound in this
...
...
29   |     [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `vec::drain::Drain`
error: unconstrained generic constant
    --> library/alloc/src/collections/binary_heap.rs:1509:37
     |
     |
1509 | impl<T, const COOP_PREFERRED: bool> DoubleEndedIterator for Drain<'_, T, COOP_PREFERRED> {
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:`
note: required for `binary_heap::Drain<'_, T, COOP_PREFERRED>` to implement `Iterator`
     |
     |
1491 | impl<T, const COOP_PREFERRED: bool> Iterator for Drain<'_, T, COOP_PREFERRED>
note: required by a bound in `DoubleEndedIterator`
    --> /checkout/library/core/src/iter/traits/double_ended.rs:40:32
     |
40   | pub trait DoubleEndedIterator: Iterator {
40   | pub trait DoubleEndedIterator: Iterator {
     |                                ^^^^^^^^ required by this bound in `DoubleEndedIterator`

error: unconstrained generic constant
    --> library/alloc/src/collections/binary_heap.rs:1509:61
     |
1509 | impl<T, const COOP_PREFERRED: bool> DoubleEndedIterator for Drain<'_, T, COOP_PREFERRED> {
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:`
note: required by a bound in `binary_heap::Drain`
     |
     |
1483 | pub struct Drain<'a, T: 'a, const COOP_PREFERRED: bool>
1484 | where
1484 | where
1485 |     [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `binary_heap::Drain`
error: unconstrained generic constant
    --> library/alloc/src/collections/binary_heap.rs:1517:37
     |
     |
1517 | impl<T, const COOP_PREFERRED: bool> ExactSizeIterator for Drain<'_, T, COOP_PREFERRED> {
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:`
note: required for `binary_heap::Drain<'_, T, COOP_PREFERRED>` to implement `Iterator`
     |
     |
1491 | impl<T, const COOP_PREFERRED: bool> Iterator for Drain<'_, T, COOP_PREFERRED>
note: required by a bound in `ExactSizeIterator`
    --> /checkout/library/core/src/iter/traits/exact_size.rs:76:30
     |
76   | pub trait ExactSizeIterator: Iterator {
76   | pub trait ExactSizeIterator: Iterator {
     |                              ^^^^^^^^ required by this bound in `ExactSizeIterator`

error: unconstrained generic constant
    --> library/alloc/src/collections/binary_heap.rs:1517:59
     |
1517 | impl<T, const COOP_PREFERRED: bool> ExactSizeIterator for Drain<'_, T, COOP_PREFERRED> {
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:`
note: required by a bound in `binary_heap::Drain`
     |
     |
1483 | pub struct Drain<'a, T: 'a, const COOP_PREFERRED: bool>
1484 | where
1484 | where
1485 |     [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `binary_heap::Drain`
error: unconstrained generic constant
    --> library/alloc/src/collections/binary_heap.rs:1524:37
     |
     |
1524 | impl<T, const COOP_PREFERRED: bool> FusedIterator for Drain<'_, T, COOP_PREFERRED> {}
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:`
note: required for `binary_heap::Drain<'_, T, COOP_PREFERRED>` to implement `Iterator`
     |
     |
1491 | impl<T, const COOP_PREFERRED: bool> Iterator for Drain<'_, T, COOP_PREFERRED>
     |                                     ^^^^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `FusedIterator`
     |
17   | pub trait FusedIterator: Iterator {}
17   | pub trait FusedIterator: Iterator {}
     |                          ^^^^^^^^ required by this bound in `FusedIterator`
error: unconstrained generic constant
    --> library/alloc/src/collections/binary_heap.rs:1524:55
     |
     |
1524 | impl<T, const COOP_PREFERRED: bool> FusedIterator for Drain<'_, T, COOP_PREFERRED> {}
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:`
note: required by a bound in `binary_heap::Drain`
     |
     |
1483 | pub struct Drain<'a, T: 'a, const COOP_PREFERRED: bool>
1484 | where
1484 | where
1485 |     [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `binary_heap::Drain`
error: unconstrained generic constant
    --> library/alloc/src/collections/binary_heap.rs:1511:18
     |
1511 |     fn next_back(&mut self) -> Option<T> {
1511 |     fn next_back(&mut self) -> Option<T> {
     |                  ^^^^^^^^^
     |
     = help: try adding a `where` bound using this expression: `where [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:`
note: required by a bound in `binary_heap::Drain`
     |
     |
1483 | pub struct Drain<'a, T: 'a, const COOP_PREFERRED: bool>
1484 | where
1484 | where
1485 |     [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `binary_heap::Drain`
error: unconstrained generic constant
    --> library/alloc/src/collections/binary_heap.rs:1518:17
     |
1518 |     fn is_empty(&self) -> bool {
1518 |     fn is_empty(&self) -> bool {
     |                 ^^^^^
     |
     = help: try adding a `where` bound using this expression: `where [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:`
note: required by a bound in `binary_heap::Drain`
     |
     |
1483 | pub struct Drain<'a, T: 'a, const COOP_PREFERRED: bool>
1484 | where
1484 | where
1485 |     [(); crate::co_alloc_metadata_num_slots_with_preference_global(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `binary_heap::Drain`
error: unconstrained generic constant
   --> library/alloc/src/collections/vec_deque/mod.rs:162:49
    |
    |
162 | impl<T, const COOP_PREFERRED: bool> Default for VecDeque<T, Global, COOP_PREFERRED>
    |
    |
    = help: try adding a `where` bound using this expression: `where [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `VecDeque`
    |
97  | pub struct VecDeque<
    |            -------- required by a bound in this
...
...
102 |     [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `VecDeque`
error: unconstrained generic constant
    --> library/alloc/src/collections/vec_deque/mod.rs:2984:9
     |
     |
2984 |     for VecDeque<T, Global, COOP_PREFERRED>
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `VecDeque`
     |
97   | pub struct VecDeque<
     |            -------- required by a bound in this
...
...
102  |     [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `VecDeque`
error: unconstrained generic constant
   --> library/alloc/src/collections/vec_deque/mod.rs:168:21
    |
    |
168 |     fn default() -> VecDeque<T, Global, COOP_PREFERRED> {
    |
    |
    = help: try adding a `where` bound using this expression: `where [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `VecDeque`
    |
97  | pub struct VecDeque<
    |            -------- required by a bound in this
...
...
102 |     [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `VecDeque`
error: unconstrained generic constant
   --> library/alloc/src/collections/vec_deque/mod.rs:571:27
    |
    |
571 |     pub const fn new() -> VecDeque<T, Global, COOP_PREFERRED> {
    |
    |
    = help: try adding a `where` bound using this expression: `where [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `VecDeque`
    |
97  | pub struct VecDeque<
    |            -------- required by a bound in this
...
...
102 |     [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `VecDeque`
error: unconstrained generic constant
    --> library/alloc/src/collections/vec_deque/mod.rs:1401:45
     |
     |
1401 |     pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `vec_deque::drain::Drain`
    --> library/alloc/src/collections/vec_deque/drain.rs:25:10
19   | pub struct Drain<
     |            ----- required by a bound in this
...
...
25   |     [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `vec_deque::drain::Drain`
error: unconstrained generic constant
    --> library/alloc/src/collections/vec_deque/mod.rs:2800:57
     |
     |
2800 |     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> VecDeque<T, Global, COOP_PREFERRED> {
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `VecDeque`
     |
97   | pub struct VecDeque<
     |            -------- required by a bound in this
...
...
102  |     [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `VecDeque`
error: unconstrained generic constant
    --> library/alloc/src/collections/vec_deque/mod.rs:2811:21
     |
2811 |     type IntoIter = IntoIter<T, A>;
2811 |     type IntoIter = IntoIter<T, A>;
     |                     ^^^^^^^^^^^^^^
     |
     = help: try adding a `where` bound using this expression: `where [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `vec_deque::into_iter::IntoIter`
     |
17   | pub struct IntoIter<
     |            -------- required by a bound in this
...
...
22   |     [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `vec_deque::into_iter::IntoIter`
error: unconstrained generic constant
    --> library/alloc/src/collections/vec_deque/mod.rs:2815:27
     |
2815 |     fn into_iter(self) -> IntoIter<T, A> {
2815 |     fn into_iter(self) -> IntoIter<T, A> {
     |                           ^^^^^^^^^^^^^^
     |
     = help: try adding a `where` bound using this expression: `where [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `vec_deque::into_iter::IntoIter`
     |
17   | pub struct IntoIter<
     |            -------- required by a bound in this
...
...
22   |     [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `vec_deque::into_iter::IntoIter`
error: unconstrained generic constant
    --> library/alloc/src/collections/vec_deque/mod.rs:2962:20
     |
     |
2962 |         mut other: VecDeque<T, A, _VECDEQUE_COOP_PREFERRED>,
     |
     |
     = help: try adding a `where` bound using this expression: `where [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:`
note: required by a bound in `VecDeque`
