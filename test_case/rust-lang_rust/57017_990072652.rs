
error: future cannot be sent between threads safely
  --> src/main.rs:19:17
   |
19 | pub fn wat() -> impl Future + Send {
   |                 ^^^^^^^^^^^^^^^^^^ future created by async block is not `Send`
   |
   = help: the trait `Sync` is not implemented for `(dyn Any + Send + 'static)`
note: future is not `Send` as this value is used across an await
  --> src/main.rs:24:26
   |
22 |         match client.status() {
   |               ------ has type `&Client` which is not `Send`
23 |             200 => {
24 |                 let _x = get().await;
   |                          ^^^^^^^^^^^ await occurs here, with `client` maybe used later
...
28 |     }
   |     - `client` is later dropped here
help: consider moving this into a `let` binding to create a shorter lived borrow
  --> src/main.rs:22:15
   |
22 |         match client.status() {
   |               ^^^^^^^^^^^^^^^
