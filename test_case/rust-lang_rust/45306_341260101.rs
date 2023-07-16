
[00:02:34] error: use of unstable library feature 'from_ref' (see issue #45703)
[00:02:34]    --> /checkout/src/liballoc/slice.rs:123:23
[00:02:34]     |
[00:02:34] 123 | pub use core::slice::{from_ref, from_ref_mut};
[00:02:34]     |                       ^^^^^^^^
[00:02:34]     |
[00:02:34]     = help: add #![feature(from_ref)] to the crate attributes to enable
[00:02:34] 
[00:02:34] error: use of unstable library feature 'from_ref' (see issue #45703)
[00:02:34]    --> /checkout/src/liballoc/slice.rs:123:33
[00:02:34]     |
[00:02:34] 123 | pub use core::slice::{from_ref, from_ref_mut};
[00:02:34]     |                                 ^^^^^^^^^^^^
[00:02:34]     |
[00:02:34]     = help: add #![feature(from_ref)] to the crate attributes to enable
