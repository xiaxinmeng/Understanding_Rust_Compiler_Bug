
error: cannot infer an appropriate lifetime
 --> src/lib.rs:7:9
  |
6 | fn foo<'a>(path: &'a Path) -> impl RR + 'static {
  |                               ----------------- this return type evaluates to the `'static` lifetime...
7 |     bar(path)
  |         ^^^^ ...but this borrow...
  |
note: ...can't outlive the lifetime 'a as defined on the function body at 6:8
 --> src/lib.rs:6:8
  |
6 | fn foo<'a>(path: &'a Path) -> impl RR + 'static {
  |        ^^
help: you can add a constraint to the return type to make it last less than `'static` and match the lifetime 'a as defined on the function body at 6:8
  |
6 | fn foo<'a>(path: &'a Path) -> impl RR + 'static + 'a {
  |                               ^^^^^^^^^^^^^^^^^^^^^^
