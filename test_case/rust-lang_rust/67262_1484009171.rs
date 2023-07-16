
error: lifetime may not live long enough
 --> src/lib.rs:8:23
  |
8 |     Wrapper(|x: &u32| x)
  |                 -   - ^ returning this value requires that `'1` must outlive `'2`
  |                 |   |
  |                 |   return type of closure is &'2 u32
  |                 let's call the lifetime of this reference `'1`

error: higher-ranked lifetime error
 --> src/lib.rs:8:5
  |
8 |     Wrapper(|x: &u32| x)
  |     ^^^^^^^^^^^^^^^^^^^^
  