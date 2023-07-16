plain
   --> /checkout/library/alloc/src/macros.rs:43:36
    |
41  | / macro_rules! vec {
42  | |     () => (
43  | |         $crate::__rust_force_expr!($crate::vec::Vec::new())
    | |                                    ^^^^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `T`
...   |
50  | |     );
51  | | }
    | |_- in this expansion of `vec!`
    | |_- in this expansion of `vec!`
    |
   ::: library/alloc/tests/linked_list.rs:396:54
    |
396 |       assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![]);


error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:419:5
    |
419 |       assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`
error[E0282]: type annotations needed
   --> /checkout/library/alloc/src/macros.rs:43:36
    |
41  | / macro_rules! vec {
41  | / macro_rules! vec {
42  | |     () => (
43  | |         $crate::__rust_force_expr!($crate::vec::Vec::new())
    | |                                    ^^^^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `T`
...   |
50  | |     );
51  | | }
    | |_- in this expansion of `vec!`
    | |_- in this expansion of `vec!`
    |
   ::: library/alloc/tests/linked_list.rs:443:54
    |
443 |       assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![]);


error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:459:9
    |
459 |           assert_eq!(removed, vec![2, 4, 6, 18, 20, 22, 24, 26, 34, 36]);
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`

error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:462:9
    |
462 | /         assert_eq!(
463 | |             list.into_iter().collect::<Vec<_>>(),
464 | |             vec![1, 7, 9, 11, 13, 15, 17, 27, 29, 31, 33, 35, 37, 39]
    | |_________- in this macro invocation
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`

error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:477:9
    |
477 |           assert_eq!(removed, vec![2, 4, 6, 18, 20, 22, 24, 26, 34, 36]);
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`

error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:480:9
    |
480 | /         assert_eq!(
481 | |             list.into_iter().collect::<Vec<_>>(),
482 | |             vec![7, 9, 11, 13, 15, 17, 27, 29, 31, 33, 35, 37, 39]
    | |_________- in this macro invocation
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`

error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:495:9
    |
495 |           assert_eq!(removed, vec![2, 4, 6, 18, 20, 22, 24, 26, 34, 36]);
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`

error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:498:9
    |
498 | /         assert_eq!(
499 | |             list.into_iter().collect::<Vec<_>>(),
500 | |             vec![7, 9, 11, 13, 15, 17, 27, 29, 31, 33, 35]
    | |_________- in this macro invocation
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`

error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:512:9
    |
512 |           assert_eq!(removed, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`

error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:515:9
    |
515 |           assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`

error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:526:9
    |
526 |           assert_eq!(removed, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`

error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/linked_list.rs:529:9
    |
529 |           assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
    |
    |
    = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    = note: required because of the requirements on the impl of `PartialEq<Vec<{integer}>>` for `Vec<&{integer}>`

error[E0277]: a value of type `map::BTreeMap<_, _>` cannot be built from an iterator over elements of type `&(usize, {integer})`
    --> library/alloc/src/collections/btree/map/tests.rs:731:54
     |
731  |     let map: BTreeMap<_, _> = [(max, 0)].into_iter().collect();
     |                                                      ^^^^^^^ value of type `map::BTreeMap<_, _>` cannot be built from `std::iter::Iterator<Item=&(usize, {integer})>`
     |
     = help: the trait `core::iter::FromIterator<&(usize, {integer})>` is not implemented for `map::BTreeMap<_, _>`
note: required by a bound in `collect`
     |
     |
1741 |     fn collect<B: FromIterator<Self::Item>>(self) -> B
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`

error[E0277]: a value of type `String` cannot be built from an iterator over elements of type `&&str`
     |
     |
492  |     let c: String = [t, u].into_iter().collect();
     |                                        ^^^^^^^ value of type `String` cannot be built from `std::iter::Iterator<Item=&&str>`
     |
     = help: the trait `FromIterator<&&str>` is not implemented for `String`
note: required by a bound in `collect`
     |
     |
1741 |     fn collect<B: FromIterator<Self::Item>>(self) -> B
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`
error[E0631]: type mismatch in closure arguments
    --> library/alloc/tests/vec.rs:452:31
     |
     |
452  |     assert_eq!([].into_iter().partition(|x: &i32| *x < 3), (vec![], vec![]));
     |                               ^^^^^^^^^ ---------------- found signature of `for<'r> fn(&'r i32) -> _`
     |                               |
     |                               expected signature of `for<'r> fn(&'r &_) -> _`
note: required by a bound in `partition`
    --> /checkout/library/core/src/iter/traits/iterator.rs:1778:12
     |
1778 |         F: FnMut(&Self::Item) -> bool,
1778 |         F: FnMut(&Self::Item) -> bool,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `partition`
error[E0308]: mismatched types
   --> library/alloc/tests/vec.rs:453:57
    |
    |
453 |     assert_eq!([1, 2, 3].into_iter().partition(|x| *x < 4), (vec![1, 2, 3], vec![]));
    |                                                         |
    |                                                         expected reference, found integer
    |                                                         help: consider borrowing here: `&4`
    |
    |
    = note: expected reference `&_`
                    found type `{integer}`

error[E0308]: mismatched types
   --> library/alloc/tests/vec.rs:454:57
    |
454 |     assert_eq!([1, 2, 3].into_iter().partition(|x| *x < 2), (vec![1], vec![2, 3]));
    |                                                         |
    |                                                         expected reference, found integer
    |                                                         help: consider borrowing here: `&2`
    |
    |
    = note: expected reference `&_`
                    found type `{integer}`

error[E0308]: mismatched types
   --> library/alloc/tests/vec.rs:455:57
    |
455 |     assert_eq!([1, 2, 3].into_iter().partition(|x| *x < 0), (vec![], vec![1, 2, 3]));
    |                                                         |
    |                                                         expected reference, found integer
    |                                                         help: consider borrowing here: `&0`
    |
    |
    = note: expected reference `&_`
                    found type `{integer}`

error[E0271]: type mismatch resolving `<std::slice::Iter<'_, {integer}> as Iterator>::Item == i32`
    |
    |
937 |     iter_equal(it.clone(), &[1, 2, 3]);
    |     ^^^^^^^^^^ expected `i32`, found `&{integer}`
    |
note: required by a bound in `iter_equal`
    |
    |
932 |     fn iter_equal<I: Iterator<Item = i32>>(it: I, slice: &[i32]) {
    |                               ^^^^^^^^^^ required by this bound in `iter_equal`
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:39:35
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                   ^^^^^^^^^^ expected `&{integer}`, found integer
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/vec.rs:938:5
    |
938 |       assert_eq!(it.next(), Some(1));
    |
    = note: expected enum `Option<&{integer}>`
               found enum `Option<{integer}>`


error[E0271]: type mismatch resolving `<Rev<std::slice::Iter<'_, {integer}>> as Iterator>::Item == i32`
    |
    |
940 |     iter_equal(it.clone(), &[3, 2]);
    |     ^^^^^^^^^^ expected `i32`, found `&{integer}`
    |
note: required by a bound in `iter_equal`
    |
    |
932 |     fn iter_equal<I: Iterator<Item = i32>>(it: I, slice: &[i32]) {
    |                               ^^^^^^^^^^ required by this bound in `iter_equal`
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:39:35
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                   ^^^^^^^^^^ expected `&{integer}`, found integer
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/vec.rs:941:5
    |
941 |       assert_eq!(it.next(), Some(3));
    |
    = note: expected enum `Option<&{integer}>`
               found enum `Option<{integer}>`


error[E0271]: type mismatch resolving `<Rev<std::slice::Iter<'_, {integer}>> as Iterator>::Item == i32`
    |
    |
942 |     iter_equal(it.clone(), &[2]);
    |     ^^^^^^^^^^ expected `i32`, found `&{integer}`
    |
note: required by a bound in `iter_equal`
    |
    |
932 |     fn iter_equal<I: Iterator<Item = i32>>(it: I, slice: &[i32]) {
    |                               ^^^^^^^^^^ required by this bound in `iter_equal`
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:39:35
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                   ^^^^^^^^^^ expected `&{integer}`, found integer
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    |
    |
   ::: library/alloc/tests/vec.rs:943:5
    |
943 |       assert_eq!(it.next(), Some(2));
    |
    = note: expected enum `Option<&{integer}>`
               found enum `Option<{integer}>`


error[E0271]: type mismatch resolving `<Rev<std::slice::Iter<'_, {integer}>> as Iterator>::Item == i32`
    |
    |
944 |     iter_equal(it.clone(), &[]);
    |     ^^^^^^^^^^ expected `i32`, found `&{integer}`
    |
note: required by a bound in `iter_equal`
    |
    |
932 |     fn iter_equal<I: Iterator<Item = i32>>(it: I, slice: &[i32]) {
    |                               ^^^^^^^^^^ required by this bound in `iter_equal`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error[E0271]: type mismatch resolving `<Filter<std::slice::Iter<'_, {integer}>, [closure@library/alloc/tests/vec.rs:1802:62: 1802:70]> as IntoIterator>::Item == i32`
     |
     |
1802 |     next_then_drop(v.splice(5..6, [1; 10].into_iter().filter(|_| true))); // lower bound not exact
     |                      ^^^^^^ expected `i32`, found `&{integer}`
     |
note: required by a bound in `Vec::<T, A>::splice`
     |
2698 |         I: IntoIterator<Item = T>,
2698 |         I: IntoIterator<Item = T>,
     |                         ^^^^^^^^ required by this bound in `Vec::<T, A>::splice`

error[E0277]: can't compare `&{integer}` with `{integer}`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                ^^ no implementation for `&{integer} == {integer}`
