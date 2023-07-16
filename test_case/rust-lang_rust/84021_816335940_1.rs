
   Compiling playground v0.0.1 (/playground)
error[E0195]: lifetime parameters or bounds on method `boo` do not match the trait declaration
 --> src/lib.rs:6:11
  |
2 |     fn boo<'ctx>();
  |           ------ lifetimes in impl do not match this method in trait
...
6 |     fn boo<'ctx>() where i32: 'ctx { }
  |           ^^^^^^ lifetimes do not match method in trait

error: aborting due to previous error
