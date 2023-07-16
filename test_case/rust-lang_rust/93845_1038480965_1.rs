
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
  --> $DIR/issue-61949-self-return-type.rs:10:40
   |
LL | impl<'a> Foo<'a> {
   |      -- hidden type `impl Future<Output = [async output]>` captures the lifetime `'a` as defined here
LL |     pub async fn new(_bar: &'a i32) -> Self {
   |                                        ^^^^
