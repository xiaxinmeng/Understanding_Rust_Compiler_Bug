
$ ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc ../src/test/compile-fail/issue-36082.rs -Z borrowck-mir
error[E0505]: cannot move out of `_` because it is borrowed (Mir)
  --> ../src/test/compile-fail/issue-36082.rs:18:32
   |
18 |     let val: &_ = x.borrow().0;
   |                   ----------   ^ move out of `_` occurs here
   |                   |
   |                   borrow of `_` occurs here
