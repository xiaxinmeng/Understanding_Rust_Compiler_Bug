
[00:01:59] error[E0549]: rustc_deprecated attribute must be paired with either stable or unstable attribute
[00:01:59]    --> /checkout/src/liballoc/lib.rs:214:1
[00:01:59]     |
[00:01:59] 214 | pub use core::ops::Bound;
[00:01:59]     | ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:01:59] 
[00:01:59] error: use of unstable library feature 'range_argument' (see issue #30877)
[00:01:59]   --> /checkout/src/liballoc/btree/map.rs:16:24
[00:01:59]    |
[00:01:59] 16 | use core::ops::{Index, RangeBounds};
[00:01:59]    |                        ^^^^^^^^^^^
[00:01:59]    |
[00:01:59]    = help: add #![feature(range_argument)] to the crate attributes to enable
