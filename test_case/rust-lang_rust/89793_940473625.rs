plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0433]: failed to resolve: use of undeclared crate or module `slice`
    --> library/core/tests/slice.rs:2161:20
     |
2161 |         assert_eq!(slice::from_ptr_range(range), &arr);
     |                    ^^^^^ use of undeclared crate or module `slice`
error[E0433]: failed to resolve: use of undeclared crate or module `slice`
    --> library/core/tests/slice.rs:2167:20
     |
     |
2167 |         assert_eq!(slice::from_mut_ptr_range(range), &arr);
     |                    ^^^^^ use of undeclared crate or module `slice`
error[E0433]: failed to resolve: use of undeclared crate or module `slice`
    --> library/core/tests/slice.rs:2172:20
     |
     |
2172 |         assert_eq!(slice::from_ptr_range(range), &[] as &[Vec<String>]);
     |                    ^^^^^ use of undeclared crate or module `slice`
error[E0412]: cannot find type `Range` in this scope
    --> library/core/tests/slice.rs:2170:16
     |
     |
2170 |     let range: Range<*const Vec<String>> = [].as_ptr_range();
     |
help: consider importing one of these items
     |
1    | use core::ops::Range;
