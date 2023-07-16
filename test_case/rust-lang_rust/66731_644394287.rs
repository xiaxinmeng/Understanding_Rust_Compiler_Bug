
error[E0277]: the trait bound `(): std::future::Future` is not satisfied
   --> src/main.rs:6:5
    |
6   |     boo().await;
    |     ^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `()`
help: maybe you want to make `boo` an async fn?
3  |   async fn boo (){}
