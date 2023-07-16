
[santiago@archlinux tmp]$ rustc +stage1 test.rs 
error[E0597]: `i` does not live long enough
  --> test.rs:15:9
   |
15 |     Foo(&i).foo()
   |     ----^^-
   |     |   |
   |     |   borrowed value does not live long enough
   |     borrow may end up in a temporary, created here
16 | }
   | -
   | |
   | borrowed value only lives until here
   | temporary later dropped here, potentially using the reference

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
