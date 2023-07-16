
[00:02:28] error: use of unstable library feature 'try_from' (see issue #33417)
[00:02:28]   --> /checkout/src/liballoc/string.rs:60:5
[00:02:28]    |
[00:02:28] 60 | use core::convert::TryFrom;
[00:02:28]    |     ^^^^^^^^^^^^^^^^^^^^^^
[00:02:28]    |
[00:02:28]    = help: add #![feature(try_from)] to the crate attributes to enable
[00:02:28] 
[00:02:28] error: use of unstable library feature 'try_from' (see issue #33417)
[00:02:28]     --> /checkout/src/liballoc/string.rs:1607:5
[00:02:28]      |
[00:02:28] 1607 |     type Error = FromUtf8Error;
[00:02:28]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:02:28]      |
[00:02:28]      = help: add #![feature(try_from)] to the crate attributes to enable
