
warning: unused variable: `x`
 --> src/lib.rs:5:9
  |
5 |     let x = unsafe { x.offset(-1) };
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: constant is never used: `FOO`
 --> src/lib.rs:2:1
  |
2 | / const FOO: () = {
3 | |     let v = [0i8; 4];
4 | |     let x = &v as *const i8;
5 | |     let x = unsafe { x.offset(-1) };
6 | | };
  | |__^
  |
  = note: `#[warn(dead_code)]` on by default

error[E0080]: evaluation of constant value failed
    | 
   ::: src/lib.rs:5:22
    |
5   |     let x = unsafe { x.offset(-1) };
    |                      ------------ inside `FOO` at src/lib.rs:5:22

For more information about this error, try `rustc --explain E0080`.
error: could not compile `playground` due to previous error; 2 warnings emitted
