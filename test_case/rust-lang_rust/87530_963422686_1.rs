
error[E0277]: `NonNull<RcBox<()>>` cannot be shared between threads safely
  --> src/main.rs:22:19
   |
22 |     requires_sync(rc);
   |     ------------- ^^ `NonNull<RcBox<()>>` cannot be shared between threads safely
   |     |
   |     required by a bound introduced by this call
   |
   = help: within `Rc<()>`, the trait `Sync` is not implemented for `NonNull<RcBox<()>>`
note: required because it appears within the type `Rc<()>`
  --> src/main.rs:15:12
   |
15 | pub struct Rc<T: ?Sized> {
   |            ^^
note: required by a bound in `requires_sync`
  --> src/main.rs:25:21
   |
25 | fn requires_sync<T: Sync>(t: T) {
   |                     ^^^^ required by this bound in `requires_sync`

error[E0277]: `Cell<usize>` cannot be shared between threads safely
  --> src/main.rs:22:19
   |
22 |     requires_sync(rc);
   |     ------------- ^^ `Cell<usize>` cannot be shared between threads safely
   |     |
   |     required by a bound introduced by this call
   |
   = help: within `Rc<()>`, the trait `Sync` is not implemented for `Cell<usize>`
note: required because it appears within the type `RcBox<()>`
  --> src/main.rs:9:8
   |
9  | struct RcBox<T: ?Sized> {
   |        ^^^^^
   = note: required because it appears within the type `PhantomData<RcBox<()>>`
note: required because it appears within the type `Rc<()>`
  --> src/main.rs:15:12
   |
15 | pub struct Rc<T: ?Sized> {
   |            ^^
note: required by a bound in `requires_sync`
  --> src/main.rs:25:21
   |
25 | fn requires_sync<T: Sync>(t: T) {
   |                     ^^^^ required by this bound in `requires_sync`
