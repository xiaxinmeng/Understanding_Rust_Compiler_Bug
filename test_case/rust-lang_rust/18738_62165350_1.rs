
bar.rs:21:60: 21:74 error: mismatched types: expected `&str`, found `&&'static str` (expected str, found &-ptr)
bar.rs:21                     let __test = (*__self_0_0).partial_cmp(&(*__self_1_0));
                                                                     ^~~~~~~~~~~~~~
error: aborting due to previous error
