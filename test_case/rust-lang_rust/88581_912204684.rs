
error[E0658]: use of unstable library feature 'int_roundings'
   --> /home/fourbytes/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.4.1/src/biguint.rs:398:45
    |
398 |                 let root_scale = extra_bits.div_ceil(&n64);
    |                                             ^^^^^^^^
    |
    = note: see issue #88581 <https://github.com/rust-lang/rust/issues/88581> for more information
    = help: add `#![feature(int_roundings)]` to the crate attributes to enable

error[E0308]: mismatched types
   --> /home/fourbytes/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.4.1/src/biguint.rs:398:54
    |
398 |                 let root_scale = extra_bits.div_ceil(&n64);
    |                                                      ^^^^ expected `u64`, found `&u64`
    |
help: consider removing the borrow
    |
398 -                 let root_scale = extra_bits.div_ceil(&n64);
398 +                 let root_scale = extra_bits.div_ceil(n64);
    |

error[E0658]: use of unstable library feature 'int_roundings'
  --> /home/fourbytes/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.4.1/src/biguint/convert.rs:70:10
   |
70 |         .div_ceil(&big_digit::BITS.into())
   |          ^^^^^^^^
   |
   = note: see issue #88581 <https://github.com/rust-lang/rust/issues/88581> for more information
   = help: add `#![feature(int_roundings)]` to the crate attributes to enable

error[E0308]: mismatched types
  --> /home/fourbytes/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.4.1/src/biguint/convert.rs:70:19
   |
70 |         .div_ceil(&big_digit::BITS.into())
   |                   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u64`, found reference
   |
   = note:   expected type `u64`
           found reference `&_`
help: consider removing the borrow
   |
70 -         .div_ceil(&big_digit::BITS.into())
70 +         .div_ceil(big_digit::BITS.into())
   |

error[E0658]: use of unstable library feature 'int_roundings'
   --> /home/fourbytes/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.4.1/src/biguint/convert.rs:585:10
    |
585 |         .div_ceil(&u64::from(bits))
    |          ^^^^^^^^
    |
    = note: see issue #88581 <https://github.com/rust-lang/rust/issues/88581> for more information
    = help: add `#![feature(int_roundings)]` to the crate attributes to enable

error[E0308]: mismatched types
   --> /home/fourbytes/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.4.1/src/biguint/convert.rs:585:19
    |
585 |         .div_ceil(&u64::from(bits))
    |                   ^^^^^^^^^^^^^^^^ expected `u64`, found `&u64`
    |
help: consider removing the borrow
    |
585 -         .div_ceil(&u64::from(bits))
585 +         .div_ceil(u64::from(bits))
    |

error[E0658]: use of unstable library feature 'int_roundings'
   --> /home/fourbytes/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.4.1/src/biguint/convert.rs:613:10
    |
613 |         .div_ceil(&u64::from(bits))
    |          ^^^^^^^^
    |
    = note: see issue #88581 <https://github.com/rust-lang/rust/issues/88581> for more information
    = help: add `#![feature(int_roundings)]` to the crate attributes to enable

error[E0308]: mismatched types
   --> /home/fourbytes/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.4.1/src/biguint/convert.rs:613:19
    |
613 |         .div_ceil(&u64::from(bits))
    |                   ^^^^^^^^^^^^^^^^ expected `u64`, found `&u64`
    |
help: consider removing the borrow
    |
613 -         .div_ceil(&u64::from(bits))
613 +         .div_ceil(u64::from(bits))
    |

Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `num-bigint` due to 8 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
