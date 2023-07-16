
[santiago@archlinux tmp]$ rustc +stage1 test.rs 
error[E0597]: `s` does not live long enough
  --> test.rs:7:24
   |
7  |     if let Some(n) = *(s.borrow_mut()) {
   |                       -^--------------
   |                       ||
   |                       |borrowed value does not live long enough
   |                       borrow may end up in a temporary, created here
...
10 | }
   | -
   | |
   | borrowed value only lives until here
   | temporary later dropped here, potentially using the reference

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
