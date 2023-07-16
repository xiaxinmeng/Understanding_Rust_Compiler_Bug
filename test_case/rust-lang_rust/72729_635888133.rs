plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 55'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d48cd5b3-eb5f-4081-86db-af856749b5d4.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72729/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72729/merge:refs/remotes/pull/72729/merge
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
..............................................................................i..................... 1800/10260
.................................................................................................... 1900/10260
...................................................................................................i 2000/10260
..i................................................................................................. 2100/10260
.........................................................................................iiiii...... 2200/10260
.................................................................................................... 2400/10260
.................................................................................................... 2500/10260
.................................................................................................... 2600/10260
.................................................................................................... 2700/10260
---
..................i...............i................................................................. 5200/10260
.................................................................................................... 5300/10260
.........................................F........................i................................. 5400/10260
...........................................................i........................................ 5500/10260
.......................................................................ii.ii........i...i........... 5600/10260
..........F....i.................................................................................... 5800/10260
.......................i............................................................................ 5900/10260
............................................................................ii...................... 6000/10260
...............i.................................................................................... 6100/10260
...............i.................................................................................... 6100/10260
.................................................................................................... 6200/10260
.................................................................................................... 6300/10260
......................................ii...i..ii...........i........................................ 6400/10260
.................................................................................................... 6600/10260
.................................................................................................... 6700/10260
.................................................................................................... 6700/10260
.......................................................................i..ii........................ 6800/10260
.................................................................................................... 7000/10260
.................................................................................................... 7100/10260
.........................i.......................................................................... 7200/10260
.................................................................................................... 7300/10260
---
.................................................................................................... 8200/10260
.................................................................................................... 8300/10260
.............................................................i...................................... 8400/10260
.................................................................................................... 8500/10260
...............iiiiii.iiiiii.i...................................................................... 8600/10260
.................................................................................................... 8800/10260
.................................................................................................... 8900/10260
.................................................................................................... 9000/10260
.................................................................................................... 9100/10260
---

12    |
13 LL | #![warn(clashing_extern_decl)]
14    |         ^^^^^^^^^^^^^^^^^^^^
-    = note: expected  `unsafe extern "C" fn(*const usize) -> bool`
-               found  `unsafe extern "C" fn(*const bool) -> bool`
+    = note: expected `unsafe extern "C" fn(*const usize) -> bool`
+               found `unsafe extern "C" fn(*const bool) -> bool`
18 warning: 1 warning emitted
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1866/issue-1866.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-1866.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-1866.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1866" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1866/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `rust_task_is_unwinding` redeclared with a different signature
  --> /checkout/src/test/ui/issues/issue-1866.rs:23:13
   |
LL |             pub fn rust_task_is_unwinding(rt: *const rust_task) -> bool;
   |             ------------------------------------------------------------ `rust_task_is_unwinding` previously declared here
...
LL |             pub fn rust_task_is_unwinding(rt: *const rust_task) -> bool;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
note: the lint level is defined here
  --> /checkout/src/test/ui/issues/issue-1866.rs:4:9
   |
LL | #![warn(clashing_extern_decl)]
LL | #![warn(clashing_extern_decl)]
   |         ^^^^^^^^^^^^^^^^^^^^
   = note: expected `unsafe extern "C" fn(*const usize) -> bool`
              found `unsafe extern "C" fn(*const bool) -> bool`
warning: 1 warning emitted


------------------------------------------
---

14    |
15 LL | #![warn(clashing_extern_decl)]
16    |         ^^^^^^^^^^^^^^^^^^^^
-    = note: expected  `unsafe extern "C" fn(i32) -> *const u8`
-               found  `unsafe extern "C" fn(i32, i32) -> *const u8`
+    = note: expected `unsafe extern "C" fn(i32) -> *const u8`
+               found `unsafe extern "C" fn(i32, i32) -> *const u8`
20 warning: 1 warning emitted
21 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5791/issue-5791.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-5791.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-5791.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5791/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5791/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `malloc2` redeclares `malloc` with a different signature
   |
   |
LL | /     #[link_name = "malloc"]
LL | |     fn malloc1(len: i32) -> *const u8;
   | |______________________________________- `malloc` previously declared here
LL | /     #[link_name = "malloc"]
LL | |     //~^ WARN `malloc2` redeclares `malloc` with a different signature
LL | |     fn malloc2(len: i32, foo: i32) -> *const u8;
   | |________________________________________________^ this signature doesn't match the previous declaration
note: the lint level is defined here
  --> /checkout/src/test/ui/issues/issue-5791.rs:3:9
   |
LL | #![warn(clashing_extern_decl)]
LL | #![warn(clashing_extern_decl)]
   |         ^^^^^^^^^^^^^^^^^^^^
   = note: expected `unsafe extern "C" fn(i32) -> *const u8`
              found `unsafe extern "C" fn(i32, i32) -> *const u8`
warning: 1 warning emitted


------------------------------------------
---

12    |
13 LL | #![warn(clashing_extern_decl)]
14    |         ^^^^^^^^^^^^^^^^^^^^
-    = note: expected  `unsafe extern "C" fn(u8)`
-               found  `unsafe extern "C" fn(u64)`
+    = note: expected `unsafe extern "C" fn(u8)`
+               found `unsafe extern "C" fn(u64)`
18 warning: `extern_fn` redeclared with a different signature
19   --> $DIR/clashing-extern-fn.rs:39:9


24 LL |         fn extern_fn(x: u32);
25    |         ^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
26    |
-    = note: expected  `unsafe extern "C" fn(u64)`
-               found  `unsafe extern "C" fn(u32)`
+    = note: expected `unsafe extern "C" fn(u64)`
+               found `unsafe extern "C" fn(u32)`
30 warning: `extern_link_name` redeclared with a different signature
31   --> $DIR/clashing-extern-fn.rs:64:9


37 LL |           fn extern_link_name(x: u32);
38    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
39    |
-    = note: expected  `unsafe extern "C" fn(i16)`
-               found  `unsafe extern "C" fn(u32)`
+    = note: expected `unsafe extern "C" fn(i16)`
+               found `unsafe extern "C" fn(u32)`
42 
43 warning: `some_other_extern_link_name` redeclares `some_other_new_name` with a different signature
44   --> $DIR/clashing-extern-fn.rs:67:9

51 LL | |         fn some_other_extern_link_name(x: u32);
52    | |_______________________________________________^ this signature doesn't match the previous declaration
53    |
-    = note: expected  `unsafe extern "C" fn(i16)`
-               found  `unsafe extern "C" fn(u32)`
+    = note: expected `unsafe extern "C" fn(i16)`
+               found `unsafe extern "C" fn(u32)`
56 
57 warning: `other_both_names_different` redeclares `link_name_same` with a different signature
58   --> $DIR/clashing-extern-fn.rs:71:9

66 LL | |         fn other_both_names_different(x: u32);
67    | |______________________________________________^ this signature doesn't match the previous declaration
68    |
-    = note: expected  `unsafe extern "C" fn(i16)`
-               found  `unsafe extern "C" fn(u32)`
+    = note: expected `unsafe extern "C" fn(i16)`
+               found `unsafe extern "C" fn(u32)`
71 
72 warning: `different_mod` redeclared with a different signature
73   --> $DIR/clashing-extern-fn.rs:84:9
78 LL |         fn different_mod(x: u64);
79    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
80    |
80    |
-    = note: expected  `unsafe extern "C" fn(u8)`
-               found  `unsafe extern "C" fn(u64)`
+    = note: expected `unsafe extern "C" fn(u8)`
+               found `unsafe extern "C" fn(u64)`
83 
84 warning: `variadic_decl` redeclared with a different signature
85   --> $DIR/clashing-extern-fn.rs:94:9

90 LL |         fn variadic_decl(x: u8);
91    |         ^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
92    |
-    = note: expected  `unsafe extern "C" fn(u8, ...)`
-               found  `unsafe extern "C" fn(u8)`
+    = note: expected `unsafe extern "C" fn(u8, ...)`
+               found `unsafe extern "C" fn(u8)`
95 
96 warning: `weigh_banana` redeclared with a different signature
97   --> $DIR/clashing-extern-fn.rs:137:22

102 LL |         extern "C" { fn weigh_banana(count: *const Banana) -> u64; }
103    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
104    |
-    = note: expected  `unsafe extern "C" fn(*const banana::one::Banana) -> u64`
-               found  `unsafe extern "C" fn(*const banana::three::Banana) -> u64`
+    = note: expected `unsafe extern "C" fn(*const banana::one::Banana) -> u64`
+               found `unsafe extern "C" fn(*const banana::three::Banana) -> u64`
108 warning: `draw_point` redeclared with a different signature
109   --> $DIR/clashing-extern-fn.rs:157:22


114 LL |         extern "C" { fn draw_point(p: Point); }
115    |                      ^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
116    |
-    = note: expected  `unsafe extern "C" fn(sameish_members::a::Point)`
-               found  `unsafe extern "C" fn(sameish_members::b::Point)`
+    = note: expected `unsafe extern "C" fn(sameish_members::a::Point)`
+               found `unsafe extern "C" fn(sameish_members::b::Point)`
120 warning: 9 warnings emitted
121 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/clashing-extern-fn/clashing-extern-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/clashing-extern-fn.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/clashing-extern-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/clashing-extern-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/clashing-extern-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `clash` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:15:9
   |
LL |     fn clash(x: u8);
   |     ---------------- `clash` previously declared here
...
LL |         fn clash(x: u64); //~ WARN `clash` redeclared with a different signature
   |         ^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:4:9
   |
LL | #![warn(clashing_extern_decl)]
LL | #![warn(clashing_extern_decl)]
   |         ^^^^^^^^^^^^^^^^^^^^
   = note: expected `unsafe extern "C" fn(u8)`
              found `unsafe extern "C" fn(u64)`

warning: `extern_fn` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:39:9
   |
LL |     fn extern_fn(x: u64);
   |     --------------------- `extern_fn` previously declared here
...
LL |         fn extern_fn(x: u32); //~ WARN `extern_fn` redeclared with a different signature
   |         ^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(u64)`
              found `unsafe extern "C" fn(u32)`

warning: `extern_link_name` redeclared with a different signature
warning: `extern_link_name` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:64:9
   |
LL | /     #[link_name = "extern_link_name"]
LL | |     fn some_new_name(x: i16);
   | |_____________________________- `extern_link_name` previously declared here
...
LL |           fn extern_link_name(x: u32);
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(i16)`
              found `unsafe extern "C" fn(u32)`


warning: `some_other_extern_link_name` redeclares `some_other_new_name` with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:67:9
   |
LL |       fn some_other_new_name(x: i16);
   |       ------------------------------- `some_other_new_name` previously declared here
...
LL | /         #[link_name = "some_other_new_name"]
LL | |         //~^ WARN `some_other_extern_link_name` redeclares `some_other_new_name` with a different
LL | |         fn some_other_extern_link_name(x: u32);
   | |_______________________________________________^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(i16)`
              found `unsafe extern "C" fn(u32)`


warning: `other_both_names_different` redeclares `link_name_same` with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:71:9
   |
LL | /     #[link_name = "link_name_same"]
LL | |     fn both_names_different(x: i16);
   | |____________________________________- `link_name_same` previously declared here
...
LL | /         #[link_name = "link_name_same"]
LL | |         //~^ WARN `other_both_names_different` redeclares `link_name_same` with a different
LL | |         fn other_both_names_different(x: u32);
   | |______________________________________________^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(i16)`
              found `unsafe extern "C" fn(u32)`


warning: `different_mod` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:84:9
LL |         fn different_mod(x: u8);
LL |         fn different_mod(x: u8);
   |         ------------------------ `different_mod` previously declared here
...
LL |         fn different_mod(x: u64); //~ WARN `different_mod` redeclared with a different signature
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(u8)`
              found `unsafe extern "C" fn(u64)`


warning: `variadic_decl` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:94:9
   |
LL |     fn variadic_decl(x: u8, ...);
   |     ----------------------------- `variadic_decl` previously declared here
...
LL |         fn variadic_decl(x: u8); //~ WARN `variadic_decl` redeclared with a different signature
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(u8, ...)`
              found `unsafe extern "C" fn(u8)`

warning: `weigh_banana` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:137:22
   |
LL |         extern "C" { fn weigh_banana(count: *const Banana) -> u64; }
   |                      --------------------------------------------- `weigh_banana` previously declared here
...
LL |         extern "C" { fn weigh_banana(count: *const Banana) -> u64; }
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(*const banana::one::Banana) -> u64`
              found `unsafe extern "C" fn(*const banana::three::Banana) -> u64`
warning: `draw_point` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:157:22
   |
   |
LL |         extern "C" { fn draw_point(p: Point); }
   |                      ------------------------ `draw_point` previously declared here
...
LL |         extern "C" { fn draw_point(p: Point); } //~ WARN `draw_point` redeclared with a different
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(sameish_members::a::Point)`
              found `unsafe extern "C" fn(sameish_members::b::Point)`
warning: 9 warnings emitted


------------------------------------------
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:07:35
Build completed unsuccessfully in 1:07:35
== clock drift check ==
  local time: Fri May 29 10:01:35 UTC 2020
  network time: Fri, 29 May 2020 10:01:35 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72729/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72729/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3828) (python)
##[section]Finishing: Finalize Job
