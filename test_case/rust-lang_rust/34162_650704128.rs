rust
error: lifetime may not live long enough
  --> src/main.rs:13:29
   |
13 |     clients.sort_by_key(|c| c.key());
   |                          -- ^^^^^^^ returning this value requires that `'1` must outlive `'2`
   |                          ||
   |                          |return type of closure is &'2 str
   |                          has type `&'1 Client`
