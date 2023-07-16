
error: lifetime may not live long enough
 --> src/main.rs:2:31
  |
2 |     let _closure = |s: &bool| s;
  |                        -    - ^ returning this value requires that `'1` must outlive `'2`
  |                        |    |
  |                        |    return type of closure is &'2 bool
  |                        let's call the lifetime of this reference `'1`

