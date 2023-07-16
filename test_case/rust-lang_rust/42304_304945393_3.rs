
error[E0512]: transmute called with differently sized types: u16 (16 bits) to u8 (8 bits)
 --> <anon>:4:25
  |
4 |       unsafe { takes_u8(::std::mem::transmute(0u16)); }
  |                         ^^^^^^^^^^^^^^^^^^^^^ transmuting between 16 bits and 8 bits
