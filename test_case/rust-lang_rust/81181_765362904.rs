
error[E0283]: type annotations needed
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/miniz_oxide-0.4.0/src/deflate/core.rs:1871:43
     |
1871 |         let far_and_small = cur_match_len == MIN_MATCH_LEN.into() && cur_match_dist >= 8 * 1024;
     |                                           ^^ -------------------- this method call resolves to `T`
     |                                           |
     |                                           cannot infer type
     |
     = note: cannot satisfy `u32: PartialEq<_>`

error[E0283]: type annotations needed
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/miniz_oxide-0.4.0/src/deflate/core.rs:2037:43
     |
2037 |                         || (cur_match_len == MIN_MATCH_LEN.into() && cur_match_dist >= 8 * 1024)
     |                                           ^^ -------------------- this method call resolves to `T`
     |                                           |
     |                                           cannot infer type
     |
     = note: cannot satisfy `u32: PartialEq<_>`

error: aborting due to 2 previous errors
