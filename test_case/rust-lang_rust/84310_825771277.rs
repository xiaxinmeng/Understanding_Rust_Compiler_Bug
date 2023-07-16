
error[E0557]: feature has been removed
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/lock_api-0.4.1/src/lib.rs:91:42
   |
91 | #![cfg_attr(feature = "nightly", feature(const_fn))]
   |                                          ^^^^^^^^ feature has been removed
   |
   = note: replaced by finer-grained feature flags
