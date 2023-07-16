
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
  --> f9.rs:8:24
   |
 8 |     pub async fn start(&self) {
   |                        ^^^^^ this data with an anonymous lifetime `'_`...
 9 |         require_static(async move {
   |         -------------- ...is required to live as long as `'static` here...
10 |             &self;
   |             ----- ...because it is captured here
help: consider using a reference counter wrapper to avoid the lifetime issues
 8 -     pub async fn start(&self) {
 8 +     pub async fn start(self: Arc<Self>) {
