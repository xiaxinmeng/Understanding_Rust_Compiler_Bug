plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0720]: cannot resolve opaque type
  --> library/core/src/iter/adapters/enumerate.rs:80:14
   |
80 |         ) -> impl FnMut(Acc, T) -> R + 'a {
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
  --> library/core/src/iter/adapters/filter.rs:43:6
   |
   |
43 | ) -> impl FnMut(Acc, T) -> R + 'a {
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
  --> library/core/src/iter/adapters/filter_map.rs:45:6
   |
   |
45 | ) -> impl FnMut(Acc, T) -> R + 'a {
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
  --> library/core/src/iter/adapters/filter_map.rs:99:14
   |
   |
99 |         ) -> impl FnMut((), T) -> ControlFlow<B> + '_ {
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
   --> library/core/src/iter/adapters/flatten.rs:302:14
    |
    |
302 |         ) -> impl FnMut(Acc, T) -> R + 'a {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
   --> library/core/src/iter/adapters/flatten.rs:335:14
    |
    |
335 |         ) -> impl FnMut(Acc, T) -> Acc + '_ {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
   --> library/core/src/iter/adapters/flatten.rs:391:14
    |
    |
391 |         ) -> impl FnMut(Acc, T) -> R + 'a
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
   --> library/core/src/iter/adapters/flatten.rs:427:14
    |
    |
427 |         ) -> impl FnMut(Acc, T) -> Acc + '_
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
  --> library/core/src/iter/adapters/inspect.rs:60:6
   |
   |
60 | ) -> impl FnMut(Acc, T) -> R + 'a {
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
  --> library/core/src/iter/adapters/map.rs:88:6
   |
   |
88 | ) -> impl FnMut(Acc, T) -> R + 'a {
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
  --> library/core/src/iter/adapters/scan.rs:65:14
   |
   |
65 |         ) -> impl FnMut(Acc, T) -> ControlFlow<R, Acc> + 'a {
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
  --> library/core/src/iter/adapters/skip_while.rs:46:14
   |
   |
46 |         ) -> impl FnMut(&T) -> bool + 'a {
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
   --> library/core/src/iter/adapters/step_by.rs:117:59
    |
    |
117 |         fn nth<I: Iterator>(iter: &mut I, step: usize) -> impl FnMut() -> Option<I::Item> + '_ {
    |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
   --> library/core/src/iter/adapters/step_by.rs:136:59
    |
    |
136 |         fn nth<I: Iterator>(iter: &mut I, step: usize) -> impl FnMut() -> Option<I::Item> + '_ {
    |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
   --> library/core/src/iter/adapters/step_by.rs:196:14
    |
    |
196 |         ) -> impl FnMut() -> Option<I::Item> + '_ {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
   --> library/core/src/iter/adapters/step_by.rs:219:14
    |
    |
219 |         ) -> impl FnMut() -> Option<I::Item> + '_ {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
error[E0720]: cannot resolve opaque type
  --> library/core/src/iter/adapters/take.rs:85:14
   |
85 |         ) -> impl FnMut(Acc, T) -> ControlFlow<R, Acc> + 'a {
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
  --> library/core/src/iter/adapters/take_while.rs:77:14
   |
   |
77 |         ) -> impl FnMut(Acc, T) -> ControlFlow<R, Acc> + 'a {
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
    --> library/core/src/iter/traits/iterator.rs:1788:14
     |
     |
1788 |         ) -> impl FnMut((), T) + 'a {
     |              ^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
    --> library/core/src/iter/traits/iterator.rs:1852:14
     |
     |
1852 |         ) -> impl FnMut(&&mut T) -> bool + 'a {
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
    --> library/core/src/iter/traits/iterator.rs:1861:66
     |
     |
1861 |         fn is_true<T>(predicate: &mut impl FnMut(&T) -> bool) -> impl FnMut(&&mut T) -> bool + '_ {
     |                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
    --> library/core/src/iter/traits/iterator.rs:2886:14
     |
     |
2886 |         ) -> impl FnMut((), (A, B)) + 'a {
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error[E0720]: cannot resolve opaque type
    --> library/core/src/iter/traits/iterator.rs:3440:14
     |
     |
3440 |         ) -> impl FnMut(T) -> bool + 'a {
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
error: aborting due to 23 previous errors

For more information about this error, try `rustc --explain E0720`.
error: could not compile `core`
