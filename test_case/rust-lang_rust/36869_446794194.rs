
error[E0621]: explicit lifetime required in the type of `x`
 --> src/lib.rs:1:35
  |
1 | fn foo(x: &Fn()) -> *const Fn() { x as _ }
  |           -----                   ^^^^^^ lifetime `'static` required
  |           |
  |           help: add explicit lifetime `'static` to the type of `x`: `&'static (dyn std::ops::Fn() + 'static)`
