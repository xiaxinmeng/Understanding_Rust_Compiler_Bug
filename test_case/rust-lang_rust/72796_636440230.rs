plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
Agent machine name: 'fv-az578'
Current agent version: '2.169.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200517.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200517.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.2)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3b8f263d-baa2-4a65-8e09-0de776e0342e.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72796/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72796/merge:refs/remotes/pull/72796/merge
---
 ---> cb2676f08729
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> df25ce111862
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 599b9ac96b27
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 091087e35a36
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling unwind v0.0.0 (/checkout/src/libunwind)
   Compiling hashbrown v0.6.2
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/checkout/src/liballoc)
error: internal compiler error: broken MIR in DefId(0:14545 ~ core[1942]::num[0]::{{impl}}[6]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:2326:26
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2326 | |                 unsafe { Bytes { val: self }.bytes }
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2483 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2484 | |     "[0x12]", "[0x12]", "", "" }


error: internal compiler error: broken MIR in DefId(0:14636 ~ core[1942]::num[0]::{{impl}}[7]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:2326:26
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2326 | |                 unsafe { Bytes { val: self }.bytes }
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2489 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2490 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error: internal compiler error: broken MIR in DefId(0:14727 ~ core[1942]::num[0]::{{impl}}[8]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:2326:26
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2326 | |                 unsafe { Bytes { val: self }.bytes }
...    |
2478 | |     }
2479 | | }
     | |_- in this expansion of `int_impl!`
     | |_- in this expansion of `int_impl!`
...
2495 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2496 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2497 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }


error: internal compiler error: broken MIR in DefId(0:14818 ~ core[1942]::num[0]::{{impl}}[9]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:2326:26
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2326 | |                 unsafe { Bytes { val: self }.bytes }
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


error: internal compiler error: broken MIR in DefId(0:14909 ~ core[1942]::num[0]::{{impl}}[10]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:2326:26
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2326 | |                 unsafe { Bytes { val: self }.bytes }
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


error: internal compiler error: broken MIR in DefId(0:15000 ~ core[1942]::num[0]::{{impl}}[11]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:2326:26
308  | / macro_rules! int_impl {
308  | / macro_rules! int_impl {
309  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
310  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
311  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2326 | |                 unsafe { Bytes { val: self }.bytes }
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


error: internal compiler error: broken MIR in DefId(0:15087 ~ core[1942]::num[0]::{{impl}}[12]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:4338:26
2547 | / macro_rules! uint_impl {
2547 | / macro_rules! uint_impl {
2548 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2549 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2550 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
4338 | |                 unsafe { Bytes { val: self }.bytes }
...    |
4486 | |     }
4487 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
4491 | /     uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
4492 | |     "[0x12]", "", "" }


error: internal compiler error: broken MIR in DefId(0:15174 ~ core[1942]::num[0]::{{impl}}[13]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:4338:26
2547 | / macro_rules! uint_impl {
2547 | / macro_rules! uint_impl {
2548 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2549 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2550 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
4338 | |                 unsafe { Bytes { val: self }.bytes }
...    |
4486 | |     }
4487 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
4990 | /     uint_impl! { u16, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
4991 | |     "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error: internal compiler error: broken MIR in DefId(0:15261 ~ core[1942]::num[0]::{{impl}}[14]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:4338:26
2547 | / macro_rules! uint_impl {
2547 | / macro_rules! uint_impl {
2548 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2549 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2550 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
4338 | |                 unsafe { Bytes { val: self }.bytes }
...    |
4486 | |     }
4487 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
4996 | /     uint_impl! { u32, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
4997 | |     "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }


error: internal compiler error: broken MIR in DefId(0:15348 ~ core[1942]::num[0]::{{impl}}[15]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:4338:26
2547 | / macro_rules! uint_impl {
2547 | / macro_rules! uint_impl {
2548 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2549 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2550 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
4338 | |                 unsafe { Bytes { val: self }.bytes }
...    |
4486 | |     }
4487 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
5002 | /     uint_impl! { u64, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
5003 | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
5004 | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
5005 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
5006 | |     "", ""}


error: internal compiler error: broken MIR in DefId(0:15435 ~ core[1942]::num[0]::{{impl}}[16]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:4338:26
2547 | / macro_rules! uint_impl {
2547 | / macro_rules! uint_impl {
2548 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2549 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2550 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
4338 | |                 unsafe { Bytes { val: self }.bytes }
...    |
4486 | |     }
4487 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
5011 | /     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "", 16,
5012 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
5013 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
5014 | |     "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
...    |
5017 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
5018 | |      "", ""}


error: internal compiler error: broken MIR in DefId(0:15522 ~ core[1942]::num[0]::{{impl}}[17]::to_ne_bytes[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [u8; _]
right-hand side has type: [u8; _]
    --> src/libcore/num/mod.rs:4338:26
2547 | / macro_rules! uint_impl {
2547 | / macro_rules! uint_impl {
2548 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2549 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2550 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
4338 | |                 unsafe { Bytes { val: self }.bytes }
...    |
4486 | |     }
4487 | | }
     | |_- in this expansion of `uint_impl!`
     | |_- in this expansion of `uint_impl!`
...
5039 | /     uint_impl! { usize, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
5040 | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
5041 | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
5042 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
5043 | |     usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }


error: internal compiler error: broken MIR in DefId(0:28762 ~ core[1942]::array[0]::{{impl}}[62]::default[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [T; _]
right-hand side has type: [T; 31]
   --> src/libcore/array/mod.rs:423:17
418 |   macro_rules! array_impl_default {
    |  _-
    | |_|
    | |
    | |
419 | |     {$n:expr, $t:ident $($ts:ident)*} => {
420 | |         #[stable(since = "1.4.0", feature = "array_default")]
421 | |         impl<T> Default for [T; $n] where T: Default {
422 | |             fn default() -> [T; $n] {
423 | |                 [$t::default(), $($ts::default()),*]
...   |
...   |
426 | |         array_impl_default!{($n - 1), $($ts)*}
...   |
433 | |     };
434 | | }
    | | -
    | | -
    | |_|
    | |_in this expansion of `array_impl_default!`
    |   in this expansion of `array_impl_default!`
435 | 
436 | | array_impl_default! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}
    | | ----------------------------------------------------------------------------------------- in this macro invocation

error: internal compiler error: broken MIR in DefId(0:28767 ~ core[1942]::array[0]::{{impl}}[63]::default[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [T; _]
right-hand side has type: [T; 30]
   --> src/libcore/array/mod.rs:423:17
418 | / macro_rules! array_impl_default {
418 | / macro_rules! array_impl_default {
419 | |     {$n:expr, $t:ident $($ts:ident)*} => {
420 | |         #[stable(since = "1.4.0", feature = "array_default")]
421 | |         impl<T> Default for [T; $n] where T: Default {
422 | |             fn default() -> [T; $n] {
423 | |                 [$t::default(), $($ts::default()),*]
...   |
...   |
426 | |         array_impl_default!{($n - 1), $($ts)*}
    | |         |
    | |         in this macro invocation (#2)
    | |         in this macro invocation (#3)
...   |
...   |
433 | |     };
434 | | }
    | | -
    | | |
    | | in this expansion of `array_impl_default!` (#1)
    | |_in this expansion of `array_impl_default!` (#2)
    |   in this expansion of `array_impl_default!` (#3)
435 | 
436 |   array_impl_default! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}


error: internal compiler error: broken MIR in DefId(0:28772 ~ core[1942]::array[0]::{{impl}}[64]::default[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [T; _]
right-hand side has type: [T; 29]
   --> src/libcore/array/mod.rs:423:17
418 | / macro_rules! array_impl_default {
418 | / macro_rules! array_impl_default {
419 | |     {$n:expr, $t:ident $($ts:ident)*} => {
420 | |         #[stable(since = "1.4.0", feature = "array_default")]
421 | |         impl<T> Default for [T; $n] where T: Default {
422 | |             fn default() -> [T; $n] {
423 | |                 [$t::default(), $($ts::default()),*]
...   |
...   |
426 | |         array_impl_default!{($n - 1), $($ts)*}
    | |         |
    | |         in this macro invocation (#2)
    | |         in this macro invocation (#3)
...   |
...   |
433 | |     };
434 | | }
    | | -
    | | |
    | | in this expansion of `array_impl_default!` (#1)
    | |_in this expansion of `array_impl_default!` (#2)
    |   in this expansion of `array_impl_default!` (#3)
435 | 
436 |   array_impl_default! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}


error: internal compiler error: broken MIR in DefId(0:28777 ~ core[1942]::array[0]::{{impl}}[65]::default[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [T; _]
right-hand side has type: [T; 28]
   --> src/libcore/array/mod.rs:423:17
418 | / macro_rules! array_impl_default {
418 | / macro_rules! array_impl_default {
419 | |     {$n:expr, $t:ident $($ts:ident)*} => {
420 | |         #[stable(since = "1.4.0", feature = "array_default")]
421 | |         impl<T> Default for [T; $n] where T: Default {
422 | |             fn default() -> [T; $n] {
423 | |                 [$t::default(), $($ts::default()),*]
...   |
...   |
426 | |         array_impl_default!{($n - 1), $($ts)*}
    | |         |
    | |         in this macro invocation (#2)
    | |         in this macro invocation (#3)
...   |
...   |
433 | |     };
434 | | }
    | | -
    | | |
    | | in this expansion of `array_impl_default!` (#1)
    | |_in this expansion of `array_impl_default!` (#2)
    |   in this expansion of `array_impl_default!` (#3)
435 | 
436 |   array_impl_default! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}


error: internal compiler error: broken MIR in DefId(0:28782 ~ core[1942]::array[0]::{{impl}}[66]::default[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [T; _]
right-hand side has type: [T; 27]
   --> src/libcore/array/mod.rs:423:17
418 | / macro_rules! array_impl_default {
418 | / macro_rules! array_impl_default {
419 | |     {$n:expr, $t:ident $($ts:ident)*} => {
420 | |         #[stable(since = "1.4.0", feature = "array_default")]
421 | |         impl<T> Default for [T; $n] where T: Default {
422 | |             fn default() -> [T; $n] {
423 | |                 [$t::default(), $($ts::default()),*]
...   |
...   |
426 | |         array_impl_default!{($n - 1), $($ts)*}
    | |         |
    | |         in this macro invocation (#2)
    | |         in this macro invocation (#3)
...   |
...   |
433 | |     };
434 | | }
    | | -
    | | |
    | | in this expansion of `array_impl_default!` (#1)
    | |_in this expansion of `array_impl_default!` (#2)
    |   in this expansion of `array_impl_default!` (#3)
435 | 
436 |   array_impl_default! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}


error: internal compiler error: broken MIR in DefId(0:28787 ~ core[1942]::array[0]::{{impl}}[67]::default[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [T; _]
right-hand side has type: [T; 26]
   --> src/libcore/array/mod.rs:423:17
418 | / macro_rules! array_impl_default {
418 | / macro_rules! array_impl_default {
419 | |     {$n:expr, $t:ident $($ts:ident)*} => {
420 | |         #[stable(since = "1.4.0", feature = "array_default")]
421 | |         impl<T> Default for [T; $n] where T: Default {
422 | |             fn default() -> [T; $n] {
423 | |                 [$t::default(), $($ts::default()),*]
...   |
...   |
426 | |         array_impl_default!{($n - 1), $($ts)*}
    | |         |
    | |         in this macro invocation (#2)
    | |         in this macro invocation (#3)
...   |
...   |
433 | |     };
434 | | }
    | | -
    | | |
    | | in this expansion of `array_impl_default!` (#1)
    | |_in this expansion of `array_impl_default!` (#2)
    |   in this expansion of `array_impl_default!` (#3)
435 | 
436 |   array_impl_default! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}


error: internal compiler error: broken MIR in DefId(0:28792 ~ core[1942]::array[0]::{{impl}}[68]::default[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [T; _]
right-hand side has type: [T; 25]
   --> src/libcore/array/mod.rs:423:17
418 | / macro_rules! array_impl_default {
418 | / macro_rules! array_impl_default {
419 | |     {$n:expr, $t:ident $($ts:ident)*} => {
420 | |         #[stable(since = "1.4.0", feature = "array_default")]
421 | |         impl<T> Default for [T; $n] where T: Default {
422 | |             fn default() -> [T; $n] {
423 | |                 [$t::default(), $($ts::default()),*]
...   |
...   |
426 | |         array_impl_default!{($n - 1), $($ts)*}
    | |         |
    | |         in this macro invocation (#2)
    | |         in this macro invocation (#3)
...   |
...   |
433 | |     };
434 | | }
    | | -
    | | |
    | | in this expansion of `array_impl_default!` (#1)
    | |_in this expansion of `array_impl_default!` (#2)
    |   in this expansion of `array_impl_default!` (#3)
435 | 
436 |   array_impl_default! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}


error: internal compiler error: broken MIR in DefId(0:28797 ~ core[1942]::array[0]::{{impl}}[69]::default[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [T; _]
right-hand side has type: [T; 24]
   --> src/libcore/array/mod.rs:423:17
418 | / macro_rules! array_impl_default {
418 | / macro_rules! array_impl_default {
419 | |     {$n:expr, $t:ident $($ts:ident)*} => {
420 | |         #[stable(since = "1.4.0", feature = "array_default")]
421 | |         impl<T> Default for [T; $n] where T: Default {
422 | |             fn default() -> [T; $n] {
423 | |                 [$t::default(), $($ts::default()),*]
...   |
...   |
---
  local time: Sun May 31 08:33:44 UTC 2020
  network time: Sun, 31 May 2020 08:33:44 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72796/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72796/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3645) (python)
##[section]Finishing: Finalize Job
