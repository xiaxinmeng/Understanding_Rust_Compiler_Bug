
error[E0597]: `bar` does not live long enough
 --> src/lib.rs:5:8
  |
4 | fn foo(bar: Bar) -> impl FnOnce() -> Bar {
  |                     -------------------- opaque type requires that `bar` is borrowed for `'static`
5 |     || bar
  |     -- ^^^ borrowed value does not live long enough
  |     |
  |     value captured here
6 | }
  |  - `bar` dropped here while still borrowed
