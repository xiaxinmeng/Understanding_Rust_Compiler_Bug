plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 46'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200430.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200430.2/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8d3a12d4-48f8-4034-bf7f-a08cd60bf8d4.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71500/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71500/merge:refs/remotes/pull/71500/merge
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
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.......................................................i............................................ 1800/10171
..................................................................................F......F.......... 1900/10171
............................................................................i..i.................... 2000/10171
.................................................................................................... 2100/10171
..................................................................iiiii............................. 2200/10171
.................................................................................................... 2400/10171
.................................................................................................... 2500/10171
.................................................................................................... 2600/10171
.................................................................................................... 2700/10171
---
.................................................................................................... 5200/10171
.................................................................................................... 5300/10171
.............................i...................................................................... 5400/10171
......................i............................................................................. 5500/10171
..............................ii.ii........i...i.................................................... 5600/10171
................................................................................i................... 5800/10171
.................................................................................................... 5900/10171
...........................ii.....................................i................................. 6000/10171
.................................................................................................... 6100/10171
.................................................................................................... 6100/10171
.................................................................................................... 6200/10171
........................................................................................ii...i...ii. 6300/10171
.................................................................................................... 6500/10171
.................................................................................................... 6600/10171
.................................................................................................... 6700/10171
.................................................................................................... 6700/10171
.....................i..ii.......................................................................... 6800/10171
.................................................................................................... 7000/10171
...........................................................................i........................ 7100/10171
.................................................................................................... 7200/10171
.................................................................................................... 7300/10171
---
.................................................................................................... 8100/10171
.................................................................................................... 8200/10171
.............................................................................................i...... 8300/10171
.................................................................................................... 8400/10171
...............................................iiiiii.iiiii.i....................................... 8500/10171
i................................................................................................... 8700/10171
.................................................................................................... 8800/10171
.................................................................................................... 8900/10171
.................................................................................................... 9000/10171
---

---- [ui] ui/consts/miri_unleashed/ptr_arith.rs stdout ----
diff of stderr:

10 LL |     let _v = x + 0;
11    |              ^^^^^ "pointer-to-integer cast" needs an rfc before being allowed inside constants
- error[E0080]: could not evaluate static initializer
-   --> $DIR/ptr_arith.rs:24:14
-    |
-    |
- LL |     let _v = core::intrinsics::offset(x, 0);
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ "calling intrinsic `offset`" needs an rfc before being allowed inside constants
19 warning: skipping const checks
20    |
21 help: skipping check for `const_compare_raw_pointers` feature


34 LL |     let _v = core::intrinsics::offset(x, 0);
36 
36 
- error: aborting due to 3 previous errors; 1 warning emitted
+ error: aborting due to 2 previous errors; 1 warning emitted
39 For more information about this error, try `rustc --explain E0080`.
40 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/ptr_arith/ptr_arith.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/ptr_arith.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/ptr_arith" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/ptr_arith/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:10:14
   |
LL |     let _v = x == x;
   |              ^^^^^^ "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:17:14
   |
   |
LL |     let _v = x + 0;
   |              ^^^^^ "pointer-to-integer cast" needs an rfc before being allowed inside constants
warning: skipping const checks
   |
help: skipping check for `const_compare_raw_pointers` feature
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:10:14
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:10:14
   |
LL |     let _v = x == x;
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:16:20
   |
LL |     let x: usize = std::mem::transmute(&0);
LL |     let x: usize = std::mem::transmute(&0);
   |                    ^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:24:14
   |
LL |     let _v = core::intrinsics::offset(x, 0);


error: aborting due to 2 previous errors; 1 warning emitted
For more information about this error, try `rustc --explain E0080`.

------------------------------------------



---- [ui] ui/consts/offset_ub.rs stdout ----
diff of stderr:

6    |         |
7    |         overflowing in-bounds pointer arithmetic
8    |         inside `std::ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
-    |         inside `BEFORE_START` at $DIR/offset_ub.rs:6:46
+    |         inside `BEFORE_START` at $DIR/offset_ub.rs:7:46
10    | 
-   ::: $DIR/offset_ub.rs:6:1
+   ::: $DIR/offset_ub.rs:7:1
12    |
13 LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) };

23    |         |
23    |         |
24    |         inbounds test failed: pointer must be in-bounds at offset 2, but is outside bounds of allocN which has size 1
25    |         inside `std::ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
-    |         inside `AFTER_END` at $DIR/offset_ub.rs:7:43
+    |         inside `AFTER_END` at $DIR/offset_ub.rs:8:43
27    | 
-   ::: $DIR/offset_ub.rs:7:1
+   ::: $DIR/offset_ub.rs:8:1
29    |
30 LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) };

38    |         |
38    |         |
39    |         inbounds test failed: pointer must be in-bounds at offset 101, but is outside bounds of allocN which has size 100
40    |         inside `std::ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
-    |         inside `AFTER_ARRAY` at $DIR/offset_ub.rs:8:45
+    |         inside `AFTER_ARRAY` at $DIR/offset_ub.rs:9:45
42    | 
-   ::: $DIR/offset_ub.rs:8:1
+   ::: $DIR/offset_ub.rs:9:1
44    |
45 LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) };

53    |         |
54    |         inbounds pointer arithmetic: overflow computing offset
54    |         inbounds pointer arithmetic: overflow computing offset
55    |         inside `std::ptr::const_ptr::<impl *const u16>::offset` at $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
-    |         inside `OVERFLOW` at $DIR/offset_ub.rs:10:43
+    |         inside `OVERFLOW` at $DIR/offset_ub.rs:11:43
57    | 
-   ::: $DIR/offset_ub.rs:10:1
+   ::: $DIR/offset_ub.rs:11:1
59    |
60 LL | pub const OVERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MAX) };

68    |         |
69    |         inbounds pointer arithmetic: overflow computing offset
69    |         inbounds pointer arithmetic: overflow computing offset
70    |         inside `std::ptr::const_ptr::<impl *const u16>::offset` at $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
-    |         inside `UNDERFLOW` at $DIR/offset_ub.rs:11:44
+    |         inside `UNDERFLOW` at $DIR/offset_ub.rs:12:44
72    | 
-   ::: $DIR/offset_ub.rs:11:1
+   ::: $DIR/offset_ub.rs:12:1
74    |
75 LL | pub const UNDERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MIN) };

83    |         |
84    |         overflowing in-bounds pointer arithmetic
84    |         overflowing in-bounds pointer arithmetic
85    |         inside `std::ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
-    |         inside `OVERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:12:56
+    |         inside `OVERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:13:56
87    | 
-   ::: $DIR/offset_ub.rs:12:1
+   ::: $DIR/offset_ub.rs:13:1
89    |
90 LL | pub const OVERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (usize::MAX as *const u8).offset(2) };

98    |         |
99    |         overflowing in-bounds pointer arithmetic
99    |         overflowing in-bounds pointer arithmetic
100    |         inside `std::ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
-    |         inside `UNDERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:13:57
+    |         inside `UNDERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:14:57
102    | 
-   ::: $DIR/offset_ub.rs:13:1
+   ::: $DIR/offset_ub.rs:14:1
104    |
105 LL | pub const UNDERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (1 as *const u8).offset(-2) };

113    |         |
113    |         |
114    |         inbounds test failed: pointer must be in-bounds at offset 1, but is outside bounds of allocN which has size 0
115    |         inside `std::ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
-    |         inside `ZERO_SIZED_ALLOC` at $DIR/offset_ub.rs:15:50
+    |         inside `ZERO_SIZED_ALLOC` at $DIR/offset_ub.rs:16:50
117    | 
-   ::: $DIR/offset_ub.rs:15:1
+   ::: $DIR/offset_ub.rs:16:1
119    |
120 LL | pub const ZERO_SIZED_ALLOC: *const u8 = unsafe { [0u8; 0].as_ptr().offset(1) };

128    |         |
129    |         unable to turn bytes into a pointer
129    |         unable to turn bytes into a pointer
130    |         inside `std::ptr::mut_ptr::<impl *mut u8>::offset` at $SRC_DIR/libcore/ptr/mut_ptr.rs:LL:COL
-    |         inside `DANGLING` at $DIR/offset_ub.rs:16:42
+    |         inside `DANGLING` at $DIR/offset_ub.rs:17:42
132    | 
-   ::: $DIR/offset_ub.rs:16:1
+   ::: $DIR/offset_ub.rs:17:1
134    |
135 LL | pub const DANGLING: *const u8 = unsafe { ptr::NonNull::<u8>::dangling().as_ptr().offset(4) };

143    |         |
143    |         |
144    |         inbounds test failed: 0x0 is not a valid pointer
145    |         inside `std::ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
-    |         inside `NULL_OFFSET_ZERO` at $DIR/offset_ub.rs:19:50
+    |         inside `NULL_OFFSET_ZERO` at $DIR/offset_ub.rs:20:50
147    | 
-   ::: $DIR/offset_ub.rs:19:1
+   ::: $DIR/offset_ub.rs:20:1
149    |
150 LL | pub const NULL_OFFSET_ZERO: *const u8 = unsafe { ptr::null::<u8>().offset(0) };


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/offset_ub.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/offset_ub.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL |         intrinsics::offset(self, count)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         overflowing in-bounds pointer arithmetic
   |         inside `std::ptr::const_ptr::<impl *const u8>::offset` at /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |         inside `BEFORE_START` at /checkout/src/test/ui/consts/offset_ub.rs:7:46
  ::: /checkout/src/test/ui/consts/offset_ub.rs:7:1
   |
   |
LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) }; //~NOTE
   |
   = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
---
   |         inside `AFTER_END` at /checkout/src/test/ui/consts/offset_ub.rs:8:43
   | 
  ::: /checkout/src/test/ui/consts/offset_ub.rs:8:1
   |
LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) }; //~NOTE

error: any use of this value will cause an error
  --> /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |
   |
LL |         intrinsics::offset(self, count)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         inbounds test failed: pointer must be in-bounds at offset 101, but is outside bounds of alloc10 which has size 100
   |         inside `std::ptr::const_ptr::<impl *const u8>::offset` at /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |         inside `AFTER_ARRAY` at /checkout/src/test/ui/consts/offset_ub.rs:9:45
  ::: /checkout/src/test/ui/consts/offset_ub.rs:9:1
   |
   |
LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) }; //~NOTE

error: any use of this value will cause an error
  --> /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |
   |
LL |         intrinsics::offset(self, count)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         inbounds pointer arithmetic: overflow computing offset
   |         inside `std::ptr::const_ptr::<impl *const u16>::offset` at /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |         inside `OVERFLOW` at /checkout/src/test/ui/consts/offset_ub.rs:11:43
  ::: /checkout/src/test/ui/consts/offset_ub.rs:11:1
   |
   |
LL | pub const OVERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MAX) }; //~NOTE

error: any use of this value will cause an error
  --> /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |
   |
LL |         intrinsics::offset(self, count)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         inbounds pointer arithmetic: overflow computing offset
   |         inside `std::ptr::const_ptr::<impl *const u16>::offset` at /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |         inside `UNDERFLOW` at /checkout/src/test/ui/consts/offset_ub.rs:12:44
  ::: /checkout/src/test/ui/consts/offset_ub.rs:12:1
   |
   |
LL | pub const UNDERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MIN) }; //~NOTE

error: any use of this value will cause an error
  --> /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |
---
   |         inside `OVERFLOW_ADDRESS_SPACE` at /checkout/src/test/ui/consts/offset_ub.rs:13:56
   | 
  ::: /checkout/src/test/ui/consts/offset_ub.rs:13:1
   |
LL | pub const OVERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (usize::MAX as *const u8).offset(2) }; //~NOTE

error: any use of this value will cause an error
  --> /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |
---
   |         inside `UNDERFLOW_ADDRESS_SPACE` at /checkout/src/test/ui/consts/offset_ub.rs:14:57
   | 
  ::: /checkout/src/test/ui/consts/offset_ub.rs:14:1
   |
LL | pub const UNDERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (1 as *const u8).offset(-2) }; //~NOTE

error: any use of this value will cause an error
  --> /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |
---
   |         inside `ZERO_SIZED_ALLOC` at /checkout/src/test/ui/consts/offset_ub.rs:16:50
   | 
  ::: /checkout/src/test/ui/consts/offset_ub.rs:16:1
   |
LL | pub const ZERO_SIZED_ALLOC: *const u8 = unsafe { [0u8; 0].as_ptr().offset(1) }; //~NOTE

error: any use of this value will cause an error
  --> /checkout/src/libcore/ptr/mut_ptr.rs:153:9
   |
   |
LL |         intrinsics::offset(self, count) as *mut T
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         unable to turn bytes into a pointer
   |         inside `std::ptr::mut_ptr::<impl *mut u8>::offset` at /checkout/src/libcore/ptr/mut_ptr.rs:153:9
   |         inside `DANGLING` at /checkout/src/test/ui/consts/offset_ub.rs:17:42
  ::: /checkout/src/test/ui/consts/offset_ub.rs:17:1
   |
   |
LL | pub const DANGLING: *const u8 = unsafe { ptr::NonNull::<u8>::dangling().as_ptr().offset(4) }; //~NOTE

error: any use of this value will cause an error
  --> /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |
   |
LL |         intrinsics::offset(self, count)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         inbounds test failed: 0x0 is not a valid pointer
   |         inside `std::ptr::const_ptr::<impl *const u8>::offset` at /checkout/src/libcore/ptr/const_ptr.rs:159:9
   |         inside `NULL_OFFSET_ZERO` at /checkout/src/test/ui/consts/offset_ub.rs:20:50
  ::: /checkout/src/test/ui/consts/offset_ub.rs:20:1
   |
   |
LL | pub const NULL_OFFSET_ZERO: *const u8 = unsafe { ptr::null::<u8>().offset(0) }; //~NOTE

error: aborting due to 10 previous errors


---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:59:03
Build completed unsuccessfully in 0:59:03
== clock drift check ==
  local time: Sat May 16 22:16:01 UTC 2020
  network time: Sat, 16 May 2020 22:16:02 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71500/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71500/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3745) (python)
##[section]Finishing: Finalize Job
