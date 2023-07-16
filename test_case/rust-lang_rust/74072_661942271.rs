
error[E0506]: cannot assign to `*x` because it is borrowed
  --> issue-74072.rs:5:5
   |
LL | pub async fn f(x: &mut i32) -> &i32 {
   |                                ---- return type of async function is &'1 i32
LL |     let y = &*x;
   |             --- borrow of `*x` occurs here
LL |     *x += 1;
   |     ^^^^^^^ assignment to borrowed `*x` occurs here
LL |     y
   |     - returning this value requires that `*x` is borrowed for `'1`
