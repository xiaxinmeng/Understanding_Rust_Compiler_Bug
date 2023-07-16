
error[E0384]: cannot assign to immutable argument `p`
  --> f11.rs:10:5
   |
7  | fn a(p: &mut &S) {
   |      - help: consider making this binding mutable: `mut p`
...
10 |     p = &mut &number;
   |     ^^^^^^^^^^^^^^^^ cannot assign to immutable argument

error[E0716]: temporary value dropped while borrowed
  --> f11.rs:10:14
   |
7  | fn a(p: &mut &S) {
   |         - let's call the lifetime of this reference `'1`
...
10 |     p = &mut &number;
   |     ---------^^^^^^^- temporary value is freed at the end of this statement
   |     |        |
   |     |        creates a temporary value which is freed while still in use
   |     assignment requires that borrow lasts for `'1`

error[E0597]: `number` does not live long enough
  --> f11.rs:10:14
   |
7  | fn a(p: &mut &S) {
   |              - let's call the lifetime of this reference `'2`
8  |
9  |     let mut number = S;
   |         ----------   - this value is assigned to `number` and lives as long as `a`
   |         |
   |         this binding only lives for the duration of `a` and gets dropped at the function
10 |     p = &mut &number;
   |     ---------^^^^^^^
   |     |        |
   |     |        borrowed value does not live long enough
   |     assignment requires that `number` is borrowed for `'2`
11 | }
   | - `number` dropped here while still borrowed
