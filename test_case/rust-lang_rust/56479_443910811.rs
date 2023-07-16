
error: borrow being returned doesn't live long enough
  --> $DIR/escape-argument-callee.rs:36:45
   |
LL |         let mut closure = expect_sig(|p, y| *p = y);
   |                                       -  -  ^^^^^^ requires that `'1` must outlive `'2`
   |                                       |  |
   |                                       |  has type `&'1 i32`
   |                                       has type `&mut &'2 i32`
