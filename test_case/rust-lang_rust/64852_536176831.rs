
error[E0609]: no field `await` on type `impl Future<Output = ()>`
  --> /home/glon/Programs/rust/src/test/ui/async-await/suggest-switching-edition-on-await.rs:40:7
   |
LL | fn await_on_apit(x: impl Future<Output = ()>) {
   |                     ------------------------ type parameter 'impl Future<Output = ()>' declared here
LL |     x.await;
   |       ^^^^^
