
error: lifetime may not live long enough
 --> src/lib.rs:4:13
  |
4 |     foo(|x| baz(x))
  |          -- ^^^^^^ returning this value requires that `'1` must outlive `'2`
  |          ||
  |          |return type of closure is &'2 u8
  |          has type `&'1 u8`
