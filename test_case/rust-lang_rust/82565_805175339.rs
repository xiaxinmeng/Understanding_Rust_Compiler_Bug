
[INFO] [stdout] error[E0308]: mismatched types
[INFO] [stdout]    --> /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/lexical-core-0.4.0/src/atof/algorithm/bigcomp.rs:246:55
[INFO] [stdout]     |
[INFO] [stdout] 246 |     let nlz = den.leading_zeros().wrapping_sub(wlz) & (u32::BITS - 1);
[INFO] [stdout]     |                                                       ^^^^^^^^^^^^^^^ expected `usize`, found `u32`
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stdout] error[E0277]: no implementation for `usize & u32`
[INFO] [stdout]    --> /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/lexical-core-0.4.0/src/atof/algorithm/bigcomp.rs:246:53
[INFO] [stdout]     |
[INFO] [stdout] 246 |     let nlz = den.leading_zeros().wrapping_sub(wlz) & (u32::BITS - 1);
[INFO] [stdout]     |                                                     ^ no implementation for `usize & u32`
[INFO] [stdout]     |
[INFO] [stdout]     = help: the trait `BitAnd<u32>` is not implemented for `usize`
