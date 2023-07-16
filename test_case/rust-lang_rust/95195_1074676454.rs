plain
.................................................................................ii................. 300/3816
.................................................................................................... 400/3816
.......................................i............................................................ 500/3816
.................................................................................................... 600/3816
.................................FFF.ii...................iiii...................................... 700/3816
.................................................................................................... 900/3816
.................................................................................................... 1000/3816
.................................................................................................... 1100/3816
.................................................................................................... 1200/3816
---
.....................i.............................................................................. 3800/3816
................
failures:

---- src/iter/traits/peekable.rs - iter::traits::peekable::PeekableIterator (line 38) stdout ----
error[E0599]: no method named `peek` found for struct `std::ops::Range` in the current scope
   |
   |
7  | assert_eq!(0, five.peek());
   |                    ^^^^ method not found in `std::ops::Range<{integer}>`
  ::: /checkout/library/core/src/iter/traits/peekable.rs:64:8
   |
   |
64 |     fn peek(&self) -> Option<Self::Item>;
   |        ---- the method is available for `std::ops::Range<{integer}>` here
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
4  | use std::iter::PeekableIterator;
4  | use std::iter::PeekableIterator;
   |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---- src/iter/traits/peekable.rs - iter::traits::peekable::PeekableIterator::peek (line 54) stdout ----
error[E0277]: can't compare `{integer}` with `Option<{integer}>`
   |
   |
10 | assert_eq!(0, five.peek());
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `{integer} == Option<{integer}>`
   |
   = help: the trait `PartialEq<Option<{integer}>>` is not implemented for `{integer}`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/iter/traits/peekable.rs - iter::traits::peekable::PeekableIterator::has_next (line 74) stdout ----
error[E0599]: no method named `has_next` found for struct `std::iter::Once` in the current scope
 --> src/iter/traits/peekable.rs:79:21
  |
8 | assert!(one_element.has_next());
  |                     ^^^^^^^^ method not found in `std::iter::Once<{integer}>`
error[E0599]: no method named `has_next` found for struct `std::iter::Once` in the current scope
  --> src/iter/traits/peekable.rs:82:22
   |
   |
11 | assert!(!one_element.has_next());
   |                      ^^^^^^^^ method not found in `std::iter::Once<{integer}>`
error: unused import: `std::iter::PeekableIterator`
 --> src/iter/traits/peekable.rs:76:5
  |
5 | use std::iter::PeekableIterator;
