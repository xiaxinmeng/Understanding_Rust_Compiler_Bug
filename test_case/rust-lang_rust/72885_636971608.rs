plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1579ad95-5fa4-45ea-8bf3-53d3d46a12e7.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72885/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72885/merge:refs/remotes/pull/72885/merge
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
.............................................F.....................................................F 1700/10277
........F..........................................................................i................ 1800/10277
.................................................................................................... 1900/10277
.................................................................................................... 2000/10277
....i..i............................................................................................ 2100/10277
..............................................................................................iiiii. 2200/10277
.................................................................................................... 2400/10277
.................................................................................................... 2500/10277
.................................................................................................... 2600/10277
.................................................................................................... 2700/10277
---
.........................i...............i.......................................................... 5200/10277
.................................................................................................... 5300/10277
.........................................................................i.......................... 5400/10277
...................................................................i................................ 5500/10277
...................................................................................ii.ii........i... 5600/10277
..........................i......................................................................... 5800/10277
..................................i................................................................. 5900/10277
........................................................................................ii.......... 6000/10277
...........................i........................................................................ 6100/10277
...........................i........................................................................ 6100/10277
.................................................................................................... 6200/10277
.................................................................................................... 6300/10277
..................................................ii...i..ii...........i............................ 6400/10277
.................................................................................................... 6600/10277
.................................................................................................... 6700/10277
.................................................................................................... 6700/10277
...................................................................................i..ii............ 6800/10277
.................................................................................................... 7000/10277
.................................................................................................... 7100/10277
.....................................i.............................................................. 7200/10277
.................................................................................................... 7300/10277
---
.................................................................................................... 8200/10277
.................................................................................................... 8300/10277
...........................................................................i........................ 8400/10277
.................................................................................................... 8500/10277
............................iiiiii.iiiiii.i......................................................... 8600/10277
.................................................................................................... 8800/10277
.................................................................................................... 8900/10277
.................................................................................................... 9000/10277
.................................................................................................... 9100/10277
---

1 error[E0080]: evaluation of constant value failed
2   --> $DIR/shift_overflow.rs:3:9
3    |
- LL |     X = 1 << ((u32::max_value() as u64) + 1),
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to shift left with overflow
+ LL |     X = 1 << ((u32::MAX as u64) + 1),
6 
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/shift_overflow/shift_overflow.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/shift_overflow.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/shift_overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/shift_overflow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/shift_overflow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/shift_overflow.rs:3:9
   |
LL |     X = 1 << ((u32::MAX as u64) + 1), //~ ERROR E0080

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---

67 error[E0716]: temporary value dropped while borrowed
68   --> $DIR/const-int-conversion.rs:14:29
69    |
- LL |     let d: &'static [u8] = &(i32::min_value().to_be().to_ne_bytes());
-    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+ LL |     let d: &'static [u8] = &(i32::MIN.to_be().to_ne_bytes());
+    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
73    |            type annotation requires that borrow lasts for `'static`
74 LL |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion/const-int-conversion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-conversion.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-conversion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:2:28
   |
LL |     let x: &'static i32 = &(5_i32.reverse_bits());
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            type annotation requires that borrow lasts for `'static`
...
LL | }
   | - temporary value is freed at the end of this statement
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:4:28
   |
LL |     let y: &'static i32 = &(i32::from_be_bytes([0x12, 0x34, 0x56, 0x78]));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            type annotation requires that borrow lasts for `'static`
...
LL | }
   | - temporary value is freed at the end of this statement
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:6:28
   |
LL |     let z: &'static i32 = &(i32::from_le_bytes([0x12, 0x34, 0x56, 0x78]));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            type annotation requires that borrow lasts for `'static`
...
LL | }
   | - temporary value is freed at the end of this statement
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:8:28
   |
LL |     let a: &'static i32 = &(i32::from_be(i32::from_ne_bytes([0x80, 0, 0, 0])));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            type annotation requires that borrow lasts for `'static`
...
LL | }
   | - temporary value is freed at the end of this statement
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:10:29
   |
LL |     let b: &'static [u8] = &(0x12_34_56_78_i32.to_be_bytes());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            type annotation requires that borrow lasts for `'static`
...
LL | }
   | - temporary value is freed at the end of this statement
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:12:29
   |
LL |     let c: &'static [u8] = &(0x12_34_56_78_i32.to_le_bytes());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            type annotation requires that borrow lasts for `'static`
...
LL | }
   | - temporary value is freed at the end of this statement
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:14:29
   |
LL |     let d: &'static [u8] = &(i32::MIN.to_be().to_ne_bytes());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            type annotation requires that borrow lasts for `'static`
   |            type annotation requires that borrow lasts for `'static`
LL |         //~^ ERROR temporary value dropped while borrowed
   | - temporary value is freed at the end of this statement

error: aborting due to 7 previous errors

---

355 error: any use of this value will cause an error
356   --> $DIR/const-int-unchecked.rs:134:25
357    |
- LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(i32::min_value(), -1) };
-    | ------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
+ LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(i32::MIN, -1) };
360    |                         |
361    |                         overflow executing `unchecked_div`
362 


371 error: any use of this value will cause an error
372   --> $DIR/const-int-unchecked.rs:139:25
373    |
- LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(i32::min_value(), -1) };
-    | ------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
+ LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(i32::MIN, -1) };
376    |                         |
377    |                         overflow executing `unchecked_rem`
378 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked/const-int-unchecked.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-unchecked.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-unchecked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:15:29
   |
LL | const SHL_U8: u8 = unsafe { intrinsics::unchecked_shl(5_u8, 8) };
   |                             |
   |                             overflowing shift by 8 in `unchecked_shl`
   |
   = note: `#[deny(const_err)]` on by default
   = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:17:31
   |
LL | const SHL_U16: u16 = unsafe { intrinsics::unchecked_shl(5_u16, 16) };
   |                               |
   |                               overflowing shift by 16 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:19:31
   |
LL | const SHL_U32: u32 = unsafe { intrinsics::unchecked_shl(5_u32, 32) };
   |                               |
   |                               overflowing shift by 32 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:21:31
   |
LL | const SHL_U64: u64 = unsafe { intrinsics::unchecked_shl(5_u64, 64) };
   |                               |
   |                               overflowing shift by 64 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:23:33
   |
LL | const SHL_U128: u128 = unsafe { intrinsics::unchecked_shl(5_u128, 128) };
   |                                 |
   |                                 overflowing shift by 128 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:28:29
   |
LL | const SHL_I8: i8 = unsafe { intrinsics::unchecked_shl(5_i8, 8) };
   |                             |
   |                             overflowing shift by 8 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:30:31
   |
LL | const SHL_I16: i16 = unsafe { intrinsics::unchecked_shl(5_16, 16) };
   |                               |
   |                               overflowing shift by 16 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:32:31
   |
LL | const SHL_I32: i32 = unsafe { intrinsics::unchecked_shl(5_i32, 32) };
   |                               |
   |                               overflowing shift by 32 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:34:31
   |
LL | const SHL_I64: i64 = unsafe { intrinsics::unchecked_shl(5_i64, 64) };
   |                               |
   |                               overflowing shift by 64 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:36:33
   |
LL | const SHL_I128: i128 = unsafe { intrinsics::unchecked_shl(5_i128, 128) };
   |                                 |
   |                                 overflowing shift by 128 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:41:33
   |
LL | const SHL_I8_NEG: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -1) };
   |                                 |
   |                                 overflowing shift by 255 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:43:35
   |
LL | const SHL_I16_NEG: i16 = unsafe { intrinsics::unchecked_shl(5_16, -1) };
   |                                   |
   |                                   overflowing shift by 65535 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:45:35
   |
LL | const SHL_I32_NEG: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -1) };
   |                                   |
   |                                   overflowing shift by 4294967295 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:47:35
   |
LL | const SHL_I64_NEG: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -1) };
   |                                   |
   |                                   overflowing shift by 18446744073709551615 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:49:37
   |
LL | const SHL_I128_NEG: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -1) };
   |                                     |
   |                                     overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:55:40
   |
LL | const SHL_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -6) };
   |                                        |
   |                                        overflowing shift by 250 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:57:42
   |
LL | const SHL_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shl(5_16, -13) };
   |                                          |
   |                                          overflowing shift by 65523 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:59:42
   |
LL | const SHL_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -25) };
   |                                          |
   |                                          overflowing shift by 4294967271 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:61:42
   |
LL | const SHL_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -30) };
   |                                          |
   |                                          overflowing shift by 18446744073709551586 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:63:44
   |
LL | const SHL_I128_NEG_RANDOM: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -93) };
   |                                            |
   |                                            overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shl`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:70:29
   |
LL | const SHR_U8: u8 = unsafe { intrinsics::unchecked_shr(5_u8, 8) };
   |                             |
   |                             overflowing shift by 8 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:72:31
   |
LL | const SHR_U16: u16 = unsafe { intrinsics::unchecked_shr(5_u16, 16) };
   |                               |
   |                               overflowing shift by 16 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:74:31
   |
LL | const SHR_U32: u32 = unsafe { intrinsics::unchecked_shr(5_u32, 32) };
   |                               |
   |                               overflowing shift by 32 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:76:31
   |
LL | const SHR_U64: u64 = unsafe { intrinsics::unchecked_shr(5_u64, 64) };
   |                               |
   |                               overflowing shift by 64 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:78:33
   |
LL | const SHR_U128: u128 = unsafe { intrinsics::unchecked_shr(5_u128, 128) };
   |                                 |
   |                                 overflowing shift by 128 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:83:29
   |
LL | const SHR_I8: i8 = unsafe { intrinsics::unchecked_shr(5_i8, 8) };
   |                             |
   |                             overflowing shift by 8 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:85:31
   |
LL | const SHR_I16: i16 = unsafe { intrinsics::unchecked_shr(5_16, 16) };
   |                               |
   |                               overflowing shift by 16 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:87:31
   |
LL | const SHR_I32: i32 = unsafe { intrinsics::unchecked_shr(5_i32, 32) };
   |                               |
   |                               overflowing shift by 32 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:89:31
   |
LL | const SHR_I64: i64 = unsafe { intrinsics::unchecked_shr(5_i64, 64) };
   |                               |
   |                               overflowing shift by 64 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:91:33
   |
LL | const SHR_I128: i128 = unsafe { intrinsics::unchecked_shr(5_i128, 128) };
   |                                 |
   |                                 overflowing shift by 128 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:96:33
   |
LL | const SHR_I8_NEG: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -1) };
   |                                 |
   |                                 overflowing shift by 255 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:98:35
   |
LL | const SHR_I16_NEG: i16 = unsafe { intrinsics::unchecked_shr(5_16, -1) };
   |                                   |
   |                                   overflowing shift by 65535 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:100:35
   |
LL | const SHR_I32_NEG: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -1) };
   |                                   |
   |                                   overflowing shift by 4294967295 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:102:35
   |
LL | const SHR_I64_NEG: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -1) };
   |                                   |
   |                                   overflowing shift by 18446744073709551615 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:104:37
   |
LL | const SHR_I128_NEG: i128 = unsafe { intrinsics::unchecked_shr(5_i128, -1) };
   |                                     |
   |                                     overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:110:40
   |
LL | const SHR_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -6) };
   |                                        |
   |                                        overflowing shift by 250 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:112:42
   |
LL | const SHR_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shr(5_16, -13) };
   |                                          |
   |                                          overflowing shift by 65523 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:114:42
   |
LL | const SHR_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -25) };
   |                                          |
   |                                          overflowing shift by 4294967271 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:116:42
   |
LL | const SHR_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -30) };
   |                                          |
   |                                          overflowing shift by 18446744073709551586 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:118:44
   |
LL | const SHR_I128_NEG_RANDOM: i128 = unsafe { intrinsics::unchecked_shr(5_i128, -93) };
   |                                            |
   |                                            overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shr`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:123:25
   |
LL | const _: u16 = unsafe { std::intrinsics::unchecked_add(40000u16, 30000) };
   |                         |
   |                         overflow executing `unchecked_add`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:126:25
   |
LL | const _: u32 = unsafe { std::intrinsics::unchecked_sub(14u32, 22) };
   |                         |
   |                         overflow executing `unchecked_sub`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:129:25
   |
LL | const _: u16 = unsafe { std::intrinsics::unchecked_mul(300u16, 250u16) };
   |                         |
   |                         overflow executing `unchecked_mul`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:132:25
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(1, 0) };
   |                         |
   |                         dividing by zero

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:134:25
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(i32::MIN, -1) };
   |                         |
   |                         overflow executing `unchecked_div`

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:137:25
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(1, 0) };
   |                         |
   |                         calculating the remainder with a divisor of zero

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:139:25
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(i32::MIN, -1) };
   |                         |
   |                         overflow executing `unchecked_rem`

error: aborting due to 47 previous errors
---
test result: FAILED. 10209 passed; 3 failed; 65 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:04:41
== clock drift check ==
  local time: Mon Jun  1 16:29:34 UTC 2020
  network time: Mon, 01 Jun 2020 16:29:35 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72885/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72885/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3370) (python)
##[section]Finishing: Finalize Job
