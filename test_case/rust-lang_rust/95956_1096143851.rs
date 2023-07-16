plain
   Compiling rustc-std-workspace-std v1.99.0 (/checkout/library/rustc-std-workspace-std)
   Compiling proc_macro v0.0.0 (/checkout/library/proc_macro)
   Compiling unicode-width v0.1.8
   Compiling getopts v0.2.21
error[E0658]: use of unstable library feature 'rt': this public module should not exist and is highly likely to disappear
    |
17  | /  pub macro panic_2015 {
18  | |      () => ({
19  | |          $crate::rt::begin_panic("explicit panic")
---
598 |        assert!(
    |  ______-
    | |______|
    | |
599 | |          len == 1 || len == 0,
600 | |          "the short_name (first argument) should be a single character, \
601 | |           or an empty string for none"
    | |      -
    | |______|
    | |______in this macro invocation (#1)
    |        in this macro invocation (#2)
    |        in this macro invocation (#2)
    |
    = help: add `#![feature(rt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rt': this public module should not exist and is highly likely to disappear
    |
17  | /  pub macro panic_2015 {
18  | |      () => ({
19  | |          $crate::rt::begin_panic("explicit panic")
---
604 |        assert!(
    |  ______-
    | |______|
    | |
605 | |          len == 0 || len > 1,
606 | |          "the long_name (second argument) should be longer than a single \
607 | |           character, or an empty string for none"
    | |      -
    | |______|
    | |______in this macro invocation (#1)
    |        in this macro invocation (#2)
    |        in this macro invocation (#2)
    |
    = help: add `#![feature(rt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rt': this public module should not exist and is highly likely to disappear
    |
17  | /  pub macro panic_2015 {
18  | |      () => ({
19  | |          $crate::rt::begin_panic("explicit panic")
---
    | |__- in this expansion of `$crate::panic::panic_2015!` (#2)
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.21/src/lib.rs:553:22
    |
553 |                    _ => panic!("the short name should only be 1 ascii char long"),
    |                         |
    |                         in this macro invocation (#1)
    |                         in this macro invocation (#2)
    |
---
    |  |_- in this expansion of `panic!` (#1)
    |
    = help: add `#![feature(rt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rt': this public module should not exist and is highly likely to disappear
    |
17  | /  pub macro panic_2015 {
18  | |      () => ({
19  | |          $crate::rt::begin_panic("explicit panic")
---
    | |__- in this expansion of `$crate::panic::panic_2015!` (#2)
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.21/src/lib.rs:766:23
    |
766 |                (0, 0) => panic!("this long-format option was given no name"),
    |                          |
    |                          in this macro invocation (#1)
    |                          in this macro invocation (#2)
    |
---
    |  |_- in this expansion of `panic!` (#1)
    |
    = help: add `#![feature(rt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rt': this public module should not exist and is highly likely to disappear
    |
17  | /  pub macro panic_2015 {
18  | |      () => ({
19  | |          $crate::rt::begin_panic("explicit panic")
---
    | |__- in this expansion of `$crate::panic::panic_2015!` (#2)
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.21/src/lib.rs:790:23
    |
790 |                (_, _) => panic!("something is wrong with the long-form opt"),
    |                          |
    |                          in this macro invocation (#1)
    |                          in this macro invocation (#2)
    |
---
    |  |_- in this expansion of `panic!` (#1)
    |
    = help: add `#![feature(rt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rt': this public module should not exist and is highly likely to disappear
    |
17  | /  pub macro panic_2015 {
18  | |      () => ({
19  | |          $crate::rt::begin_panic("explicit panic")
---
    | |__- in this expansion of `$crate::panic::panic_2015!` (#2)
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.21/src/lib.rs:799:21
    |
799 |                None => panic!("No option '{}' defined", nm),
    |                        |
    |                        in this macro invocation (#1)
    |                        in this macro invocation (#2)
    |
