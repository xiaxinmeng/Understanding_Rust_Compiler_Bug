
error[E0631]: type mismatch in function arguments
  --> src/main.rs:23:14
   |
13 | fn sanitize_by_reference(s: &()) -> () {
   | --------------------------------------
   | |                           |
   | |                           help: do not borrow the argument: `()`
   | found signature defined here
...
23 |         .map(sanitize_by_reference)
   |          --- ^^^^^^^^^^^^^^^^^^^^^ expected due to this
   |          |
   |          required by a bound introduced by this call
   |
   = note: expected function signature `fn(()) -> _`
              found function signature `for<'a> fn(&'a ()) -> _`
note: required by a bound in `map`
  --> /rustc/270c94e484e19764a2832ef918c95224eb3f17c7/library/core/src/iter/traits/iterator.rs:777:5

error[E0599]: the method `collect` exists for struct `Map<impl Iterator<Item = ()> + '_, for<'a> fn(&'a ()) {sanitize_by_reference}>`, but its trait bounds were not satisfied
  --> src/main.rs:26:10
   |
26 |         .collect::<Vec<_>>();
   |          ^^^^^^^ method cannot be called due to unsatisfied trait bounds
  --> /rustc/270c94e484e19764a2832ef918c95224eb3f17c7/library/core/src/iter/adapters/map.rs:61:1
   |
   = note: doesn't satisfy `_: Iterator`
   |
   = note: the following trait bounds were not satisfied:
           `<for<'a> fn(&'a ()) {sanitize_by_reference} as FnOnce<((),)>>::Output = _`
           which is required by `Map<impl Iterator<Item = ()> + '_, for<'a> fn(&'a ()) {sanitize_by_reference}>: Iterator`
           `for<'a> fn(&'a ()) {sanitize_by_reference}: FnMut<((),)>`
           which is required by `Map<impl Iterator<Item = ()> + '_, for<'a> fn(&'a ()) {sanitize_by_reference}>: Iterator`
           `Map<impl Iterator<Item = ()> + '_, for<'a> fn(&'a ()) {sanitize_by_reference}>: Iterator`
           which is required by `&mut Map<impl Iterator<Item = ()> + '_, for<'a> fn(&'a ()) {sanitize_by_reference}>: Iterator`
