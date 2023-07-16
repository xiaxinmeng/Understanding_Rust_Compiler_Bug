
error[E0597]: `number` does not live long enough
  --> f11.rs:14:9
   |
12 | fn g(mut p: &mut S) {
   |             - let's call the lifetime of this reference `'1`
13 |     let mut number = S;
   |         ----------   - this value is assigned to `number` and lives as long as `g`
   |         |
   |         this binding only lives for the duration of `g` and gets dropped at the function
14 |     p = &mut number;
   |     ----^^^^^^^^^^^
   |     |   |
   |     |   borrowed value does not live long enough
   |     assignment requires that `number` is borrowed for `'1`
15 | }
   | - `number` dropped here while still borrowed
