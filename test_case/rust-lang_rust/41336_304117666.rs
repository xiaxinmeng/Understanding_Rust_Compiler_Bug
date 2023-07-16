
error[E0282]: type annotations needed
   --> /root/.cargo/registry/src/github.com-1ecc6299db9ec823/zxcvbn-0.4.4/src/scoring.rs:381:25
    |
381 |                     let shifted_variations = (1..(cmp::min(shifted_count, unshifted_count) + 1))
    |                         ^^^^^^^^^^^^^^^^^^
    |                         |
    |                         consider giving `shifted_variations` a type
    |                         cannot infer type for `_`
