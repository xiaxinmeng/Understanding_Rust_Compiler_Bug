
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> $DIR/issue-108680.rs:9:5
   |
LL |     foo();
   |     ^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target features: neon aes sha2 and 2 more.
   = note: can only be called if the required target features are available

error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

