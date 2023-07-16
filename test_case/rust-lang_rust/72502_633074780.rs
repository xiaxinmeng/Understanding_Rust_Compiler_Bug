plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 10'
Agent machine name: 'fv-az578'
Current agent version: '2.169.0'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/789316b9-954b-4b96-b8c9-729f6a06ee0d.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72502/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72502/merge:refs/remotes/pull/72502/merge
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
...........................................................................i........................ 1800/10218
.................................................................................................... 1900/10218
..............................................................................................i..i.. 2000/10218
.................................................................................................... 2100/10218
....................................................................................iiiii........... 2200/10218
.................................................................................................... 2400/10218
.................................................................................................... 2500/10218
.................................................................................................... 2600/10218
.................................................................................................... 2700/10218
---
..........i...............i......................................................................... 5200/10218
.................................................................................................... 5300/10218
.........................................................i.......................................... 5400/10218
..................................................i................................................. 5500/10218
............................................................ii.ii........i...i...................... 5600/10218
..i................................................................................................. 5800/10218
..........i......................................................................................... 5900/10218
..............................................................ii.................................... 6000/10218
.i.................................................................................................. 6100/10218
.i.................................................................................................. 6100/10218
.................................................................................................... 6200/10218
.................................................................................................... 6300/10218
.......................ii...i..ii...........i....................................................... 6400/10218
.................................................................................................... 6600/10218
.................................................................................................... 6700/10218
.................................................................................................... 6700/10218
........................................................i..ii....................................... 6800/10218
.................................................................................................... 7000/10218
.................................................................................................... 7100/10218
..........i......................................................................................... 7200/10218
.................................................................................................... 7300/10218
---
.................................................................................................... 8100/10218
.................................................................................................... 8200/10218
.................................................................................................... 8300/10218
...................................i................................................................ 8400/10218
.........................................................................................iiiiii.iiii 8500/10218
ii.i................................................................................................ 8600/10218
.................................................................................................... 8800/10218
.................................................................................................... 8900/10218
.................................................................................................... 9000/10218
.................................................................................................... 9100/10218
---
.................................................F.F................................................ 100/105
.....
failures:

---- [mir-opt] mir-opt/generator-drop-cleanup.rs stdout ----
9     let mut _5: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:12:9: 12:14
10     let mut _7: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:18: 10:18
11     let mut _8: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
-     let mut _9: isize;                   // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+     let mut _9: u32;                     // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
13     scope 1 {
14         debug _s => (((*_1) as variant#3).0: std::string::String); // in scope 1 at $DIR/generator-drop-cleanup.rs:11:13: 11:15


thread '[mir-opt] mir-opt/generator-drop-cleanup.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator-drop-cleanup/rustc.main-{{closure}}.generator_drop.0.mir', src/tools/compiletest/src/runtest.rs:3166:25

---- [mir-opt] mir-opt/generator-tiny.rs stdout ----
12     let _8: ();                          // in scope 0 at $DIR/generator-tiny.rs:23:13: 23:21
13     let mut _9: ();                      // in scope 0 at $DIR/generator-tiny.rs:19:25: 19:25
13     let mut _9: ();                      // in scope 0 at $DIR/generator-tiny.rs:19:25: 19:25
14     let _10: u8;                         // in scope 0 at $DIR/generator-tiny.rs:19:17: 19:19
-     let mut _11: isize;                  // in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
+     let mut _11: u32;                    // in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
16     scope 1 {
17         debug _d => (((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 25:6 {u8, HasDrop, ()}])) as variant#3).0: HasDrop); // in scope 1 at $DIR/generator-tiny.rs:20:13: 20:15


thread '[mir-opt] mir-opt/generator-tiny.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator-tiny/rustc.main-{{closure}}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3166:25

failures:
    [mir-opt] mir-opt/generator-drop-cleanup.rs
    [mir-opt] mir-opt/generator-tiny.rs
    [mir-opt] mir-opt/generator-tiny.rs

test result: FAILED. 103 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:04:26
Build completed unsuccessfully in 1:04:26
== clock drift check ==
  local time: Sat May 23 15:20:12 UTC 2020
  network time: Sat, 23 May 2020 15:20:12 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72502/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72502/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3899) (python)
##[section]Finishing: Finalize Job
