plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error[E0658]: use of unstable library feature 'split_as_slice'
     |
     |
2346 |     assert_eq!(split.as_slice(), &[1, 2, 3, 4, 5, 6]);
     |
     = note: see issue #96137 <https://github.com/rust-lang/rust/issues/96137> for more information
     = help: add `#![feature(split_as_slice)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'split_as_slice'
     |
     |
2348 |     assert_eq!(split.as_slice(), &[3, 4, 5, 6]);
     |
     = note: see issue #96137 <https://github.com/rust-lang/rust/issues/96137> for more information
     = help: add `#![feature(split_as_slice)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'split_as_slice'
     |
     |
2351 |     assert_eq!(split.as_slice(), &[]);
     |
     = note: see issue #96137 <https://github.com/rust-lang/rust/issues/96137> for more information
     = help: add `#![feature(split_as_slice)]` to the crate attributes to enable

