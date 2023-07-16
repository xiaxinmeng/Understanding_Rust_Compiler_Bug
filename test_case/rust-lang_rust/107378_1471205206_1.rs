rust
error[[E0106]](https://doc.rust-lang.org/stable/error_codes/E0106.html): missing lifetime specifier
 --> src/lib.rs:1:67
  |
1 | async fn does_not_compile(mut iter: impl Iterator<Item = &()>) -> &() {
  |                                                                   ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
1 | async fn does_not_compile(mut iter: impl Iterator<Item = &()>) -> &'static () {
  |                                                                    +++++++

error: lifetime may not live long enough
 --> src/lib.rs:2:5
  |
1 | async fn does_not_compile(mut iter: impl Iterator<Item = &()>) -> &() {
  |                                                                   --- return type `impl Future<Output = &'static ()>` contains a lifetime `'1`
2 |     iter.next().unwrap()
  |     ^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'static`

error: lifetime may not live long enough
  --> src/lib.rs:10:5
   |
9  | async fn wrong_error(iter: &mut impl Iterator<Item = &()>) -> &() {
   |                            -                                  --- return type `impl Future<Output = &()>` contains a lifetime `'1`
   |                            |
   |                            let's call the lifetime of this reference `'2`
10 |     iter.next().unwrap()
   |     ^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`

For more information about this error, try `rustc --explain E0106`.
error: could not compile `playground` due to 3 previous errors
