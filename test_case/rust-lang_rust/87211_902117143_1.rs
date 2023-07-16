
error[E0277]: `&impl Future` is not a future
   --> main.rs:6:5
    |
6   |     fut.await;
    |     ^^^^^^^^^ `&impl Future` is not a future (mod)
    |
    = help: the trait `Future` is not implemented for `&impl Future`
note: required by `poll`
