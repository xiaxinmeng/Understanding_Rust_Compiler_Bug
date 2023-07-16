
error[E0277]: `()` is not a future
  --> $DIR/unnecessary-await.rs:9:10
   |
LL |     boo().await;
   |     -----^^^^^^ `()` is not a future
   |     |
   |     this call returns `()`
   |
   = help: the trait `Future` is not implemented for `()`
help: do not `.await` the expression
   |
LL -     boo().await;
LL +     boo();
   | 
help: alternatively, consider making `fn boo` asynchronous
   |
LL | async fn boo () {}
   | +++++
