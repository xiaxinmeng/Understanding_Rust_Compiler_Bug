
error: unsatisfied lifetime constraints
 --> free-willy.rs:2:41
  |
1 | fn willy<'w>(p: &'w str) -> &'w str {
  |          -- lifetime `'w` defined here
2 |     let free_dumb = |x: &str| -> &str { p };
  |                                         ^ returning this value requires that `'w` must outlive `'static`

error[E0597]: `hello` does not live long enough
 --> free-willy.rs:4:15
  |
2 |     let free_dumb = |x: &str| -> &str { p };
  |                                         - returning this value requires that `hello` is borrowed for `'static`
3 |     let hello = format!("Hello");
4 |     free_dumb(&hello)
  |               ^^^^^^ borrowed value does not live long enough
5 | }
  | - `hello` dropped here while still borrowed
