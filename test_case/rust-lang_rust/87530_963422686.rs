
error[E0277]: `Rc<()>` cannot be shared between threads safely
  --> src/main.rs:24:19
   |
24 |     requires_sync(rc);
   |     ------------- ^^ `Rc<()>` cannot be shared between threads safely
   |     |
   |     required by a bound introduced by this call
   |
   = help: the trait `Sync` is not implemented for `Rc<()>`
note: required by a bound in `requires_sync`
  --> src/main.rs:27:21
   |
27 | fn requires_sync<T: Sync>(t: T) {
   |                     ^^^^ required by this bound in `requires_sync`
