
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
 --> f16.rs:6:24
  |
6 |     pub async fn start(&self) {
  |                        ^^^^^ this data with an anonymous lifetime `'_`...
7 |         require_static(async move {
  |         -------------- ...is required to live as long as `'static` here...
8 |             &self;
  |             ----- ...and is captured here
  |
note: `'static` obligation introduced by this bound
 --> f16.rs:1:22
  |
1 | fn require_static<T: 'static>(val: T) -> T { val }
  |                      ^^^^^^^
