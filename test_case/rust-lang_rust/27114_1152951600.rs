
   Compiling playground v0.0.1 (/playground)
error: lifetime may not live long enough
 --> src/main.rs:4:5
  |
3 | fn bar<'b>() {
  |        -- lifetime `'b` defined here
4 |     foo::<&'b i32>();
  |     ^^^^^^^^^^^^^^ requires that `'b` must outlive `'static`

error: could not compile `playground` due to previous error
