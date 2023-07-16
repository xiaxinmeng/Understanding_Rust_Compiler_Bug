
error: lifetime may not live long enough
 --> src/main.rs:4:5
  |
3 | async fn async_fn(x: &i32, y: &mut &i32) {
  |                      -             - let's call the lifetime of this reference `'2`
  |                      |
  |                      let's call the lifetime of this reference `'1`
4 |     *y = x;
  |     ^^^^^^ assignment requires that `'1` must outlive `'2`

error: lifetime may not live long enough
  --> src/main.rs:11:9
   |
10 |     async fn async_fn(self: &Struct, f: &u32) -> &u32 {
   |                             -           - let's call the lifetime of this reference `'1`
   |                             |
   |                             let's call the lifetime of this reference `'2`
11 |         f
   |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
