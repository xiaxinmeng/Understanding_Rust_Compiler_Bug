
error[[E0525]](https://doc.rust-lang.org/nightly/error-index.html#E0525): expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
  --> src/lib.rs:6:9
   |
6  |         move || {
   |         ^^^^^^^ this closure implements `FnMut`, not `Fn`
7  |             i += 1;
   |             - closure is `FnMut` because it mutates the variable `i` here
...
11 |     is_fn(f);
   |     ----- - the requirement to implement `Fn` derives from here
   |     |
   |     required by a bound introduced by this call
   |
note: required by a bound in `is_fn`
  --> src/lib.rs:1:13
   |
1  | fn is_fn<F: Fn()>(_: F) {}
   |             ^^^^ required by this bound in `is_fn`

For more information about this error, try `rustc --explain E0525`.
