
error[E0658]: use of unstable library feature 'iterator_fold_mut'
    --> library/core/tests/iter.rs:3230:30
     |
3230 |     let result = nums.iter().fold_mut(Vec::new(), |v, i| {
     |                              ^^^^^^^^
     |
     = note: see issue #76725 <https://github.com/rust-lang/rust/issues/76725> for more information
     = help: add `#![feature(iterator_fold_mut)]` to the crate attributes to enable
