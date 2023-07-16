
error: cannot infer an appropriate lifetime
  --> $DIR/static-return-lifetime-infered.rs:21:16
   |
LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> {
   |                                     ----------------------- this return type evaluates to the `'static` lifetime...
LL |         self.x.iter().map(|a| a.0)
   |         ------ ^^^^
   |         |
   |         ...but this borrow...
   |
note: ...can't outlive the lifetime 'a as defined on the method body at 20:5
  --> $DIR/static-return-lifetime-infered.rs:20:5
   |
LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: you can add a constraint to the return type to make it last less than `'static` and match the lifetime 'a as defined on the method body at 20:5
   |
LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> + 'a {
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
