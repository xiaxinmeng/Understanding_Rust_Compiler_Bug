
error[E0308]: mismatched types
  --> C:\Users\<user>\.cargo\registry\src\github.com-1ecc6299db9ec823\lexical-core-0.7.4\src\atof\algorithm\bhcomp.rs:62:24
   |
62 |     let bytes = bits / Limb::BITS;
   |                        ^^^^^^^^^^ expected `usize`, found `u32`

error[E0277]: cannot divide `usize` by `u32`
  --> C:\Users\<user>\.cargo\registry\src\github.com-1ecc6299db9ec823\lexical-core-0.7.4\src\atof\algorithm\bhcomp.rs:62:22
   |
62 |     let bytes = bits / Limb::BITS;
   |                      ^ no implementation for `usize / u32`
   |
   = help: the trait `Div<u32>` is not implemented for `usize`
