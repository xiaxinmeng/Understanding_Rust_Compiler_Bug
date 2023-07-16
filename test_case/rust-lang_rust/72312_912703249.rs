
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
 --> f9.rs:8:24
  |
8 |     pub async fn start(&self) {
  |                        ^^^^^
  |                        |
  |                        this data with an anonymous lifetime `'_`...
  |                        ...is captured here...
9 |         require_static(async move {
  |         -------------- ...and is required to live as long as `'static` here
