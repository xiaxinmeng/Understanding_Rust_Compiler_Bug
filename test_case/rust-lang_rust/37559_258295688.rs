
   │0x54aad71c <core::f32::{{impl}}::powi+4>        mov    r11, sp                                                                                                                  │
   │0x54aad720 <core::f32::{{impl}}::powi+8>        sub    sp, sp, #24                                                                                                              │
   │0x54aad724 <core::f32::{{impl}}::powi+12>       vstr   s0, [r11, #-4]                                                                                                           │
   │0x54aad728 <core::f32::{{impl}}::powi+16>       str    r0, [r11, #-8]                                                                                                           │
   │0x54aad72c <core::f32::{{impl}}::powi+20>       vldr   s0, [r11, #-4]                                                                                                           │
   │0x54aad730 <core::f32::{{impl}}::powi+24>       vstr   s0, [sp, #12]                                                                                                            │
   │0x54aad734 <core::f32::{{impl}}::powi+28>       ldr    r0, [r11, #-8]                                                                                                           │
   │0x54aad738 <core::f32::{{impl}}::powi+32>       str    r0, [sp, #8]                                                                                                             │
   │0x54aad73c <core::f32::{{impl}}::powi+36>       vldr   s0, [sp, #12]                                                                                                            │
   │0x54aad740 <core::f32::{{impl}}::powi+40>       ldr    r0, [sp, #8]                                                                                                             │
  >│0x54aad744 <core::f32::{{impl}}::powi+44>       blx    0x54ad5480 <__powisf2>                                                                                                   │
   │0x54aad748 <core::f32::{{impl}}::powi+48>       vstr   s0, [sp, #4]                                                                                                             │
   │0x54aad74c <core::f32::{{impl}}::powi+52>       vldr   s0, [sp, #4]                                                                                                             │
   │0x54aad750 <core::f32::{{impl}}::powi+56>       vstr   s0, [sp]                                                                                                                 │
   │0x54aad754 <core::f32::{{impl}}::powi+60>       vldr   s0, [sp]                                                                                                                 │
   │0x54aad758 <core::f32::{{impl}}::powi+64>       mov    sp, r11
