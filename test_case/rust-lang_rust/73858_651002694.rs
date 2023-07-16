plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
Agent machine name: 'fv-az578'
Current agent version: '2.171.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200621.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200621.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.171.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0d6ab3b1-2269-4dbc-af25-3c9d1caf24a0.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73858/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73858/merge:refs/remotes/pull/73858/merge
---
 ---> a9ec21d337b3
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 5ff2c13d8dba
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 6b931e755c7e
Successfully built 6b931e755c7e
Successfully tagged rust-ci:latest
Built container sha256:6b931e755c7ea1f69816640eb9df74fafd40a545d0e5aa8341d35009dabb0f3c
---
   Compiling cc v1.0.54
    Checking core v0.0.0 (/checkout/src/libcore)
   Compiling libc v0.2.71
   Compiling autocfg v0.1.7
error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<i8>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:883:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
883  | |                     Some(unsafe { intrinsics::unchecked_div(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2504 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2505 | |     "[0x12]", "[0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<i16>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:883:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
883  | |                     Some(unsafe { intrinsics::unchecked_div(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2510 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2511 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<i32>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:883:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
883  | |                     Some(unsafe { intrinsics::unchecked_div(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2516 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2517 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2518 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<i64>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:883:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
883  | |                     Some(unsafe { intrinsics::unchecked_div(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2523 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2524 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2525 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2526 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<i128>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:883:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
883  | |                     Some(unsafe { intrinsics::unchecked_div(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2531 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2532 | |     170141183460469231731687303715884105727, "", "", 16,
2533 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2534 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
2537 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2538 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<isize>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:883:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
883  | |                     Some(unsafe { intrinsics::unchecked_div(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2561 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2562 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2563 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2564 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2565 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_rem::<i8>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:943:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
943  | |                     Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2504 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2505 | |     "[0x12]", "[0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_rem::<i16>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:943:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
943  | |                     Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2510 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2511 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_rem::<i32>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:943:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
943  | |                     Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2516 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2517 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2518 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_rem::<i64>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:943:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
943  | |                     Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2523 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2524 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2525 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2526 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_rem::<i128>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:943:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
943  | |                     Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2531 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2532 | |     170141183460469231731687303715884105727, "", "", 16,
2533 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2534 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
2537 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2538 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_rem::<isize>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:943:35
309  | / macro_rules! int_impl {
309  | / macro_rules! int_impl {
310  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
311  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
312  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
943  | |                     Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
...    |
2499 | |     }
2500 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2561 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2562 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2563 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2564 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2565 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<u8>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:3132:42
2568 | / macro_rules! uint_impl {
2568 | / macro_rules! uint_impl {
2569 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2570 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2571 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
3132 | |                     rhs => Some(unsafe { intrinsics::unchecked_div(self, rhs) }),
...    |
4519 | |     }
4520 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
4524 | /     uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
4525 | |     "[0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<u16>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:3132:42
2568 | / macro_rules! uint_impl {
2568 | / macro_rules! uint_impl {
2569 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2570 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2571 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
3132 | |                     rhs => Some(unsafe { intrinsics::unchecked_div(self, rhs) }),
...    |
4519 | |     }
4520 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
5033 | /     uint_impl! { u16, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
5034 | |     "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<u32>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:3132:42
2568 | / macro_rules! uint_impl {
2568 | / macro_rules! uint_impl {
2569 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2570 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2571 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
3132 | |                     rhs => Some(unsafe { intrinsics::unchecked_div(self, rhs) }),
...    |
4519 | |     }
4520 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
5039 | /     uint_impl! { u32, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
5040 | |     "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<u64>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:3132:42
2568 | / macro_rules! uint_impl {
2568 | / macro_rules! uint_impl {
2569 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2570 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2571 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
3132 | |                     rhs => Some(unsafe { intrinsics::unchecked_div(self, rhs) }),
...    |
4519 | |     }
4520 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
5045 | /     uint_impl! { u64, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
5046 | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
5047 | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
5048 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
5049 | |     "", ""}
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<u128>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:3132:42
2568 | / macro_rules! uint_impl {
2568 | / macro_rules! uint_impl {
2569 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2570 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2571 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
3132 | |                     rhs => Some(unsafe { intrinsics::unchecked_div(self, rhs) }),
...    |
4519 | |     }
4520 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
5054 | /     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "", 16,
5055 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
5056 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
5057 | |     "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
...    |
5060 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
5061 | |      "", ""}
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_div::<usize>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:3132:42
2568 | / macro_rules! uint_impl {
2568 | / macro_rules! uint_impl {
2569 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2570 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2571 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
3132 | |                     rhs => Some(unsafe { intrinsics::unchecked_div(self, rhs) }),
...    |
4519 | |     }
4520 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
5082 | /     uint_impl! { usize, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
5083 | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
5084 | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
5085 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
5086 | |     usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_rem::<u8>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:3189:35
2568 | / macro_rules! uint_impl {
2568 | / macro_rules! uint_impl {
2569 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2570 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2571 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
3189 | |                     Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
...    |
4519 | |     }
4520 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
4524 | /     uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
4525 | |     "[0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_rem::<u16>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:3189:35
2568 | / macro_rules! uint_impl {
2568 | / macro_rules! uint_impl {
2569 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2570 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2571 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
3189 | |                     Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
...    |
4519 | |     }
4520 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
5033 | /     uint_impl! { u16, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
5034 | |     "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_rem::<u32>` is not stable as `const fn`
    --> src/libcore/num/mod.rs:3189:35
2568 | / macro_rules! uint_impl {
2568 | / macro_rules! uint_impl {
2569 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2570 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2571 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
3189 | |                     Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
...    |
4519 | |     }
4520 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
5039 | /     uint_impl! { u32, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
5040 | |     "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::unchecked_rem::<u64>` is not stable as `const fn`
---
  local time: Mon Jun 29 08:07:56 UTC 2020
  network time: Mon, 29 Jun 2020 08:07:56 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73858/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73858/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4446) (python)
##[section]Finishing: Finalize Job
