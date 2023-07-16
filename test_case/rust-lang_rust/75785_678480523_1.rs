
LL | pub async fn async_fn(x: &mut i32) -> (&i32, &i32) {
   |                                        - let's call the lifetime of this reference `'1`
LL |     let y = &*x;
   |             --- borrow of `*x` occurs here
LL |     *x += 1;
   |     ^^^^^^^ assignment to borrowed `*x` occurs here
LL |     (&32, y)
   |     -------- returning this value requires that `*x` is borrowed for `'1`
