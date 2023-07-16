plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
Agent machine name: 'fv-az619'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200614.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200614.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fdfad78d-d247-41e3-a479-5b19d04ab1f9.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73513/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73513/merge:refs/remotes/pull/73513/merge
---
 ---> 31fea614d2f3
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> 4195cadf126d
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 4e90f6b48f05
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> dfa0a356d899
---
Set({"src/librustc_parse_format"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_passes"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_plugin_impl"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_privacy"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_save_analysis"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_serialize"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_session"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_span"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
---
Set({"src/librustc_parse_format"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_serialize"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
...i................................................................................................ 1900/10344
.................................................................................................... 2000/10344
.............................i..i................................................................... 2100/10344
.................................................................................................... 2200/10344
...................iiiii............................................................................ 2300/10344
.................................................................................................... 2500/10344
.................................................................................................... 2600/10344
.................................................................................................... 2700/10344
.................................................................................................... 2800/10344
---
.................................................................................................... 5300/10344
...................................................................................................i 5400/10344
.............................................................................................i...... 5500/10344
.................................................................................................... 5600/10344
.............ii.ii........i...i..................................................................... 5700/10344
.......FF..F..............................................................i......................... 5900/10344
.................................................................................................... 6000/10344
............................ii.....................................i................................ 6100/10344
.................................................................................................... 6200/10344
.................................................................................................... 6200/10344
.................................................................................................... 6300/10344
...........................................................................................ii...i..i 6400/10344
.................................................................................................... 6600/10344
.................................................................................................... 6700/10344
.................................................................................................... 6800/10344
.................................................................................................... 6800/10344
.........................i..ii...................................................................... 6900/10344
.................................................................................................... 7100/10344
.................................................................................i.................. 7200/10344
.................................................................................................... 7300/10344
.................................................................................................... 7400/10344
---
.................................................................................................... 8200/10344
.................................................................................................... 8300/10344
.................................................................................................... 8400/10344
..........................i......................................................................... 8500/10344
................................................................................iiiiii..iiiiii.i.... 8600/10344
......................................i............................................................. 8800/10344
.................................................................................................... 8900/10344
.................................................................................................... 9000/10344
.................................................................................................... 9100/10344
---
diff of stderr:

140   --> $DIR/lint-exceeding-bitshifts.rs:76:15
141    |
142 LL |       let n = 1_isize << BITS;
-    |               ^^^^^^^^^^^^^^^ attempt to shift left by 32_usize which would overflow
+    |               ^^^^^^^^^^^^^^^ attempt to shift left by 64_usize which would overflow
145 warning: this arithmetic operation will overflow
146   --> $DIR/lint-exceeding-bitshifts.rs:77:15

147    |
147    |
148 LL |       let n = 1_usize << BITS;
-    |               ^^^^^^^^^^^^^^^ attempt to shift left by 32_usize which would overflow
+    |               ^^^^^^^^^^^^^^^ attempt to shift left by 64_usize which would overflow
151 warning: 24 warnings emitted
152 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt/lint-exceeding-bitshifts.noopt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts.rs`

error in revision `noopt`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:17:20
   |
LL |     const N: i32 = T::N << 42; //~ WARN: arithmetic operation will overflow
   |                    ^^^^^^^^^^ attempt to shift left by 42_i32 which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:9:9
   |
LL | #![warn(arithmetic_overflow, const_err)]
LL | #![warn(arithmetic_overflow, const_err)]
   |         ^^^^^^^^^^^^^^^^^^^

warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:21:13
   |
LL |     let _ = x << 42; //~ WARN: arithmetic operation will overflow
   |             ^^^^^^^ attempt to shift left by 42_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:26:15
   |
   |
LL |       let n = 1u8 << 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift left by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:28:15
   |
   |
LL |       let n = 1u16 << 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:30:15
   |
   |
LL |       let n = 1u32 << 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 32_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:32:15
   |
   |
LL |       let n = 1u64 << 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:34:15
   |
   |
LL |       let n = 1i8 << 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift left by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:36:15
   |
   |
LL |       let n = 1i16 << 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:38:15
   |
   |
LL |       let n = 1i32 << 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 32_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:40:15
   |
   |
LL |       let n = 1i64 << 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:43:15
   |
   |
LL |       let n = 1u8 >> 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift right by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:45:15
   |
   |
LL |       let n = 1u16 >> 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:47:15
   |
   |
LL |       let n = 1u32 >> 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 32_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:49:15
   |
   |
LL |       let n = 1u64 >> 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:51:15
   |
   |
LL |       let n = 1i8 >> 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift right by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:53:15
   |
   |
LL |       let n = 1i16 >> 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:55:15
   |
   |
LL |       let n = 1i32 >> 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 32_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:57:15
   |
   |
LL |       let n = 1i64 >> 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:61:15
   |
   |
LL |       let n = n << 8; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^ attempt to shift left by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:63:15
   |
   |
LL |       let n = 1u8 << -8; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^ attempt to shift left by -8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:68:15
   |
   |
LL |       let n = 1u8 << (4+4); //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^ attempt to shift left by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:70:15
   |
   |
LL |       let n = 1i64 >> [64][0]; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift right by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:76:15
   |
   |
LL |       let n = 1_isize << BITS; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift left by 64_usize which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:77:15
   |
   |
LL |       let n = 1_usize << BITS; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift left by 64_usize which would overflow
warning: 24 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/lint-exceeding-bitshifts.rs#opt stdout ----
diff of stderr:

140   --> $DIR/lint-exceeding-bitshifts.rs:76:15
141    |
142 LL |       let n = 1_isize << BITS;
-    |               ^^^^^^^^^^^^^^^ attempt to shift left by 32_usize which would overflow
+    |               ^^^^^^^^^^^^^^^ attempt to shift left by 64_usize which would overflow
145 warning: this arithmetic operation will overflow
146   --> $DIR/lint-exceeding-bitshifts.rs:77:15

147    |
147    |
148 LL |       let n = 1_usize << BITS;
-    |               ^^^^^^^^^^^^^^^ attempt to shift left by 32_usize which would overflow
+    |               ^^^^^^^^^^^^^^^ attempt to shift left by 64_usize which would overflow
151 warning: 24 warnings emitted
152 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt/lint-exceeding-bitshifts.opt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts.rs`
error in revision `opt`: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:17:20
   |
LL |     const N: i32 = T::N << 42; //~ WARN: arithmetic operation will overflow
   |                    ^^^^^^^^^^ attempt to shift left by 42_i32 which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:9:9
   |
LL | #![warn(arithmetic_overflow, const_err)]
LL | #![warn(arithmetic_overflow, const_err)]
   |         ^^^^^^^^^^^^^^^^^^^

warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:21:13
   |
LL |     let _ = x << 42; //~ WARN: arithmetic operation will overflow
   |             ^^^^^^^ attempt to shift left by 42_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:26:15
   |
   |
LL |       let n = 1u8 << 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift left by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:28:15
   |
   |
LL |       let n = 1u16 << 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:30:15
   |
   |
LL |       let n = 1u32 << 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 32_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:32:15
   |
   |
LL |       let n = 1u64 << 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:34:15
   |
   |
LL |       let n = 1i8 << 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift left by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:36:15
   |
   |
LL |       let n = 1i16 << 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:38:15
   |
   |
LL |       let n = 1i32 << 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 32_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:40:15
   |
   |
LL |       let n = 1i64 << 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:43:15
   |
   |
LL |       let n = 1u8 >> 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift right by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:45:15
   |
   |
LL |       let n = 1u16 >> 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:47:15
   |
   |
LL |       let n = 1u32 >> 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 32_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:49:15
   |
   |
LL |       let n = 1u64 >> 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:51:15
   |
   |
LL |       let n = 1i8 >> 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift right by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:53:15
   |
   |
LL |       let n = 1i16 >> 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:55:15
   |
   |
LL |       let n = 1i32 >> 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 32_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:57:15
   |
   |
LL |       let n = 1i64 >> 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:61:15
   |
   |
LL |       let n = n << 8; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^ attempt to shift left by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:63:15
   |
   |
LL |       let n = 1u8 << -8; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^ attempt to shift left by -8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:68:15
   |
   |
LL |       let n = 1u8 << (4+4); //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^ attempt to shift left by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:70:15
   |
   |
LL |       let n = 1i64 >> [64][0]; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift right by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:76:15
   |
   |
LL |       let n = 1_isize << BITS; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift left by 64_usize which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:77:15
   |
   |
LL |       let n = 1_usize << BITS; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift left by 64_usize which would overflow
warning: 24 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/lint-exceeding-bitshifts.rs#opt_with_overflow_checks stdout ----
diff of stderr:

140   --> $DIR/lint-exceeding-bitshifts.rs:76:15
141    |
142 LL |       let n = 1_isize << BITS;
-    |               ^^^^^^^^^^^^^^^ attempt to shift left by 32_usize which would overflow
+    |               ^^^^^^^^^^^^^^^ attempt to shift left by 64_usize which would overflow
145 warning: this arithmetic operation will overflow
146   --> $DIR/lint-exceeding-bitshifts.rs:77:15

147    |
147    |
148 LL |       let n = 1_usize << BITS;
-    |               ^^^^^^^^^^^^^^^ attempt to shift left by 32_usize which would overflow
+    |               ^^^^^^^^^^^^^^^ attempt to shift left by 64_usize which would overflow
151 warning: 24 warnings emitted
152 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt_with_overflow_checks/lint-exceeding-bitshifts.opt_with_overflow_checks.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts.rs`

error in revision `opt_with_overflow_checks`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt_with_overflow_checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt_with_overflow_checks/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:17:20
   |
LL |     const N: i32 = T::N << 42; //~ WARN: arithmetic operation will overflow
   |                    ^^^^^^^^^^ attempt to shift left by 42_i32 which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:9:9
   |
LL | #![warn(arithmetic_overflow, const_err)]
LL | #![warn(arithmetic_overflow, const_err)]
   |         ^^^^^^^^^^^^^^^^^^^

warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:21:13
   |
LL |     let _ = x << 42; //~ WARN: arithmetic operation will overflow
   |             ^^^^^^^ attempt to shift left by 42_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:26:15
   |
   |
LL |       let n = 1u8 << 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift left by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:28:15
   |
   |
LL |       let n = 1u16 << 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:30:15
   |
   |
LL |       let n = 1u32 << 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 32_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:32:15
   |
   |
LL |       let n = 1u64 << 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:34:15
   |
   |
LL |       let n = 1i8 << 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift left by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:36:15
   |
   |
LL |       let n = 1i16 << 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:38:15
   |
   |
LL |       let n = 1i32 << 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 32_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:40:15
   |
   |
LL |       let n = 1i64 << 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by 64_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:43:15
   |
   |
LL |       let n = 1u8 >> 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift right by 8_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:45:15
   |
   |
LL |       let n = 1u16 >> 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 16_i32 which would overflow
warning: this arithmetic operation will overflow
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:47:15
   |
   |
LL |       let n = 1u32 >> 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by 32_i32 which would overflow
warning: this arithmetic operation will overflow
---
test result: FAILED. 10275 passed; 3 failed; 66 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:10:21
== clock drift check ==
  local time: Sat Jun 20 16:06:36 UTC 2020
  network time: Sat, 20 Jun 2020 16:06:36 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73513/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73513/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4848) (python)
##[section]Finishing: Finalize Job
