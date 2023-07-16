
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
  --> $DIR/issue-62097.rs:12:31
   |
LL |     pub async fn run_dummy_fn(&self) {
   |                               ^^^^^ this data with an anonymous lifetime `'_`...
LL |         foo(|| self.bar()).await;
   |         --- ...and is required to live as long as `'static` here
   |
note: data is captured here
  --> $DIR/issue-62097.rs:13:9
   |
LL |         foo(|| self.bar()).await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^
note: data is captured here
  --> $DIR/issue-62097.rs:13:9
   |
LL |         foo(|| self.bar()).await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^
note: data is captured here
  --> $DIR/issue-62097.rs:13:9
   |
LL |         foo(|| self.bar()).await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^
note: `'static` obligation introduced by this bound
  --> $DIR/issue-62097.rs:4:19
   |
LL |     F: FnOnce() + 'static
   |                   ^^^^^^^
