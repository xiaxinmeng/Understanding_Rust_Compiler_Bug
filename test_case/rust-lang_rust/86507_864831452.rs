
error: future cannot be sent between threads safely
  --> src/main.rs:11:47
   |
11 |       async fn bar(x: &(impl Serialize + Send)) {
   |  _______________________________________________^
12 | |     }
   | |_____^ future created by async block is not `Send`
   |
note: `&` references cannot be sent unless their referent is `Sync`
  --> src/main.rs:11:18
   |
11 |     async fn bar(x: &(impl Serialize + Send)) {
   |                  ^ refers to `impl Serialize + Send` which is not `Sync`
   = note: required for the cast to the object type `dyn Future<Output = ()> + Send`
help: consider further restricting this bound
   |
11 |     async fn bar(x: &(impl Serialize + Send + std::marker::Sync)) {
   |                                             ^^^^^^^^^^^^^^^^^^^
