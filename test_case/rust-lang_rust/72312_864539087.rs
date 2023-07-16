txt
   Compiling playground v0.0.1 (/playground)
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
 --> src/main.rs:6:24
  |
6 |     pub async fn start(&self) {
  |                        ^^^^^
  |                        |
  |                        this data with an anonymous lifetime `'_`...
  |                        ...is captured here...
7 |         require_static(async move {
  |         -------------- ...and is required to live as long as `'static` here
