
error: cannot infer an appropriate lifetime
  --> src/lib.rs:9:15
   |
8  |       fn test(&self) -> impl Future<Output=()> + 'static {
   |                         -------------------------------- this return type evaluates to the `'static` lifetime...
9  |           async {
   |  _______________^
10 | |             foo(self).await
11 | |         }
   | |_________^ ...but this borrow...
   |
note: ...can't outlive the anonymous lifetime #1 defined on the method body at 8:5
  --> src/lib.rs:8:5
   |
8  | /     fn test(&self) -> impl Future<Output=()> + 'static {
9  | |         async {
10 | |             foo(self).await
11 | |         }
12 | |     }
   | |_____^
help: you can add a constraint to the return type to make it last less than `'static` and match the anonymous lifetime #1 defined on the method body at 8:5
   |
8  |     fn test(&self) -> impl Future<Output=()> + 'static + '_ {
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

