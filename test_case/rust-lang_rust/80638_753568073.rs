
error[E0277]: expected a `FnOnce<()>` closure, found `{integer}`
 --> src/main.rs:6:5
  |
1 | fn closure_ret_closure<T: FnOnce() -> T>(f: T) -> T {
  |                           ------------- required by this bound in `closure_ret_closure`
...
6 |     closure_ret_closure(|| 4);
  |                            ^ expected an `FnOnce<()>` closure, found `{integer}`
  |
  = help: the trait `FnOnce<()>` is not implemented for `{integer}`
  = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
