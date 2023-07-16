
error[E0597]: `buf` does not live long enough
  --> src/lib.rs:9:19
   |
8  | fn foo<'a>(buf: &'a mut str) -> Db<'a> {
   |        -- lifetime `'a` defined here
9  |     Db::<'a>::new(&buf)
   |     --------------^^^^-
   |     |             |
   |     |             borrowed value does not live long enough
   |     argument requires that `buf` is borrowed for `'a`
10 | }
   | - `buf` dropped here while still borrowed
