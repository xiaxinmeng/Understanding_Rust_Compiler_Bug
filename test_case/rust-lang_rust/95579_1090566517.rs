plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:881:23
    |
881 |     let v = vec![[(), usize::MAX]; 2];

error[E0658]: use of unstable library feature 'slice_flatten'
   --> library/alloc/tests/string.rs:882:15
    |
    |
882 |     let _ = v.into_flattened();
    |
    = note: see issue #95629 <https://github.com/rust-lang/rust/issues/95629> for more information
    = note: see issue #95629 <https://github.com/rust-lang/rust/issues/95629> for more information
    = help: add `#![feature(slice_flatten)]` to the crate attributes to enable
Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error[E0599]: the method `flatten` exists for array `[[(); 18446744073709551615]; 2]`, but its trait bounds were not satisfied
     |
     |
2512 |     let _ = x.flatten();
     |               ^^^^^^^ method cannot be called on `[[(); 18446744073709551615]; 2]` due to unsatisfied trait bounds
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `[[(); 18446744073709551615]; 2]: std::iter::Iterator`
             which is required by `&mut [[(); 18446744073709551615]; 2]: std::iter::Iterator`
             `[[(); 18446744073709551615]]: std::iter::Iterator`
             which is required by `&mut [[(); 18446744073709551615]]: std::iter::Iterator`

error[E0599]: no method named `flatten_mut` found for array `[[(); 18446744073709551615]; 2]` in the current scope
     |
     |
2519 |     let _ = x.flatten_mut();
     |               ^^^^^^^^^^^ method not found in `[[(); 18446744073709551615]; 2]`
For more information about this error, try `rustc --explain E0599`.
error: build failed
Build completed unsuccessfully in 0:01:29
