
error: lifetime bound not satisfied
  --> src/lib.rs:10:9
   |
10 | /         async move {
11 | |             self.run::<'a, 'b, T>(t).await;
12 | |         }
   | |_________^
   |
note: the lifetime `'a` defined here...
  --> src/lib.rs:9:16
   |
9  |     fn run_via<'a, 'b: 'a, T: Sync>(&'a self, t: &'b T) -> impl Future<Output = ()> + 'a + Send {
   |                ^^
note: ...must outlive the lifetime `'a` defined here
  --> src/lib.rs:9:16
   |
9  |     fn run_via<'a, 'b: 'a, T: Sync>(&'a self, t: &'b T) -> impl Future<Output = ()> + 'a + Send {
   |                ^^
   = note: this is a known limitation that will be removed in the future ([see issue #100013 <https://github.com/rust-lang/rust/issues/100013>](https://github.com/rust-lang/rust/issues/100013) for more information)
