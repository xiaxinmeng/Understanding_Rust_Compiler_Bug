
error[E0277]: cannot subtract `usize` from `u32`
    --> $CARGO_REGISTRY_PATH/lexical-core-0.7.4/src/atof/algorithm/math.rs:2065:25
     |
2065 |     let rs = Limb::BITS - s;
     |                         ^ no implementation for `u32 - usize`
     |
     = help: the trait `Sub<usize>` is not implemented for `u32`
 