
error[E0277]: expected a `Fn<(&PanicInfo<'_>,)>` closure, found `for<'a, 'b> fn(&'a PanicInfo<'b>) {panic}`
  --> src/main.rs:11:26
   |
11 |     std::panic::set_hook(Box::new(panic));
   |                          ^^^^^^^^^^^^^^^ expected an `Fn<(&PanicInfo<'_>,)>` closure, found `for<'a, 'b> fn(&'a PanicInfo<'b>) {panic}`
   |
   = help: the trait `for<'a, 'b> Fn<(&'a PanicInfo<'b>,)>` is not implemented for fn item `for<'a, 'b> fn(&'a PanicInfo<'b>) {panic}`
   = note: `#[target_feature]` functions do not implement the `Fn` traits
   = note: required for the cast from `for<'a, 'b> fn(&'a PanicInfo<'b>) {panic}` to the object type `dyn for<'a, 'b> Fn(&'a PanicInfo<'b>) + Send + Sync`
