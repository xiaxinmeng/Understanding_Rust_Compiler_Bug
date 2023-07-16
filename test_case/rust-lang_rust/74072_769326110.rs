
error[E0506]: cannot assign to `*x` because it is borrowed
 --> src/lib.rs:3:5
  |
1 | pub async fn f(x: &mut i32) -> &i32 {
  |                   - let's call the lifetime of this reference `'1`
2 |     let y = &*x;
  |             --- borrow of `*x` occurs here
3 |     *x += 1;
  |     ^^^^^^^ assignment to borrowed `*x` occurs here
4 |     y
  |     - returning this value requires that `*x` is borrowed for `'1`
