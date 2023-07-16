
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
