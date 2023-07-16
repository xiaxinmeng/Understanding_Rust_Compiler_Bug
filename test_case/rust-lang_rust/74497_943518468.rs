
error: lifetime may not live long enough
 --> src/lib.rs:4:13
  |
4 |     foo(|x| baz(x)).await;
  |          -- ^^^^^^ returning this value requires that `'1` must outlive `'2`
  |          ||
  |          |return type of closure `impl Future` contains a lifetime `'2`
  |          has type `&'1 u8`
