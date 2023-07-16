
error[E0506]: cannot assign to `a.foo` because it is borrowed
 --> src/lib.rs:9:5
  |
5 | pub fn foo1(mut a: &mut Demo) {
  |                    - let's call the lifetime of this reference `'1`
6 |     if let Some(ref mut b) = *&mut a.foo {
  |                 ---------     ---------- borrow of `a.foo` occurs here
  |                 |
  |                 assignment requires that `a.foo` is borrowed for `'1`
...
9 |     a.foo = None;
  |     ^^^^^ assignment to borrowed `a.foo` occurs here
