plain
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 146 tests
FFFFFFF.................F..........F.F.F....F.FF.FFF..F.F........F.....FFF...F..F...F...F.....F.F.F. 100/146
FF.F..F.FFF...F...FF..F..FFFFFFFF.F.F.F.......

---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(std[3765])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(std[3765]): {"PathBuf": DefId(1:7009 ~ std[3765]::path::PathBuf), "OsString": DefId(1:2781 ~ std[3765]::ffi::os_str::OsString), "FileType": DefId(1:10508 ~ std[3765]::fs::FileType), "std_panic_2015_macro": DefId(1:6762 ~ std[3765]::panic::panic_2015), "IoRead": DefId(1:4337 ~ std[3765]::io::Read), "HashMapEntry": DefId(1:1397 ~ std[3765]::collections::hash::map::Entry), "Path": DefId(1:7116 ~ std[3765]::path::Path), "File": DefId(1:3030 ~ std[3765]::fs::File), "OsStr": DefId(1:2784 ~ std[3765]::ffi::os_str::OsStr), "DirBuilder": DefId(1:10526 ~ std[3765]::fs::DirBuilder), "hashset_type": DefId(1:1807 ~ std[3765]::collections::hash::set::HashSet), "CStr": DefId(1:10319 ~ std[3765]::ffi::c_str::CStr), "mutex_type": DefId(1:8401 ~ std[3765]::sync::mutex::Mutex), "cstring_type": DefId(1:10301 ~ std[3765]::ffi::c_str::CString), "Receiver": DefId(1:8127 ~ std[3765]::sync::mpsc::Receiver), "IoWrite": DefId(1:4398 ~ std[3765]::io::Write), "std_panic_macro": DefId(1:9 ~ std[3765]::macros::panic), "hashmap_type": DefId(1:1090 ~ std[3765]::collections::hash::map::HashMap)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [mir_const] processing MIR for `point::<impl at /checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs:26:5: 38:6>::distance_from_origin`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(std[3765])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(std[3765]): {"PathBuf": DefId(1:7009 ~ std[3765]::path::PathBuf), "OsString": DefId(1:2781 ~ std[3765]::ffi::os_str::OsString), "FileType": DefId(1:10508 ~ std[3765]::fs::FileType), "std_panic_2015_macro": DefId(1:6762 ~ std[3765]::panic::panic_2015), "IoRead": DefId(1:4337 ~ std[3765]::io::Read), "HashMapEntry": DefId(1:1397 ~ std[3765]::collections::hash::map::Entry), "Path": DefId(1:7116 ~ std[3765]::path::Path), "File": DefId(1:3030 ~ std[3765]::fs::File), "OsStr": DefId(1:2784 ~ std[3765]::ffi::os_str::OsStr), "DirBuilder": DefId(1:10526 ~ std[3765]::fs::DirBuilder), "hashset_type": DefId(1:1807 ~ std[3765]::collections::hash::set::HashSet), "CStr": DefId(1:10319 ~ std[3765]::ffi::c_str::CStr), "mutex_type": DefId(1:8401 ~ std[3765]::sync::mutex::Mutex), "cstring_type": DefId(1:10301 ~ std[3765]::ffi::c_str::CString), "Receiver": DefId(1:8127 ~ std[3765]::sync::mpsc::Receiver), "IoWrite": DefId(1:4398 ~ std[3765]::io::Write), "std_panic_macro": DefId(1:9 ~ std[3765]::macros::panic), "hashmap_type": DefId(1:1090 ~ std[3765]::collections::hash::map::HashMap)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [all_diagnostic_items] calculating the diagnostic items map
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(core[54ad])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(core[54ad]): {"pointer_trait": DefId(2:9322 ~ core[54ad]::fmt::Pointer), "debug_assert_macro": DefId(2:8 ~ core[54ad]::macros::debug_assert), "Hash": DefId(2:9511 ~ core[54ad]::hash::Hash), "mem_uninitialized": DefId(2:1972 ~ core[54ad]::mem::uninitialized), "DoubleEndedIterator": DefId(2:7425 ~ core[54ad]::iter::traits::double_ended::DoubleEndedIterator), "mem_discriminant": DefId(2:2006 ~ core[54ad]::mem::discriminant), "option_type": DefId(2:43195 ~ core[54ad]::option::Option), "atomic_mod": DefId(2:8828 ~ core[54ad]::sync::atomic), "IntoIterator": DefId(2:7381 ~ core[54ad]::iter::traits::collect::IntoIterator), "pointer_trait_fmt": DefId(2:9323 ~ core[54ad]::fmt::Pointer::fmt), "FromIterator": DefId(2:7377 ~ core[54ad]::iter::traits::collect::FromIterator), "mem_zeroed": DefId(2:1970 ~ core[54ad]::mem::zeroed), "Duration": DefId(2:45728 ~ core[54ad]::time::Duration), "mem_drop": DefId(2:1980 ~ core[54ad]::mem::drop), "fence": DefId(2:8933 ~ core[54ad]::sync::atomic::fence), "Copy": DefId(2:2999 ~ core[54ad]::marker::Copy), "core_panic_2021_macro": DefId(2:8363 ~ core[54ad]::panic::panic_2021), "PartialEq": DefId(2:2640 ~ core[54ad]::cmp::PartialEq), "Ord": DefId(2:2676 ~ core[54ad]::cmp::Ord), "deref_target": DefId(2:3201 ~ core[54ad]::ops::deref::Deref::Target), "send_trait": DefId(2:2989 ~ core[54ad]::marker::Send), "ref_unwind_safe_trait": DefId(2:8281 ~ core[54ad]::panic::unwind_safe::RefUnwindSafe), "noop_method_borrow": DefId(2:2596 ~ core[54ad]::borrow::{impl#0}::borrow), "try_from_trait": DefId(2:2902 ~ core[54ad]::convert::TryFrom), "mem_forget": DefId(2:1948 ~ core[54ad]::mem::forget), "ptr_null": DefId(2:2502 ~ core[54ad]::ptr::null), "PartialOrd": DefId(2:2687 ~ core[54ad]::cmp::PartialOrd), "Ordering": DefId(2:43399 ~ core[54ad]::sync::atomic::Ordering), "from_trait": DefId(2:2895 ~ core[54ad]::convert::From), "assume_init": DefId(2:1914 ~ core[54ad]::mem::maybe_uninit::{impl#2}::assume_init), "noop_method_clone": DefId(2:2635 ~ core[54ad]::clone::impls::{impl#3}::clone), "debug_trait": DefId(2:9305 ~ core[54ad]::fmt::Debug), "mem_replace": DefId(2:1978 ~ core[54ad]::mem::replace), "ptr_null_mut": DefId(2:2504 ~ core[54ad]::ptr::null_mut), "Default": DefId(2:2967 ~ core[54ad]::default::Default), "compiler_fence": DefId(2:8934 ~ core[54ad]::sync::atomic::compiler_fence), "cmp_max": DefId(2:2705 ~ core[54ad]::cmp::max), "needs_drop": DefId(2:1968 ~ core[54ad]::mem::needs_drop), "gen_future": DefId(2:12725 ~ core[54ad]::future::from_generator::GenFuture), "AsMut": DefId(2:2889 ~ core[54ad]::convert::AsMut), "noop_method_deref": DefId(2:3206 ~ core[54ad]::ops::deref::{impl#0}::deref), "Eq": DefId(2:2645 ~ core[54ad]::cmp::Eq), "try_into_trait": DefId(2:2898 ~ core[54ad]::convert::TryInto), "mem_size_of": DefId(2:1952 ~ core[54ad]::mem::size_of), "Deref": DefId(2:3200 ~ core[54ad]::ops::deref::Deref), "maybe_uninit_uninit": DefId(2:1905 ~ core[54ad]::mem::maybe_uninit::{impl#2}::uninit), "iter_repeat": DefId(2:7216 ~ core[54ad]::iter::sources::repeat::repeat), "cmp_min": DefId(2:2695 ~ core[54ad]::cmp::min), "unwind_safe_trait": DefId(2:8280 ~ core[54ad]::panic::unwind_safe::UnwindSafe), "core_panic_2015_macro": DefId(2:8362 ~ core[54ad]::panic::panic_2015), "into_trait": DefId(2:2892 ~ core[54ad]::convert::Into), "result_type": DefId(2:43321 ~ core[54ad]::result::Result), "AsRef": DefId(2:2886 ~ core[54ad]::convert::AsRef), "sync_trait": DefId(2:3001 ~ core[54ad]::marker::Sync), "Any": DefId(2:3650 ~ core[54ad]::any::Any), "mem_size_of_val": DefId(2:1954 ~ core[54ad]::mem::size_of_val), "transmute": DefId(2:1659 ~ core[54ad]::intrinsics::#1::transmute), "core_panic_macro": DefId(2:4 ~ core[54ad]::macros::panic), "maybe_uninit_zeroed": DefId(2:1910 ~ core[54ad]::mem::maybe_uninit::{impl#2}::zeroed), "Clone": DefId(2:2610 ~ core[54ad]::clone::Clone), "Borrow": DefId(2:2588 ~ core[54ad]::borrow::Borrow), "display_trait": DefId(2:9312 ~ core[54ad]::fmt::Display), "deref_method": DefId(2:3202 ~ core[54ad]::ops::deref::Deref::deref), "Iterator": DefId(2:7561 ~ core[54ad]::iter::traits::iterator::Iterator), "assert_macro": DefId(2:37 ~ core[54ad]::macros::builtin::assert)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [mir_const] processing MIR for `point::<impl at /checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs:27:5: 48:6>::distance_from_point`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(core[54ad])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(core[54ad]): {"pointer_trait": DefId(2:9322 ~ core[54ad]::fmt::Pointer), "debug_assert_macro": DefId(2:8 ~ core[54ad]::macros::debug_assert), "Hash": DefId(2:9511 ~ core[54ad]::hash::Hash), "mem_uninitialized": DefId(2:1972 ~ core[54ad]::mem::uninitialized), "DoubleEndedIterator": DefId(2:7425 ~ core[54ad]::iter::traits::double_ended::DoubleEndedIterator), "mem_discriminant": DefId(2:2006 ~ core[54ad]::mem::discriminant), "option_type": DefId(2:43195 ~ core[54ad]::option::Option), "atomic_mod": DefId(2:8828 ~ core[54ad]::sync::atomic), "core_panic_2021_macro": DefId(2:8363 ~ core[54ad]::panic::panic_2021), "IntoIterator": DefId(2:7381 ~ core[54ad]::iter::traits::collect::IntoIterator), "pointer_trait_fmt": DefId(2:9323 ~ core[54ad]::fmt::Pointer::fmt), "FromIterator": DefId(2:7377 ~ core[54ad]::iter::traits::collect::FromIterator), "mem_zeroed": DefId(2:1970 ~ core[54ad]::mem::zeroed), "Duration": DefId(2:45728 ~ core[54ad]::time::Duration), "mem_drop": DefId(2:1980 ~ core[54ad]::mem::drop), "fence": DefId(2:8933 ~ core[54ad]::sync::atomic::fence), "Copy": DefId(2:2999 ~ core[54ad]::marker::Copy), "PartialEq": DefId(2:2640 ~ core[54ad]::cmp::PartialEq), "Ord": DefId(2:2676 ~ core[54ad]::cmp::Ord), "deref_target": DefId(2:3201 ~ core[54ad]::ops::deref::Deref::Target), "send_trait": DefId(2:2989 ~ core[54ad]::marker::Send), "ref_unwind_safe_trait": DefId(2:8281 ~ core[54ad]::panic::unwind_safe::RefUnwindSafe), "noop_method_borrow": DefId(2:2596 ~ core[54ad]::borrow::{impl#0}::borrow), "try_from_trait": DefId(2:2902 ~ core[54ad]::convert::TryFrom), "mem_forget": DefId(2:1948 ~ core[54ad]::mem::forget), "ptr_null": DefId(2:2502 ~ core[54ad]::ptr::null), "PartialOrd": DefId(2:2687 ~ core[54ad]::cmp::PartialOrd), "Ordering": DefId(2:43399 ~ core[54ad]::sync::atomic::Ordering), "from_trait": DefId(2:2895 ~ core[54ad]::convert::From), "assume_init": DefId(2:1914 ~ core[54ad]::mem::maybe_uninit::{impl#2}::assume_init), "noop_method_clone": DefId(2:2635 ~ core[54ad]::clone::impls::{impl#3}::clone), "debug_trait": DefId(2:9305 ~ core[54ad]::fmt::Debug), "mem_replace": DefId(2:1978 ~ core[54ad]::mem::replace), "ptr_null_mut": DefId(2:2504 ~ core[54ad]::ptr::null_mut), "Default": DefId(2:2967 ~ core[54ad]::default::Default), "compiler_fence": DefId(2:8934 ~ core[54ad]::sync::atomic::compiler_fence), "cmp_max": DefId(2:2705 ~ core[54ad]::cmp::max), "needs_drop": DefId(2:1968 ~ core[54ad]::mem::needs_drop), "gen_future": DefId(2:12725 ~ core[54ad]::future::from_generator::GenFuture), "AsMut": DefId(2:2889 ~ core[54ad]::convert::AsMut), "noop_method_deref": DefId(2:3206 ~ core[54ad]::ops::deref::{impl#0}::deref), "Eq": DefId(2:2645 ~ core[54ad]::cmp::Eq), "try_into_trait": DefId(2:2898 ~ core[54ad]::convert::TryInto), "mem_size_of": DefId(2:1952 ~ core[54ad]::mem::size_of), "Deref": DefId(2:3200 ~ core[54ad]::ops::deref::Deref), "maybe_uninit_uninit": DefId(2:1905 ~ core[54ad]::mem::maybe_uninit::{impl#2}::uninit), "iter_repeat": DefId(2:7216 ~ core[54ad]::iter::sources::repeat::repeat), "cmp_min": DefId(2:2695 ~ core[54ad]::cmp::min), "unwind_safe_trait": DefId(2:8280 ~ core[54ad]::panic::unwind_safe::UnwindSafe), "core_panic_2015_macro": DefId(2:8362 ~ core[54ad]::panic::panic_2015), "into_trait": DefId(2:2892 ~ core[54ad]::convert::Into), "result_type": DefId(2:43321 ~ core[54ad]::result::Result), "AsRef": DefId(2:2886 ~ core[54ad]::convert::AsRef), "sync_trait": DefId(2:3001 ~ core[54ad]::marker::Sync), "Any": DefId(2:3650 ~ core[54ad]::any::Any), "mem_size_of_val": DefId(2:1954 ~ core[54ad]::mem::size_of_val), "transmute": DefId(2:1659 ~ core[54ad]::intrinsics::#1::transmute), "core_panic_macro": DefId(2:4 ~ core[54ad]::macros::panic), "maybe_uninit_zeroed": DefId(2:1910 ~ core[54ad]::mem::maybe_uninit::{impl#2}::zeroed), "Clone": DefId(2:2610 ~ core[54ad]::clone::Clone), "Borrow": DefId(2:2588 ~ core[54ad]::borrow::Borrow), "display_trait": DefId(2:9312 ~ core[54ad]::fmt::Display), "deref_method": DefId(2:3202 ~ core[54ad]::ops::deref::Deref::deref), "Iterator": DefId(2:7561 ~ core[54ad]::iter::traits::iterator::Iterator), "assert_macro": DefId(2:37 ~ core[54ad]::macros::builtin::assert)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [all_diagnostic_items] calculating the diagnostic items map
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/change_add_field/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(core[54ad])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(core[54ad]): {"pointer_trait": DefId(2:9322 ~ core[54ad]::fmt::Pointer), "debug_assert_macro": DefId(2:8 ~ core[54ad]::macros::debug_assert), "Hash": DefId(2:9511 ~ core[54ad]::hash::Hash), "mem_uninitialized": DefId(2:1972 ~ core[54ad]::mem::uninitialized), "DoubleEndedIterator": DefId(2:7425 ~ core[54ad]::iter::traits::double_ended::DoubleEndedIterator), "mem_discriminant": DefId(2:2006 ~ core[54ad]::mem::discriminant), "core_panic_2021_macro": DefId(2:8363 ~ core[54ad]::panic::panic_2021), "option_type": DefId(2:43195 ~ core[54ad]::option::Option), "atomic_mod": DefId(2:8828 ~ core[54ad]::sync::atomic), "IntoIterator": DefId(2:7381 ~ core[54ad]::iter::traits::collect::IntoIterator), "pointer_trait_fmt": DefId(2:9323 ~ core[54ad]::fmt::Pointer::fmt), "FromIterator": DefId(2:7377 ~ core[54ad]::iter::traits::collect::FromIterator), "mem_zeroed": DefId(2:1970 ~ core[54ad]::mem::zeroed), "Duration": DefId(2:45728 ~ core[54ad]::time::Duration), "mem_drop": DefId(2:1980 ~ core[54ad]::mem::drop), "fence": DefId(2:8933 ~ core[54ad]::sync::atomic::fence), "Copy": DefId(2:2999 ~ core[54ad]::marker::Copy), "PartialEq": DefId(2:2640 ~ core[54ad]::cmp::PartialEq), "Ord": DefId(2:2676 ~ core[54ad]::cmp::Ord), "deref_target": DefId(2:3201 ~ core[54ad]::ops::deref::Deref::Target), "send_trait": DefId(2:2989 ~ core[54ad]::marker::Send), "ref_unwind_safe_trait": DefId(2:8281 ~ core[54ad]::panic::unwind_safe::RefUnwindSafe), "noop_method_borrow": DefId(2:2596 ~ core[54ad]::borrow::{impl#0}::borrow), "try_from_trait": DefId(2:2902 ~ core[54ad]::convert::TryFrom), "mem_forget": DefId(2:1948 ~ core[54ad]::mem::forget), "ptr_null": DefId(2:2502 ~ core[54ad]::ptr::null), "PartialOrd": DefId(2:2687 ~ core[54ad]::cmp::PartialOrd), "Ordering": DefId(2:43399 ~ core[54ad]::sync::atomic::Ordering), "from_trait": DefId(2:2895 ~ core[54ad]::convert::From), "assume_init": DefId(2:1914 ~ core[54ad]::mem::maybe_uninit::{impl#2}::assume_init), "noop_method_clone": DefId(2:2635 ~ core[54ad]::clone::impls::{impl#3}::clone), "debug_trait": DefId(2:9305 ~ core[54ad]::fmt::Debug), "mem_replace": DefId(2:1978 ~ core[54ad]::mem::replace), "ptr_null_mut": DefId(2:2504 ~ core[54ad]::ptr::null_mut), "Default": DefId(2:2967 ~ core[54ad]::default::Default), "compiler_fence": DefId(2:8934 ~ core[54ad]::sync::atomic::compiler_fence), "cmp_max": DefId(2:2705 ~ core[54ad]::cmp::max), "needs_drop": DefId(2:1968 ~ core[54ad]::mem::needs_drop), "gen_future": DefId(2:12725 ~ core[54ad]::future::from_generator::GenFuture), "AsMut": DefId(2:2889 ~ core[54ad]::convert::AsMut), "noop_method_deref": DefId(2:3206 ~ core[54ad]::ops::deref::{impl#0}::deref), "Eq": DefId(2:2645 ~ core[54ad]::cmp::Eq), "try_into_trait": DefId(2:2898 ~ core[54ad]::convert::TryInto), "mem_size_of": DefId(2:1952 ~ core[54ad]::mem::size_of), "Deref": DefId(2:3200 ~ core[54ad]::ops::deref::Deref), "maybe_uninit_uninit": DefId(2:1905 ~ core[54ad]::mem::maybe_uninit::{impl#2}::uninit), "iter_repeat": DefId(2:7216 ~ core[54ad]::iter::sources::repeat::repeat), "cmp_min": DefId(2:2695 ~ core[54ad]::cmp::min), "unwind_safe_trait": DefId(2:8280 ~ core[54ad]::panic::unwind_safe::UnwindSafe), "core_panic_2015_macro": DefId(2:8362 ~ core[54ad]::panic::panic_2015), "into_trait": DefId(2:2892 ~ core[54ad]::convert::Into), "result_type": DefId(2:43321 ~ core[54ad]::result::Result), "AsRef": DefId(2:2886 ~ core[54ad]::convert::AsRef), "sync_trait": DefId(2:3001 ~ core[54ad]::marker::Sync), "Any": DefId(2:3650 ~ core[54ad]::any::Any), "mem_size_of_val": DefId(2:1954 ~ core[54ad]::mem::size_of_val), "transmute": DefId(2:1659 ~ core[54ad]::intrinsics::#1::transmute), "core_panic_macro": DefId(2:4 ~ core[54ad]::macros::panic), "maybe_uninit_zeroed": DefId(2:1910 ~ core[54ad]::mem::maybe_uninit::{impl#2}::zeroed), "Clone": DefId(2:2610 ~ core[54ad]::clone::Clone), "Borrow": DefId(2:2588 ~ core[54ad]::borrow::Borrow), "display_trait": DefId(2:9312 ~ core[54ad]::fmt::Display), "deref_method": DefId(2:3202 ~ core[54ad]::ops::deref::Deref::deref), "Iterator": DefId(2:7561 ~ core[54ad]::iter::traits::iterator::Iterator), "assert_macro": DefId(2:37 ~ core[54ad]::macros::builtin::assert)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [mir_const] processing MIR for `fn_with_type_in_sig::boop`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----

error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(std[3765])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(std[3765]): {"hashmap_type": DefId(1:1090 ~ std[3765]::collections::hash::map::HashMap), "PathBuf": DefId(1:7009 ~ std[3765]::path::PathBuf), "OsString": DefId(1:2781 ~ std[3765]::ffi::os_str::OsString), "FileType": DefId(1:10508 ~ std[3765]::fs::FileType), "std_panic_2015_macro": DefId(1:6762 ~ std[3765]::panic::panic_2015), "IoRead": DefId(1:4337 ~ std[3765]::io::Read), "HashMapEntry": DefId(1:1397 ~ std[3765]::collections::hash::map::Entry), "Path": DefId(1:7116 ~ std[3765]::path::Path), "File": DefId(1:3030 ~ std[3765]::fs::File), "OsStr": DefId(1:2784 ~ std[3765]::ffi::os_str::OsStr), "DirBuilder": DefId(1:10526 ~ std[3765]::fs::DirBuilder), "hashset_type": DefId(1:1807 ~ std[3765]::collections::hash::set::HashSet), "CStr": DefId(1:10319 ~ std[3765]::ffi::c_str::CStr), "cstring_type": DefId(1:10301 ~ std[3765]::ffi::c_str::CString), "mutex_type": DefId(1:8401 ~ std[3765]::sync::mutex::Mutex), "Receiver": DefId(1:8127 ~ std[3765]::sync::mpsc::Receiver), "IoWrite": DefId(1:4398 ~ std[3765]::io::Write), "std_panic_macro": DefId(1:9 ~ std[3765]::macros::panic)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [all_diagnostic_items] calculating the diagnostic items map
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----

error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(std[3765])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(std[3765]): {"PathBuf": DefId(1:7009 ~ std[3765]::path::PathBuf), "OsString": DefId(1:2781 ~ std[3765]::ffi::os_str::OsString), "FileType": DefId(1:10508 ~ std[3765]::fs::FileType), "std_panic_2015_macro": DefId(1:6762 ~ std[3765]::panic::panic_2015), "IoRead": DefId(1:4337 ~ std[3765]::io::Read), "HashMapEntry": DefId(1:1397 ~ std[3765]::collections::hash::map::Entry), "Path": DefId(1:7116 ~ std[3765]::path::Path), "File": DefId(1:3030 ~ std[3765]::fs::File), "OsStr": DefId(1:2784 ~ std[3765]::ffi::os_str::OsStr), "DirBuilder": DefId(1:10526 ~ std[3765]::fs::DirBuilder), "hashset_type": DefId(1:1807 ~ std[3765]::collections::hash::set::HashSet), "CStr": DefId(1:10319 ~ std[3765]::ffi::c_str::CStr), "mutex_type": DefId(1:8401 ~ std[3765]::sync::mutex::Mutex), "cstring_type": DefId(1:10301 ~ std[3765]::ffi::c_str::CString), "Receiver": DefId(1:8127 ~ std[3765]::sync::mpsc::Receiver), "IoWrite": DefId(1:4398 ~ std[3765]::io::Write), "std_panic_macro": DefId(1:9 ~ std[3765]::macros::panic), "hashmap_type": DefId(1:1090 ~ std[3765]::collections::hash::map::HashMap)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [all_diagnostic_items] calculating the diagnostic items map
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/change_implementation_cross_crate/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_implementation_cross_crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(core[54ad])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(core[54ad]): {"pointer_trait": DefId(2:9322 ~ core[54ad]::fmt::Pointer), "debug_assert_macro": DefId(2:8 ~ core[54ad]::macros::debug_assert), "Hash": DefId(2:9511 ~ core[54ad]::hash::Hash), "mem_uninitialized": DefId(2:1972 ~ core[54ad]::mem::uninitialized), "DoubleEndedIterator": DefId(2:7425 ~ core[54ad]::iter::traits::double_ended::DoubleEndedIterator), "mem_discriminant": DefId(2:2006 ~ core[54ad]::mem::discriminant), "option_type": DefId(2:43195 ~ core[54ad]::option::Option), "atomic_mod": DefId(2:8828 ~ core[54ad]::sync::atomic), "IntoIterator": DefId(2:7381 ~ core[54ad]::iter::traits::collect::IntoIterator), "pointer_trait_fmt": DefId(2:9323 ~ core[54ad]::fmt::Pointer::fmt), "FromIterator": DefId(2:7377 ~ core[54ad]::iter::traits::collect::FromIterator), "mem_zeroed": DefId(2:1970 ~ core[54ad]::mem::zeroed), "Duration": DefId(2:45728 ~ core[54ad]::time::Duration), "mem_drop": DefId(2:1980 ~ core[54ad]::mem::drop), "fence": DefId(2:8933 ~ core[54ad]::sync::atomic::fence), "Copy": DefId(2:2999 ~ core[54ad]::marker::Copy), "PartialEq": DefId(2:2640 ~ core[54ad]::cmp::PartialEq), "Ord": DefId(2:2676 ~ core[54ad]::cmp::Ord), "deref_target": DefId(2:3201 ~ core[54ad]::ops::deref::Deref::Target), "send_trait": DefId(2:2989 ~ core[54ad]::marker::Send), "ref_unwind_safe_trait": DefId(2:8281 ~ core[54ad]::panic::unwind_safe::RefUnwindSafe), "noop_method_borrow": DefId(2:2596 ~ core[54ad]::borrow::{impl#0}::borrow), "try_from_trait": DefId(2:2902 ~ core[54ad]::convert::TryFrom), "mem_forget": DefId(2:1948 ~ core[54ad]::mem::forget), "ptr_null": DefId(2:2502 ~ core[54ad]::ptr::null), "PartialOrd": DefId(2:2687 ~ core[54ad]::cmp::PartialOrd), "Ordering": DefId(2:43399 ~ core[54ad]::sync::atomic::Ordering), "from_trait": DefId(2:2895 ~ core[54ad]::convert::From), "assume_init": DefId(2:1914 ~ core[54ad]::mem::maybe_uninit::{impl#2}::assume_init), "noop_method_clone": DefId(2:2635 ~ core[54ad]::clone::impls::{impl#3}::clone), "debug_trait": DefId(2:9305 ~ core[54ad]::fmt::Debug), "mem_replace": DefId(2:1978 ~ core[54ad]::mem::replace), "ptr_null_mut": DefId(2:2504 ~ core[54ad]::ptr::null_mut), "Default": DefId(2:2967 ~ core[54ad]::default::Default), "compiler_fence": DefId(2:8934 ~ core[54ad]::sync::atomic::compiler_fence), "cmp_max": DefId(2:2705 ~ core[54ad]::cmp::max), "needs_drop": DefId(2:1968 ~ core[54ad]::mem::needs_drop), "gen_future": DefId(2:12725 ~ core[54ad]::future::from_generator::GenFuture), "AsMut": DefId(2:2889 ~ core[54ad]::convert::AsMut), "core_panic_2021_macro": DefId(2:8363 ~ core[54ad]::panic::panic_2021), "noop_method_deref": DefId(2:3206 ~ core[54ad]::ops::deref::{impl#0}::deref), "Eq": DefId(2:2645 ~ core[54ad]::cmp::Eq), "try_into_trait": DefId(2:2898 ~ core[54ad]::convert::TryInto), "mem_size_of": DefId(2:1952 ~ core[54ad]::mem::size_of), "Deref": DefId(2:3200 ~ core[54ad]::ops::deref::Deref), "maybe_uninit_uninit": DefId(2:1905 ~ core[54ad]::mem::maybe_uninit::{impl#2}::uninit), "iter_repeat": DefId(2:7216 ~ core[54ad]::iter::sources::repeat::repeat), "cmp_min": DefId(2:2695 ~ core[54ad]::cmp::min), "unwind_safe_trait": DefId(2:8280 ~ core[54ad]::panic::unwind_safe::UnwindSafe), "core_panic_2015_macro": DefId(2:8362 ~ core[54ad]::panic::panic_2015), "into_trait": DefId(2:2892 ~ core[54ad]::convert::Into), "result_type": DefId(2:43321 ~ core[54ad]::result::Result), "AsRef": DefId(2:2886 ~ core[54ad]::convert::AsRef), "sync_trait": DefId(2:3001 ~ core[54ad]::marker::Sync), "Any": DefId(2:3650 ~ core[54ad]::any::Any), "mem_size_of_val": DefId(2:1954 ~ core[54ad]::mem::size_of_val), "transmute": DefId(2:1659 ~ core[54ad]::intrinsics::#1::transmute), "core_panic_macro": DefId(2:4 ~ core[54ad]::macros::panic), "maybe_uninit_zeroed": DefId(2:1910 ~ core[54ad]::mem::maybe_uninit::{impl#2}::zeroed), "Clone": DefId(2:2610 ~ core[54ad]::clone::Clone), "Borrow": DefId(2:2588 ~ core[54ad]::borrow::Borrow), "display_trait": DefId(2:9312 ~ core[54ad]::fmt::Display), "deref_method": DefId(2:3202 ~ core[54ad]::ops::deref::Deref::deref), "Iterator": DefId(2:7561 ~ core[54ad]::iter::traits::iterator::Iterator), "assert_macro": DefId(2:37 ~ core[54ad]::macros::builtin::assert)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [analysis] running analysis passes on this crate
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/crate_hash_reorder.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(core[54ad])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(core[54ad]): {"pointer_trait": DefId(2:9322 ~ core[54ad]::fmt::Pointer), "debug_assert_macro": DefId(2:8 ~ core[54ad]::macros::debug_assert), "Hash": DefId(2:9511 ~ core[54ad]::hash::Hash), "mem_uninitialized": DefId(2:1972 ~ core[54ad]::mem::uninitialized), "DoubleEndedIterator": DefId(2:7425 ~ core[54ad]::iter::traits::double_ended::DoubleEndedIterator), "mem_discriminant": DefId(2:2006 ~ core[54ad]::mem::discriminant), "option_type": DefId(2:43195 ~ core[54ad]::option::Option), "atomic_mod": DefId(2:8828 ~ core[54ad]::sync::atomic), "IntoIterator": DefId(2:7381 ~ core[54ad]::iter::traits::collect::IntoIterator), "pointer_trait_fmt": DefId(2:9323 ~ core[54ad]::fmt::Pointer::fmt), "FromIterator": DefId(2:7377 ~ core[54ad]::iter::traits::collect::FromIterator), "core_panic_2021_macro": DefId(2:8363 ~ core[54ad]::panic::panic_2021), "mem_zeroed": DefId(2:1970 ~ core[54ad]::mem::zeroed), "Duration": DefId(2:45728 ~ core[54ad]::time::Duration), "mem_drop": DefId(2:1980 ~ core[54ad]::mem::drop), "fence": DefId(2:8933 ~ core[54ad]::sync::atomic::fence), "Copy": DefId(2:2999 ~ core[54ad]::marker::Copy), "PartialEq": DefId(2:2640 ~ core[54ad]::cmp::PartialEq), "Ord": DefId(2:2676 ~ core[54ad]::cmp::Ord), "deref_target": DefId(2:3201 ~ core[54ad]::ops::deref::Deref::Target), "send_trait": DefId(2:2989 ~ core[54ad]::marker::Send), "ref_unwind_safe_trait": DefId(2:8281 ~ core[54ad]::panic::unwind_safe::RefUnwindSafe), "noop_method_borrow": DefId(2:2596 ~ core[54ad]::borrow::{impl#0}::borrow), "try_from_trait": DefId(2:2902 ~ core[54ad]::convert::TryFrom), "mem_forget": DefId(2:1948 ~ core[54ad]::mem::forget), "ptr_null": DefId(2:2502 ~ core[54ad]::ptr::null), "PartialOrd": DefId(2:2687 ~ core[54ad]::cmp::PartialOrd), "Ordering": DefId(2:43399 ~ core[54ad]::sync::atomic::Ordering), "from_trait": DefId(2:2895 ~ core[54ad]::convert::From), "assume_init": DefId(2:1914 ~ core[54ad]::mem::maybe_uninit::{impl#2}::assume_init), "noop_method_clone": DefId(2:2635 ~ core[54ad]::clone::impls::{impl#3}::clone), "debug_trait": DefId(2:9305 ~ core[54ad]::fmt::Debug), "mem_replace": DefId(2:1978 ~ core[54ad]::mem::replace), "ptr_null_mut": DefId(2:2504 ~ core[54ad]::ptr::null_mut), "Default": DefId(2:2967 ~ core[54ad]::default::Default), "compiler_fence": DefId(2:8934 ~ core[54ad]::sync::atomic::compiler_fence), "cmp_max": DefId(2:2705 ~ core[54ad]::cmp::max), "needs_drop": DefId(2:1968 ~ core[54ad]::mem::needs_drop), "gen_future": DefId(2:12725 ~ core[54ad]::future::from_generator::GenFuture), "AsMut": DefId(2:2889 ~ core[54ad]::convert::AsMut), "noop_method_deref": DefId(2:3206 ~ core[54ad]::ops::deref::{impl#0}::deref), "Eq": DefId(2:2645 ~ core[54ad]::cmp::Eq), "try_into_trait": DefId(2:2898 ~ core[54ad]::convert::TryInto), "mem_size_of": DefId(2:1952 ~ core[54ad]::mem::size_of), "Deref": DefId(2:3200 ~ core[54ad]::ops::deref::Deref), "maybe_uninit_uninit": DefId(2:1905 ~ core[54ad]::mem::maybe_uninit::{impl#2}::uninit), "iter_repeat": DefId(2:7216 ~ core[54ad]::iter::sources::repeat::repeat), "cmp_min": DefId(2:2695 ~ core[54ad]::cmp::min), "unwind_safe_trait": DefId(2:8280 ~ core[54ad]::panic::unwind_safe::UnwindSafe), "core_panic_2015_macro": DefId(2:8362 ~ core[54ad]::panic::panic_2015), "into_trait": DefId(2:2892 ~ core[54ad]::convert::Into), "result_type": DefId(2:43321 ~ core[54ad]::result::Result), "AsRef": DefId(2:2886 ~ core[54ad]::convert::AsRef), "sync_trait": DefId(2:3001 ~ core[54ad]::marker::Sync), "Any": DefId(2:3650 ~ core[54ad]::any::Any), "mem_size_of_val": DefId(2:1954 ~ core[54ad]::mem::size_of_val), "transmute": DefId(2:1659 ~ core[54ad]::intrinsics::#1::transmute), "core_panic_macro": DefId(2:4 ~ core[54ad]::macros::panic), "maybe_uninit_zeroed": DefId(2:1910 ~ core[54ad]::mem::maybe_uninit::{impl#2}::zeroed), "Clone": DefId(2:2610 ~ core[54ad]::clone::Clone), "Borrow": DefId(2:2588 ~ core[54ad]::borrow::Borrow), "display_trait": DefId(2:9312 ~ core[54ad]::fmt::Display), "deref_method": DefId(2:3202 ~ core[54ad]::ops::deref::Deref::deref), "Iterator": DefId(2:7561 ~ core[54ad]::iter::traits::iterator::Iterator), "assert_macro": DefId(2:37 ~ core[54ad]::macros::builtin::assert)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [all_diagnostic_items] calculating the diagnostic items map
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/hashes/closure_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zmir-opt-level=0" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(core[54ad])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(core[54ad]): {"pointer_trait": DefId(2:9322 ~ core[54ad]::fmt::Pointer), "debug_assert_macro": DefId(2:8 ~ core[54ad]::macros::debug_assert), "Hash": DefId(2:9511 ~ core[54ad]::hash::Hash), "mem_uninitialized": DefId(2:1972 ~ core[54ad]::mem::uninitialized), "DoubleEndedIterator": DefId(2:7425 ~ core[54ad]::iter::traits::double_ended::DoubleEndedIterator), "mem_discriminant": DefId(2:2006 ~ core[54ad]::mem::discriminant), "option_type": DefId(2:43195 ~ core[54ad]::option::Option), "atomic_mod": DefId(2:8828 ~ core[54ad]::sync::atomic), "IntoIterator": DefId(2:7381 ~ core[54ad]::iter::traits::collect::IntoIterator), "pointer_trait_fmt": DefId(2:9323 ~ core[54ad]::fmt::Pointer::fmt), "FromIterator": DefId(2:7377 ~ core[54ad]::iter::traits::collect::FromIterator), "mem_zeroed": DefId(2:1970 ~ core[54ad]::mem::zeroed), "Duration": DefId(2:45728 ~ core[54ad]::time::Duration), "mem_drop": DefId(2:1980 ~ core[54ad]::mem::drop), "fence": DefId(2:8933 ~ core[54ad]::sync::atomic::fence), "Copy": DefId(2:2999 ~ core[54ad]::marker::Copy), "PartialEq": DefId(2:2640 ~ core[54ad]::cmp::PartialEq), "Ord": DefId(2:2676 ~ core[54ad]::cmp::Ord), "deref_target": DefId(2:3201 ~ core[54ad]::ops::deref::Deref::Target), "send_trait": DefId(2:2989 ~ core[54ad]::marker::Send), "ref_unwind_safe_trait": DefId(2:8281 ~ core[54ad]::panic::unwind_safe::RefUnwindSafe), "noop_method_borrow": DefId(2:2596 ~ core[54ad]::borrow::{impl#0}::borrow), "try_from_trait": DefId(2:2902 ~ core[54ad]::convert::TryFrom), "mem_forget": DefId(2:1948 ~ core[54ad]::mem::forget), "ptr_null": DefId(2:2502 ~ core[54ad]::ptr::null), "PartialOrd": DefId(2:2687 ~ core[54ad]::cmp::PartialOrd), "Ordering": DefId(2:43399 ~ core[54ad]::sync::atomic::Ordering), "from_trait": DefId(2:2895 ~ core[54ad]::convert::From), "assume_init": DefId(2:1914 ~ core[54ad]::mem::maybe_uninit::{impl#2}::assume_init), "noop_method_clone": DefId(2:2635 ~ core[54ad]::clone::impls::{impl#3}::clone), "debug_trait": DefId(2:9305 ~ core[54ad]::fmt::Debug), "mem_replace": DefId(2:1978 ~ core[54ad]::mem::replace), "ptr_null_mut": DefId(2:2504 ~ core[54ad]::ptr::null_mut), "Default": DefId(2:2967 ~ core[54ad]::default::Default), "compiler_fence": DefId(2:8934 ~ core[54ad]::sync::atomic::compiler_fence), "cmp_max": DefId(2:2705 ~ core[54ad]::cmp::max), "needs_drop": DefId(2:1968 ~ core[54ad]::mem::needs_drop), "gen_future": DefId(2:12725 ~ core[54ad]::future::from_generator::GenFuture), "AsMut": DefId(2:2889 ~ core[54ad]::convert::AsMut), "noop_method_deref": DefId(2:3206 ~ core[54ad]::ops::deref::{impl#0}::deref), "Eq": DefId(2:2645 ~ core[54ad]::cmp::Eq), "try_into_trait": DefId(2:2898 ~ core[54ad]::convert::TryInto), "mem_size_of": DefId(2:1952 ~ core[54ad]::mem::size_of), "Deref": DefId(2:3200 ~ core[54ad]::ops::deref::Deref), "maybe_uninit_uninit": DefId(2:1905 ~ core[54ad]::mem::maybe_uninit::{impl#2}::uninit), "iter_repeat": DefId(2:7216 ~ core[54ad]::iter::sources::repeat::repeat), "cmp_min": DefId(2:2695 ~ core[54ad]::cmp::min), "unwind_safe_trait": DefId(2:8280 ~ core[54ad]::panic::unwind_safe::UnwindSafe), "core_panic_2015_macro": DefId(2:8362 ~ core[54ad]::panic::panic_2015), "into_trait": DefId(2:2892 ~ core[54ad]::convert::Into), "result_type": DefId(2:43321 ~ core[54ad]::result::Result), "AsRef": DefId(2:2886 ~ core[54ad]::convert::AsRef), "sync_trait": DefId(2:3001 ~ core[54ad]::marker::Sync), "Any": DefId(2:3650 ~ core[54ad]::any::Any), "mem_size_of_val": DefId(2:1954 ~ core[54ad]::mem::size_of_val), "transmute": DefId(2:1659 ~ core[54ad]::intrinsics::#1::transmute), "core_panic_macro": DefId(2:4 ~ core[54ad]::macros::panic), "maybe_uninit_zeroed": DefId(2:1910 ~ core[54ad]::mem::maybe_uninit::{impl#2}::zeroed), "Clone": DefId(2:2610 ~ core[54ad]::clone::Clone), "Borrow": DefId(2:2588 ~ core[54ad]::borrow::Borrow), "display_trait": DefId(2:9312 ~ core[54ad]::fmt::Display), "deref_method": DefId(2:3202 ~ core[54ad]::ops::deref::Deref::deref), "Iterator": DefId(2:7561 ~ core[54ad]::iter::traits::iterator::Iterator), "core_panic_2021_macro": DefId(2:8363 ~ core[54ad]::panic::panic_2021), "assert_macro": DefId(2:37 ~ core[54ad]::macros::builtin::assert)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -Z mir-opt-level=0 -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [mir_const] processing MIR for `add_type_ascription_to_parameter`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/hashes/enum_constructors.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zmir-opt-level=0" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(std[3765])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(std[3765]): {"PathBuf": DefId(1:7009 ~ std[3765]::path::PathBuf), "OsString": DefId(1:2781 ~ std[3765]::ffi::os_str::OsString), "FileType": DefId(1:10508 ~ std[3765]::fs::FileType), "std_panic_2015_macro": DefId(1:6762 ~ std[3765]::panic::panic_2015), "IoRead": DefId(1:4337 ~ std[3765]::io::Read), "HashMapEntry": DefId(1:1397 ~ std[3765]::collections::hash::map::Entry), "Path": DefId(1:7116 ~ std[3765]::path::Path), "File": DefId(1:3030 ~ std[3765]::fs::File), "OsStr": DefId(1:2784 ~ std[3765]::ffi::os_str::OsStr), "DirBuilder": DefId(1:10526 ~ std[3765]::fs::DirBuilder), "hashset_type": DefId(1:1807 ~ std[3765]::collections::hash::set::HashSet), "CStr": DefId(1:10319 ~ std[3765]::ffi::c_str::CStr), "mutex_type": DefId(1:8401 ~ std[3765]::sync::mutex::Mutex), "cstring_type": DefId(1:10301 ~ std[3765]::ffi::c_str::CString), "Receiver": DefId(1:8127 ~ std[3765]::sync::mpsc::Receiver), "IoWrite": DefId(1:4398 ~ std[3765]::io::Write), "std_panic_macro": DefId(1:9 ~ std[3765]::macros::panic), "hashmap_type": DefId(1:1090 ~ std[3765]::collections::hash::map::HashMap)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -Z mir-opt-level=0 -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [all_diagnostic_items] calculating the diagnostic items map
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/hashes/enum_defs.rs stdout ----

error in revision `cfail3`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: encountered incremental compilation error with diagnostic_items(std[3765])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for diagnostic_items(std[3765]): {"PathBuf": DefId(1:7009 ~ std[3765]::path::PathBuf), "mutex_type": DefId(1:8401 ~ std[3765]::sync::mutex::Mutex), "OsString": DefId(1:2781 ~ std[3765]::ffi::os_str::OsString), "FileType": DefId(1:10508 ~ std[3765]::fs::FileType), "std_panic_2015_macro": DefId(1:6762 ~ std[3765]::panic::panic_2015), "IoRead": DefId(1:4337 ~ std[3765]::io::Read), "HashMapEntry": DefId(1:1397 ~ std[3765]::collections::hash::map::Entry), "Path": DefId(1:7116 ~ std[3765]::path::Path), "File": DefId(1:3030 ~ std[3765]::fs::File), "OsStr": DefId(1:2784 ~ std[3765]::ffi::os_str::OsStr), "DirBuilder": DefId(1:10526 ~ std[3765]::fs::DirBuilder), "hashset_type": DefId(1:1807 ~ std[3765]::collections::hash::set::HashSet), "CStr": DefId(1:10319 ~ std[3765]::ffi::c_str::CStr), "cstring_type": DefId(1:10301 ~ std[3765]::ffi::c_str::CString), "Receiver": DefId(1:8127 ~ std[3765]::sync::mpsc::Receiver), "IoWrite": DefId(1:4398 ~ std[3765]::io::Write), "std_panic_macro": DefId(1:9 ~ std[3765]::macros::panic), "hashmap_type": DefId(1:1090 ~ std[3765]::collections::hash::map::HashMap)}', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:632:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (4851d167f 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [diagnostic_items] calculating the diagnostic items map in a crate
#1 [all_diagnostic_items] calculating the diagnostic items map
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/hashes/indexing_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
---
test result: FAILED. 94 passed; 52 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "incremental" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:05
