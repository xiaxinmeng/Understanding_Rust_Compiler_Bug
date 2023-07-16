
error[E0599]: no method named `unwrap` found for type `bool` in the current scope
   --> src/tools/miri/src/shims/intrinsics.rs:521:76
    |
521 |                     && !layout.might_permit_raw_init(this, /*zero:*/ true).unwrap()
    |                                                                            ^^^^^^ method not found in `bool`

error[E0599]: no method named `unwrap` found for type `bool` in the current scope
   --> src/tools/miri/src/shims/intrinsics.rs:529:77
    |
529 |                     && !layout.might_permit_raw_init(this, /*zero:*/ false).unwrap()
    |                                                                             ^^^^^^ method not found in `bool`

