
error[E0412]: cannot find type `Foo` in this scope
 --> src/lib.rs:1:6
  |
1 | impl Foo {}
  |      ^^^ not found in this scope

warning: unreachable call
 --> src/lib.rs:5:13
  |
5 |     let _ = std::io::copy(&mut r, unimplemented!());
  |             ^^^^^^^^^^^^^         ---------------- any code following this expression is unreachable
  |             |
  |             unreachable call
  |
  = note: `#[warn(unreachable_code)]` on by default

error[E0283]: type annotations needed
  --> src/lib.rs:5:13
   |
5  |     let _ = std::io::copy(&mut r, unimplemented!());
   |             ^^^^^^^^^^^^^ cannot infer type for type parameter `W` declared on the function `copy`
   |
   = note: cannot satisfy `_: std::io::Write`
help: consider specifying the type arguments in the function call
   |
5  |     let _ = std::io::copy::<R, W>(&mut r, unimplemented!());
   |                          ^^^^^^^^
