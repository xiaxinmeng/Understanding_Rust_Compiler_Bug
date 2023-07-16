
[00:34:56] error[E0658]: use of unstable library feature 'stdsimd' (see issue #27731)
[00:34:56]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.1.2/src/specialized/pclmulqdq.rs:13:12
[00:34:56]    |
[00:34:56] 13 |         if is_x86_feature_detected!("pclmulqdq")
[00:34:56]    |
[00:34:56]    |
[00:34:56]    = help: add #![feature(stdsimd)] to the crate attributes to enable
