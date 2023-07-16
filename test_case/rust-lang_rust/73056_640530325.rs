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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d4e2f1cb-67da-410a-b721-c6de47be903c.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73056/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73056/merge:refs/remotes/pull/73056/merge
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
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 10287 tests
.................................................................................................... 100/10287
.............................................iF..Fii................................................ 200/10287
.................................................................................................... 400/10287
.................................................................................................... 500/10287
................................................................................................i... 600/10287
.................................................................................................... 700/10287
---
............................i...............i....................................................... 5200/10287
.................................................................................................... 5300/10287
............................................................................i....................... 5400/10287
......................................................................i............................. 5500/10287
.......................................................................................ii.ii........ 5600/10287
i...i............................................................................................... 5700/10287
......................................i............................................................. 5900/10287
............................................................................................ii...... 6000/10287
...............................i.................................................................... 6100/10287
.................................................................................................... 6200/10287
.................................................................................................... 6200/10287
.................................................................................................... 6300/10287
......................................................ii...i..ii...........i........................ 6400/10287
.................................................................................................... 6600/10287
.................................................................................................... 6700/10287
.................................................................................................... 6700/10287
.......................................................................................i..ii........ 6800/10287
.................................................................................................... 7000/10287
.................................................................................................... 7100/10287
.........................................i.......................................................... 7200/10287
.................................................................................................... 7300/10287
---
.................................................................................................... 8200/10287
.................................................................................................... 8300/10287
................................................................................i................... 8400/10287
.................................................................................................... 8500/10287
..................................iiiiii.iiiiii.i................................................... 8600/10287
.................................................................................................... 8800/10287
.................................................................................................... 8900/10287
.................................................................................................... 9000/10287
.................................................................................................... 9100/10287
---

---- [ui] ui/asm/bad-template.rs stdout ----
diff of stderr:

70 LL |         asm!("{1}", in(reg) foo);
72    |
-    = note: `#[warn(asm_unused_argument)]` on by default
+    = note: `#[warn(unused_asm_arguments)]` on by default
74 
74 
75 warning: asm argument not used in template
76   --> $DIR/bad-template.rs:15:20


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template/bad-template.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/bad-template.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-template.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:8:15
   |
LL |         asm!("{}");
   |               ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:10:15
  --> /checkout/src/test/ui/asm/bad-template.rs:10:15
   |
LL |         asm!("{1}", in(reg) foo);
   |               ^^^ from here
   = note: there is 1 argument

error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:13:15
  --> /checkout/src/test/ui/asm/bad-template.rs:13:15
   |
LL |         asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:15:15
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |               ^^   --------------- named argument
   |               from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:15:20
   |
LL |         asm!("{}", a = in(reg) foo);

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:18:15
   |
   |
LL |         asm!("{1}", a = in(reg) foo);
   |               ^^^ from here
   = note: no positional arguments were given

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:21:15
  --> /checkout/src/test/ui/asm/bad-template.rs:21:15
   |
LL |         asm!("{}", in("eax") foo);
   |               ^^   ------------- explicit register argument
   |               from here
   |
   = note: no positional arguments were given
note: explicit register arguments cannot be used in the asm template
note: explicit register arguments cannot be used in the asm template
  --> /checkout/src/test/ui/asm/bad-template.rs:21:20
   |
LL |         asm!("{}", in("eax") foo);

error: asm template modifier must be a single character
  --> /checkout/src/test/ui/asm/bad-template.rs:23:17
   |
   |
LL |         asm!("{:foo}", in(reg) foo);

warning: asm argument not used in template
  --> /checkout/src/test/ui/asm/bad-template.rs:10:21
   |
   |
LL |         asm!("{1}", in(reg) foo);
   |
   = note: `#[warn(unused_asm_arguments)]` on by default

warning: asm argument not used in template
warning: asm argument not used in template
  --> /checkout/src/test/ui/asm/bad-template.rs:15:20
   |
LL |         asm!("{}", a = in(reg) foo);

warning: asm argument not used in template
  --> /checkout/src/test/ui/asm/bad-template.rs:18:21
   |
   |
LL |         asm!("{1}", a = in(reg) foo);

warning: asm arguments not used in template
  --> /checkout/src/test/ui/asm/bad-template.rs:25:18
   |
   |
LL |         asm!("", in(reg) 0, in(reg) 1);


error: aborting due to 7 previous errors; 4 warnings emitted

------------------------------------------



---- [ui] ui/asm/parse-error.rs stdout ----
diff of stderr:

158 LL |         asm!("{a}", a = const foo, a = const bar);
160    |
-    = note: `#[warn(asm_unused_argument)]` on by default
+    = note: `#[warn(unused_asm_arguments)]` on by default
162 
162 
163 error: aborting due to 23 previous errors; 1 warning emitted


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/parse-error/parse-error.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/parse-error/parse-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/parse-error.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/parse-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/parse-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/parse-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error: expected token: `,`
  --> /checkout/src/test/ui/asm/parse-error.rs:13:19
   |
LL |         asm!("{}" foo);
   |                   ^^^ expected `,`

error: expected one of `const`, `in`, `inlateout`, `inout`, `lateout`, `options`, `out`, or `sym`, found `foo`
   |
LL |         asm!("{}", foo);
   |                    ^^^ expected one of 8 possible tokens

---

error: expected `)`, found `foo`
  --> /checkout/src/test/ui/asm/parse-error.rs:19:27
   |
LL |         asm!("{}", in(reg foo));
   |                           ^^^ expected `)`
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/parse-error.rs:21:27
   |
   |
LL |         asm!("{}", in(reg));

error: expected register class or explicit register
  --> /checkout/src/test/ui/asm/parse-error.rs:23:26
   |
   |
LL |         asm!("{}", inout(=) foo => bar);

error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/parse-error.rs:25:37
   |
   |
LL |         asm!("{}", inout(reg) foo =>);


error: expected one of `!`, `,`, `.`, `::`, `?`, `{`, or an operator, found `=>`
   |
   |
LL |         asm!("{}", in(reg) foo => bar);
   |                                ^^ expected one of 7 possible tokens
error: argument to `sym` must be a path expression
  --> /checkout/src/test/ui/asm/parse-error.rs:29:24
   |
   |
LL |         asm!("{}", sym foo + bar);


error: expected one of `)`, `att_syntax`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, or `readonly`, found `foo`
   |
   |
LL |         asm!("", options(foo));
   |                          ^^^ expected one of 8 possible tokens

error: expected one of `)` or `,`, found `foo`
   |
   |
LL |         asm!("", options(nomem foo));
   |                                ^^^ expected one of `)` or `,`

error: expected one of `)`, `att_syntax`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, or `readonly`, found `foo`
   |
   |
LL |         asm!("", options(nomem, foo));
   |                                 ^^^ expected one of 8 possible tokens
error: asm options cannot be specified multiple times
  --> /checkout/src/test/ui/asm/parse-error.rs:37:29
   |
   |
LL |         asm!("", options(), options());
   |                  ---------  ^^^^^^^^^ duplicate options
   |                  previously here

error: asm options cannot be specified multiple times
  --> /checkout/src/test/ui/asm/parse-error.rs:39:29
  --> /checkout/src/test/ui/asm/parse-error.rs:39:29
   |
LL |         asm!("", options(), options(), options());
   |                  ---------  ^^^^^^^^^ duplicate options
   |                  previously here

error: asm options cannot be specified multiple times
  --> /checkout/src/test/ui/asm/parse-error.rs:39:40
  --> /checkout/src/test/ui/asm/parse-error.rs:39:40
   |
LL |         asm!("", options(), options(), options());
   |                  ---------             ^^^^^^^^^ duplicate options
   |                  previously here

error: arguments are not allowed after options
  --> /checkout/src/test/ui/asm/parse-error.rs:42:31
  --> /checkout/src/test/ui/asm/parse-error.rs:42:31
   |
LL |         asm!("{}", options(), const foo);
   |                    ---------  ^^^^^^^^^ argument
   |                    previous options

error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/parse-error.rs:44:36
  --> /checkout/src/test/ui/asm/parse-error.rs:44:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ duplicate argument
   |                     previously here

error: explicit register arguments cannot have names
  --> /checkout/src/test/ui/asm/parse-error.rs:47:18
  --> /checkout/src/test/ui/asm/parse-error.rs:47:18
   |
LL |         asm!("", a = in("eax") foo);

error: named arguments cannot follow explicit register arguments
  --> /checkout/src/test/ui/asm/parse-error.rs:49:36
   |
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument

error: named arguments cannot follow explicit register arguments
  --> /checkout/src/test/ui/asm/parse-error.rs:51:36
  --> /checkout/src/test/ui/asm/parse-error.rs:51:36
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument

error: positional arguments cannot follow named arguments or explicit register arguments
  --> /checkout/src/test/ui/asm/parse-error.rs:53:36
  --> /checkout/src/test/ui/asm/parse-error.rs:53:36
   |
LL |         asm!("{1}", in("eax") foo, const bar);
   |                     -------------  ^^^^^^^^^ positional argument
   |                     explicit register argument

warning: asm argument not used in template
  --> /checkout/src/test/ui/asm/parse-error.rs:44:36
  --> /checkout/src/test/ui/asm/parse-error.rs:44:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |
   = note: `#[warn(unused_asm_arguments)]` on by default


error: aborting due to 23 previous errors; 1 warning emitted

------------------------------------------


---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:02:21
Build completed unsuccessfully in 1:02:21
== clock drift check ==
  local time: Mon Jun  8 10:54:51 UTC 2020
  network time: Mon, 08 Jun 2020 10:54:51 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73056/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73056/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3486) (python)
##[section]Finishing: Finalize Job
