plain
    Checking addr2line v0.16.0
error[E0658]: use of unstable library feature 'autoref'
    --> /checkout/library/core/src/macros/mod.rs:590:15
     |
575  | / macro_rules! writeln {
576  | |     ($dst:expr $(,)?) => {
577  | |         $crate::write!($dst, "\n")
...    |
...    |
590  | |         match $crate::autoref_mut!($dst) {
...    |
597  | |     };
598  | | }
     | |_- in this expansion of `writeln!`
     | |_- in this expansion of `writeln!`
     |
    ::: library/std/src/process.rs:2205:25
     |
2205 |                   let _ = writeln!(io::stderr(), "Error: {err:?}");
     |
     = help: add `#![feature(autoref)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'autoref'
---
     |                   ----------- in this macro invocation (#1)
     |
    ::: /checkout/library/core/src/macros/mod.rs:499:1
     |
499  | / macro_rules! write {
500  | |     // Retrocompat hack to support two phased borrows:
501  | |     // when `$dst` is made of chaining `.field` accesses to a local,
502  | |     // it is guaranteed to be a *place* and thus there is no question of
...    |
518  | |         match $crate::autoref_mut!($dst) {
...    |
524  | |     };
525  | | }
     | |_- in this expansion of `$crate::write!` (#2)
     | |_- in this expansion of `$crate::write!` (#2)
...
575  | / macro_rules! writeln {
576  | |     ($dst:expr $(,)?) => {
577  | |         $crate::write!($dst, "\n")
578  | |     };
...    |
597  | |     };
598  | | }
598  | | }
     | |_- in this expansion of `writeln!` (#1)
    --> /checkout/library/core/src/lib.rs:270:57
     |
267  | /     macro_rules! autoref_mut {
268  | |         ($x:expr) => {{
269  | |             use $crate::autoref::AutoRef as _;
270  | |             $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::autoref::UnstableMethodSeal }>()
271  | |         }};
272  | |     }
     | |_____- in this expansion of `$crate::autoref_mut!` (#3)
     |
---
     |                   ----------- in this macro invocation (#1)
     |
    ::: /checkout/library/core/src/macros/mod.rs:499:1
     |
499  | / macro_rules! write {
500  | |     // Retrocompat hack to support two phased borrows:
501  | |     // when `$dst` is made of chaining `.field` accesses to a local,
502  | |     // it is guaranteed to be a *place* and thus there is no question of
...    |
518  | |         match $crate::autoref_mut!($dst) {
...    |
524  | |     };
525  | | }
     | |_- in this expansion of `$crate::write!` (#2)
     | |_- in this expansion of `$crate::write!` (#2)
...
575  | / macro_rules! writeln {
576  | |     ($dst:expr $(,)?) => {
577  | |         $crate::write!($dst, "\n")
578  | |     };
...    |
597  | |     };
598  | | }
598  | | }
     | |_- in this expansion of `writeln!` (#1)
    --> /checkout/library/core/src/lib.rs:270:57
     |
267  | /     macro_rules! autoref_mut {
268  | |         ($x:expr) => {{
269  | |             use $crate::autoref::AutoRef as _;
270  | |             $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::autoref::UnstableMethodSeal }>()
271  | |         }};
272  | |     }
     | |_____- in this expansion of `$crate::autoref_mut!` (#3)
     |
     |
     = help: add `#![feature(autoref)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'autoref'
     |
    ::: library/std/src/process.rs:2205:25
     |
2205 |                   let _ = writeln!(io::stderr(), "Error: {err:?}");
     |
    ::: /checkout/library/core/src/macros/mod.rs:575:1
     |
575  | / macro_rules! writeln {
575  | / macro_rules! writeln {
576  | |     ($dst:expr $(,)?) => {
577  | |         $crate::write!($dst, "\n")
...    |
...    |
590  | |         match $crate::autoref_mut!($dst) {
...    |
597  | |     };
598  | | }
     | |_- in this expansion of `writeln!` (#1)
     | |_- in this expansion of `writeln!` (#1)
    --> /checkout/library/core/src/lib.rs:269:17
     |
267  | /     macro_rules! autoref_mut {
268  | |         ($x:expr) => {{
269  | |             use $crate::autoref::AutoRef as _;
     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^
270  | |             $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::autoref::UnstableMethodSeal }>()
271  | |         }};
     | |_____- in this expansion of `$crate::autoref_mut!` (#2)
     |
     = help: add `#![feature(autoref)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'autoref'
     |
    ::: library/std/src/process.rs:2205:25
     |
2205 |                   let _ = writeln!(io::stderr(), "Error: {err:?}");
     |
    ::: /checkout/library/core/src/macros/mod.rs:575:1
     |
575  | / macro_rules! writeln {
575  | / macro_rules! writeln {
576  | |     ($dst:expr $(,)?) => {
577  | |         $crate::write!($dst, "\n")
...    |
...    |
590  | |         match $crate::autoref_mut!($dst) {
...    |
597  | |     };
598  | | }
     | |_- in this expansion of `writeln!` (#1)
     | |_- in this expansion of `writeln!` (#1)
    --> /checkout/library/core/src/lib.rs:270:57
     |
267  | /     macro_rules! autoref_mut {
268  | |         ($x:expr) => {{
269  | |             use $crate::autoref::AutoRef as _;
270  | |             $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::autoref::UnstableMethodSeal }>()
271  | |         }};
272  | |     }
     | |_____- in this expansion of `$crate::autoref_mut!` (#2)
     |
---
     |                   ----------- in this macro invocation (#1)
     |
    ::: /checkout/library/core/src/macros/mod.rs:499:1
     |
499  | / macro_rules! write {
500  | |     // Retrocompat hack to support two phased borrows:
501  | |     // when `$dst` is made of chaining `.field` accesses to a local,
502  | |     // it is guaranteed to be a *place* and thus there is no question of
...    |
518  | |         match $crate::autoref_mut!($dst) {
...    |
524  | |     };
525  | | }
     | |_- in this expansion of `$crate::write!` (#2)
     | |_- in this expansion of `$crate::write!` (#2)
...
575  | / macro_rules! writeln {
576  | |     ($dst:expr $(,)?) => {
577  | |         $crate::write!($dst, "\n")
578  | |     };
...    |
597  | |     };
598  | | }
598  | | }
     | |_- in this expansion of `writeln!` (#1)
    --> /checkout/library/core/src/lib.rs:270:16
     |
267  | /     macro_rules! autoref_mut {
268  | |         ($x:expr) => {{
269  | |             use $crate::autoref::AutoRef as _;
270  | |             $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::autoref::UnstableMethodSeal }>()
271  | |         }};
272  | |     }
     | |_____- in this expansion of `$crate::autoref_mut!` (#3)
     |
---
     |                   ----------- in this macro invocation (#1)
     |
    ::: /checkout/library/core/src/macros/mod.rs:499:1
     |
499  | / macro_rules! write {
500  | |     // Retrocompat hack to support two phased borrows:
501  | |     // when `$dst` is made of chaining `.field` accesses to a local,
502  | |     // it is guaranteed to be a *place* and thus there is no question of
...    |
518  | |         match $crate::autoref_mut!($dst) {
...    |
524  | |     };
525  | | }
     | |_- in this expansion of `$crate::write!` (#2)
     | |_- in this expansion of `$crate::write!` (#2)
...
575  | / macro_rules! writeln {
576  | |     ($dst:expr $(,)?) => {
577  | |         $crate::write!($dst, "\n")
578  | |     };
...    |
597  | |     };
598  | | }
598  | | }
     | |_- in this expansion of `writeln!` (#1)
    --> /checkout/library/core/src/lib.rs:270:16
     |
267  | /     macro_rules! autoref_mut {
268  | |         ($x:expr) => {{
269  | |             use $crate::autoref::AutoRef as _;
270  | |             $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::autoref::UnstableMethodSeal }>()
271  | |         }};
272  | |     }
     | |_____- in this expansion of `$crate::autoref_mut!` (#3)
     |
     |
     = help: add `#![feature(autoref)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'autoref'
     |
    ::: library/std/src/process.rs:2205:25
     |
2205 |                   let _ = writeln!(io::stderr(), "Error: {err:?}");
     |
    ::: /checkout/library/core/src/macros/mod.rs:575:1
     |
575  | / macro_rules! writeln {
575  | / macro_rules! writeln {
576  | |     ($dst:expr $(,)?) => {
577  | |         $crate::write!($dst, "\n")
...    |
...    |
590  | |         match $crate::autoref_mut!($dst) {
...    |
597  | |     };
598  | | }
     | |_- in this expansion of `writeln!` (#1)
     | |_- in this expansion of `writeln!` (#1)
    --> /checkout/library/core/src/lib.rs:270:16
     |
267  | /     macro_rules! autoref_mut {
268  | |         ($x:expr) => {{
269  | |             use $crate::autoref::AutoRef as _;
270  | |             $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::autoref::UnstableMethodSeal }>()
271  | |         }};
272  | |     }
     | |_____- in this expansion of `$crate::autoref_mut!` (#2)
     |
