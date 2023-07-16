
error[E0512]: transmute called with two types that may have different sizes
 --> <anon>:4:25
  |
4 |       unsafe { takes_u8(::std::mem::transmute(0u16)); }
  |                         ^^^^^^^^^^^^^^^^^^^^^ source and target type of transmute may have different sizes
  | note: source type is `u16`
          16 bits
  | note: target type is `u8`
          8 bits
