
error[E0597]: `number` does not live long enough
 --> f11.rs:5:10
  |
2 | fn f(p: &mut &S) {
  |              - let's call the lifetime of this reference `'1`
3 |
4 |     let mut number = S;
  |         ----------   - this value is assigned to `number` and lives as long as `f`
  |         |
  |         this binding only lives for the duration of `f` and gets dropped at the function
5 |     *p = &number;
  |     -----^^^^^^^
  |     |    |
  |     |    borrowed value does not live long enough
  |     assignment requires that `number` is borrowed for `'1`
6 | }
  | - `number` dropped here while still borrowed
