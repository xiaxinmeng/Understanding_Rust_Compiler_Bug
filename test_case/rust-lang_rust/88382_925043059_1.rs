
error[E0631]: type mismatch in closure arguments
  --> src/main.rs:39:40
   |
33 |         f: |i| {
   |            --- found signature of `for<'r> fn(&'r mut std::iter::Empty<usize>) -> _`
...
39 |     do_some_more(SomeImplementation(), do_something);
   |     ------------                       ^^^^^^^^^^^^ expected signature of `for<'r, 's> fn(&'r mut <SomeImplementation as Iterable>::Iterator<'s>) -> _`
   |     |
   |     required by a bound introduced by this call
   |
note: required by a bound in `do_some_more`
  --> src/main.rs:42:33
   |
42 | fn do_some_more<I: Iterable, F: Fn(&mut I::Iterator<'_>)>(
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `do_some_more`

For more information about this error, try `rustc --explain E0631`.
