
[...]
error[E0308]: mismatched types
    --> /home/marcel/.cargo/registry/src/github.com-1ecc6299db9ec823/lexical-core-0.7.4/src/atof/algorithm/math.rs:2065:27
     |
2065 |     let rs = Limb::BITS - s;
     |                           ^ expected `u32`, found `usize`

error[E0277]: cannot subtract `usize` from `u32`
    --> /home/marcel/.cargo/registry/src/github.com-1ecc6299db9ec823/lexical-core-0.7.4/src/atof/algorithm/math.rs:2065:25
     |
2065 |     let rs = Limb::BITS - s;
     |                         ^ no implementation for `u32 - usize`
     |
     = help: the trait `Sub<usize>` is not implemented for `u32`
