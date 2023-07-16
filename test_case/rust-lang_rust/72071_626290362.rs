plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
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
Downloading task: Bash (3.163.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4aacfec9-a0b8-4efc-88f7-ab9858207d39.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72071/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72071/merge:refs/remotes/pull/72071/merge
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
.....................................................i.............................................. 1800/10160
.................................................................................................... 1900/10160
.......................................................................i............................ 2000/10160
.................................................................................................... 2100/10160
.............................................................iiiii.................................. 2200/10160
.................................................................................................... 2400/10160
.................................................................................................... 2500/10160
.................................................................................................... 2600/10160
.................................................................................................... 2700/10160
---
.................................................................................................... 5200/10160
.................................................................................................... 5300/10160
........................i........................................................................... 5400/10160
.................i.................................................................................. 5500/10160
........................ii.ii........i...i.......................................................... 5600/10160
.........................................................................i.......................... 5800/10160
.................................................................................................... 5900/10160
....................ii.....................................i........................................ 6000/10160
.................................................................................................... 6100/10160
.................................................................................................... 6100/10160
.................................................................................................... 6200/10160
.................................................................................ii...i...ii........ 6300/10160
.................................................................................................... 6500/10160
.................................................................................................... 6600/10160
.................................................................................................... 6700/10160
.................................................................................................... 6700/10160
..............i..ii................................................................................. 6800/10160
.................................................................................................... 7000/10160
....................................................................i............................... 7100/10160
.................................................................................................... 7200/10160
.................................................................................................... 7300/10160
---
.................................................................................................... 8100/10160
.................................................................................................... 8200/10160
...................................................................................i................ 8300/10160
.................................................................................................... 8400/10160
.....................................iiiiii.iiiii.i................................................. 8500/10160
.................................................................................................... 8700/10160
.................................................................................................... 8800/10160
.................................................................................................... 8900/10160
.................................................................................................... 9000/10160
---
28 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/E0687/E0687.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args in-band-lifetimes/E0687.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/in-band-lifetimes/E0687.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/E0687" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/E0687/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0687]: lifetimes used in `fn` or `Fn` syntax must be explicitly declared using `<...>` binders
   |
   |
LL | fn foo(x: fn(&'a u32)) {} //~ ERROR must be explicitly
   |               ^^ in-band lifetime definition

error[E0687]: lifetimes used in `fn` or `Fn` syntax must be explicitly declared using `<...>` binders
   |
   |
LL | fn bar(x: &Fn(&'a u32)) {} //~ ERROR must be explicitly
   |                ^^ in-band lifetime definition

error[E0687]: lifetimes used in `fn` or `Fn` syntax must be explicitly declared using `<...>` binders
   |
   |
LL | fn baz(x: fn(&'a u32), y: &'a u32) {} //~ ERROR must be explicitly
   |               ^^ in-band lifetime definition

error[E0687]: lifetimes used in `fn` or `Fn` syntax must be explicitly declared using `<...>` binders
   |
   |
LL |     fn bar(&self, x: fn(&'a u32)) {} //~ ERROR must be explicitly
   |                          ^^ in-band lifetime definition
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0687`.

---
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/E0687_where/E0687_where.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args in-band-lifetimes/E0687_where.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/in-band-lifetimes/E0687_where.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/E0687_where" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/E0687_where/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0687]: lifetimes used in `fn` or `Fn` syntax must be explicitly declared using `<...>` binders
   |
   |
LL | fn bar<F>(x: &F) where F: Fn(&'a u32) {} //~ ERROR must be explicitly
   |                               ^^ in-band lifetime definition

error[E0687]: lifetimes used in `fn` or `Fn` syntax must be explicitly declared using `<...>` binders
   |
   |
LL | fn baz(x: &impl Fn(&'a u32)) {} //~ ERROR must be explicitly
   |                     ^^ in-band lifetime definition
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0687`.

---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:07:44
Build completed unsuccessfully in 1:07:44
== clock drift check ==
  local time: Sun May 10 08:10:42 UTC 2020
  network time: Sun, 10 May 2020 08:10:42 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72071/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72071/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4337) (python)
##[section]Finishing: Finalize Job
