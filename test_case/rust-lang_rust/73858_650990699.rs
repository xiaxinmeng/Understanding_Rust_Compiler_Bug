plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 2'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/79292634-17d4-4011-a67b-1053d3691b47.sh

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
   Compiling autocfg v0.1.7
error[E0723]: loops and conditional expressions are not stable in const fn
   --> src/libcore/num/mod.rs:85:21
    |
45  |  / macro_rules! nonzero_integers {
46  |  |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
48  |  |             doc_comment! {
...    |
...    |
85  | /|                     if n != 0 {
86  | ||                         // SAFETY: we just checked that there's no `0`
87  | ||                         Some(unsafe { Self(n) })
88  | ||                     } else {
89  | ||                         None
90  | ||                     }
    | ||_____________________^
169 |  |     }
170 |  | }
    |  |_- in this expansion of `nonzero_integers!`
171 | 
171 | 
172 | /  nonzero_integers! {
173 | |      #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
174 | |      #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
175 | |      #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
...   |
184 | |      #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
    | |__- in this macro invocation
    |
    |
    = note: see issue #57563 <***/issues/57563> for more information
    = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:748:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
748  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2483 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2484 | |     "[0x12]", "[0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:748:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
748  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2489 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2490 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:748:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
748  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2495 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2496 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2497 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:748:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
748  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2502 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2503 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2504 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2505 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:748:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
748  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2510 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2511 | |     170141183460469231731687303715884105727, "", "", 16,
2512 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2513 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
2516 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2517 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:748:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
748  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2540 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2541 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2542 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2543 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2544 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:790:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
790  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2483 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2484 | |     "[0x12]", "[0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:790:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
790  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2489 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2490 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:790:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
790  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2495 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2496 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2497 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:790:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
790  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2502 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2503 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2504 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2505 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:790:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
790  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2510 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2511 | |     170141183460469231731687303715884105727, "", "", 16,
2512 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2513 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
2516 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2517 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:790:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
790  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2540 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2541 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2542 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2543 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2544 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:832:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
832  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2483 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2484 | |     "[0x12]", "[0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:832:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
832  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2489 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2490 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:832:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
832  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2495 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2496 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2497 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:832:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
832  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2502 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2503 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2504 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2505 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:832:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
832  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2510 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2511 | |     170141183460469231731687303715884105727, "", "", 16,
2512 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2513 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
2516 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2517 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:832:17
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
832  | |                 if b {None} else {Some(a)}
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2540 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2541 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2542 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2543 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2544 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:874:20
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
874  | |                 if rhs == 0 || (self == Self::MIN && rhs == -1) {
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2483 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2484 | |     "[0x12]", "[0x12]", "", "" }
     |
     |
     = note: see issue #57563 <***/issues/57563> for more information
     = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
    --> src/libcore/num/mod.rs:874:20
     |
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
874  | |                 if rhs == 0 || (self == Self::MIN && rhs == -1) {
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2489 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
---
  local time: Mon Jun 29 07:43:20 UTC 2020
  network time: Mon, 29 Jun 2020 07:43:20 GMT
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
Terminate orphan process: pid (4378) (python)
##[section]Finishing: Finalize Job
