
error[E0658]: use of unstable library feature 'int_roundings'
   --> /home/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.3.2/src/biguint.rs:393:45
    |
393 |                 let root_scale = extra_bits.div_ceil(&n64);
    |                                             ^^^^^^^^
    |
    = note: see issue #88581 <https://github.com/rust-lang/rust/issues/88581> for more information
    = help: add `#![feature(int_roundings)]` to the crate attributes to enable
error[E0308]: mismatched types
   --> /home/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.3.2/src/biguint.rs:393:54
    |
393 |                 let root_scale = extra_bits.div_ceil(&n64);
    |                                                      ^^^^ expected `u64`, found `&u64`
    |
help: consider removing the borrow
    |
393 -                 let root_scale = extra_bits.div_ceil(&n64);
393 +                 let root_scale = extra_bits.div_ceil(n64);
    | 
error[E0658]: use of unstable library feature 'int_roundings'
  --> /home/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.3.2/src/biguint/convert.rs:70:10
   |
70 |         .div_ceil(&big_digit::BITS.into())
   |          ^^^^^^^^
   |
   = note: see issue #88581 <https://github.com/rust-lang/rust/issues/88581> for more information
   = help: add `#![feature(int_roundings)]` to the crate attributes to enable
error[E0308]: mismatched types
  --> /home/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.3.2/src/biguint/convert.rs:70:19
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
   --> /home/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.3.2/src/biguint/convert.rs:585:10
    |
585 |         .div_ceil(&u64::from(bits))
    |          ^^^^^^^^
    |
    = note: see issue #88581 <https://github.com/rust-lang/rust/issues/88581> for more information
    = help: add `#![feature(int_roundings)]` to the crate attributes to enable
error[E0308]: mismatched types
   --> /home/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.3.2/src/biguint/convert.rs:585:19
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
   --> /home/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.3.2/src/biguint/convert.rs:613:10
    |
613 |         .div_ceil(&u64::from(bits))
    |          ^^^^^^^^
    |
    = note: see issue #88581 <https://github.com/rust-lang/rust/issues/88581> for more information
    = help: add `#![feature(int_roundings)]` to the crate attributes to enable
error[E0308]: mismatched types
   --> /home/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.3.2/src/biguint/convert.rs:613:19
    |
613 |         .div_ceil(&u64::from(bits))
    |                   ^^^^^^^^^^^^^^^^ expected `u64`, found `&u64`
    |
help: consider removing the borrow
    |
613 -         .div_ceil(&u64::from(bits))
613 +         .div_ceil(u64::from(bits))
    | 
    
