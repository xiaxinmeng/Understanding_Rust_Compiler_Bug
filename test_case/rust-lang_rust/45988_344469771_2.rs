
error[E0506]: cannot write to `x` while borrowed
 --> main:15:5
   |
15 |     let wrap = Wrap { p: &mut x };
   |                               - borrow of `x` occurs here (into `wrap`)
17 |     let foo = Foo { a: s, b: wrap };
   |                              ---- borrow moved here (into `foo`)
18 |     x = 1;
   |     ^^^^^^ write to `x` occurs here, while borrow is still active
 5 |     println!("{:?}", foo);
   |                      --- borrow is later used here
