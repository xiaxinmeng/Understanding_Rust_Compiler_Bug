
   Compiling playground v0.0.1 (/playground)
error: future cannot be sent between threads safely
  --> src/lib.rs:15:17
   |
15 | pub fn wat() -> impl Future + Send {
   |                 ^^^^^^^^^^^^^^^^^^ future created by async block is not `Send`
   |
   = help: the trait `Sync` is not implemented for `(dyn Any + Send + 'static)`
note: future is not `Send` as this value is used across an await
  --> src/lib.rs:20:31
   |
18 |         match client.status() {
   |               ------ has type `&Client` which is not `Send`
19 |             200 => {
20 |                 let _x = get().await;
   |                               ^^^^^^ await occurs here, with `client` maybe used later
...
24 |     }
   |     - `client` is later dropped here
help: consider moving this into a `let` binding to create a shorter lived borrow
  --> src/lib.rs:18:15
   |
18 |         match client.status() {
   |               ^^^^^^^^^^^^^^^

error: could not compile `playground` due to previous error
