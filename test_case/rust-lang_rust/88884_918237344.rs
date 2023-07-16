
  |
3 | fn do_nothing<'z>(arg: &'z ()) {
  |               -- lifetime `'z` defined here
4 |     (|s: &'z ()| s)(ident(&arg));
  |                     ------^^^^-
  |                     |     |
  |                     |     borrowed value does not live long enough
  |                     argument requires that `arg` is borrowed for `'z`
5 | }
  | - `arg` dropped here while still borrowed
