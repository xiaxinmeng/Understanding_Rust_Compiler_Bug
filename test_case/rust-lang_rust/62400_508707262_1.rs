
error: lifetime may not live long enough
 --> src/main.rs:4:5
  |
3 | fn foo<T>(x: &mut &mut T) {
  |              -    - let's call the lifetime of this reference `'2`
  |              |
  |              let's call the lifetime of this reference `'1`
4 |     *x = *x;
  |     ^^^^^^^ assignment requires that `'1` must outlive `'2`
