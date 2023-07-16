
> error[E0658]: use of unstable library feature 'vec_try_remove'
>    --> library/alloc/tests/vec.rs:527:20
>     |
> 527 |     assert_eq!(vec.try_remove(0), Some(1));
>     |                    ^^^^^^^^^^
>     |
>     = note: see issue #77481 <https://github.com/rust-lang/rust/issues/77481> for more information
>     = help: add `#![feature(vec_try_remove)]` to the crate attributes to enable
> 