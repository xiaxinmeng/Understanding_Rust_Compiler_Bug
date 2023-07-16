
error[E0597]: `xs` does not live long enough
 --> src/lib.rs:5:8
  |
1 | fn wrap_vec_in_closure<'a, T>(xs: &'a Vec<T>) -> (impl Fn() -> &'a Vec<T>)
  |                        --                         ----------------------- opaque type requires that `xs` is borrowed for `'a`
  |                        |
  |                        lifetime `'a` defined here
...
5 |     || xs
  |     -- ^^ borrowed value does not live long enough
  |     |
  |     value captured here
6 | }
  |  - `xs` dropped here while still borrowed
