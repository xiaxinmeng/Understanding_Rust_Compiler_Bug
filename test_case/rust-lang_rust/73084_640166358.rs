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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c17ac236-d3b3-4865-94eb-2bca04d5cf74.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73084/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73084/merge:refs/remotes/pull/73084/merge
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
............................i...............i....................................................... 5200/10289
.................................................................................................... 5300/10289
............................................................................i....................... 5400/10289
......................................................................i............................. 5500/10289
.......................................................................................ii.ii........ 5600/10289
i...i............................................................................................... 5700/10289
......................................i............................................................. 5900/10289
............................................................................................ii...... 6000/10289
...............................i.................................................................... 6100/10289
.................................................................................................... 6200/10289
.................................................................................................... 6200/10289
.................................................................................................... 6300/10289
......................................................ii...i..ii...........i........................ 6400/10289
.................................................................................................... 6600/10289
.................................................................................................... 6700/10289
.................................................................................................... 6700/10289
.......................................................................................i..ii........ 6800/10289
.................................................................................................... 7000/10289
.................................................................................................... 7100/10289
.........................................i.......................................................... 7200/10289
.................................................................................................... 7300/10289
---
.................................................................................................... 8200/10289
.................................................................................................... 8300/10289
....................................................................................i............... 8400/10289
.................................................................................................... 8500/10289
......................................iiiiii.iiiiii.i............................................... 8600/10289
.................................................................................................... 8800/10289
.................................................................................................... 8900/10289
.................................................................................................... 9000/10289
.................................................................................................... 9100/10289
---
diff of stdout:

13     },
14 ]
15 PRINT-ATTR INPUT (DISPLAY): const A: u8 = 0;
- PRINT-ATTR RE-COLLECTED (DISPLAY): const A : u8 = 0 ;
+ PRINT-ATTR RE-COLLECTED (DISPLAY): const  A  : u8 = 0 ;
17 PRINT-ATTR INPUT (DEBUG): TokenStream [
19         ident: "const",


-         span: #0 bytes(0..0),
+         span: #3 bytes(315..320),
-     Ident {
-         ident: "A",
-         ident: "A",
-         span: #0 bytes(0..0),
+     Group {
+         delimiter: None,
+         stream: TokenStream [
+                 ident: "A",
+                 ident: "A",
+                 span: #0 bytes(402..403),
+         ],
+         ],
+         span: #3 bytes(321..323),
26     Punct {
26     Punct {
27         ch: ':',
28         spacing: Alone,
28         spacing: Alone,
-         span: #0 bytes(0..0),
+         span: #3 bytes(323..324),
31     Ident {
32         ident: "u8",


-         span: #0 bytes(0..0),
+         span: #3 bytes(325..327),
35     Punct {
35     Punct {
36         ch: '=',
37         spacing: Alone,
37         spacing: Alone,
-         span: #0 bytes(0..0),
+         span: #3 bytes(328..329),
40     Literal {
41         kind: Integer,

42         symbol: "0",
42         symbol: "0",
43         suffix: None,
-         span: #0 bytes(0..0),
+         span: #3 bytes(330..331),
46     Punct {
46     Punct {
47         ch: ';',
48         spacing: Alone,
48         spacing: Alone,
-         span: #0 bytes(0..0),
+         span: #3 bytes(331..332),
51 ]
52 PRINT-DERIVE INPUT (DISPLAY): struct A {

53 }
53 }
- PRINT-DERIVE RE-COLLECTED (DISPLAY): struct A { }
+ PRINT-DERIVE RE-COLLECTED (DISPLAY): struct  A  { }
55 PRINT-DERIVE INPUT (DEBUG): TokenStream [
57         ident: "struct",


-         span: #0 bytes(0..0),
+         span: #3 bytes(367..373),
-     Ident {
-         ident: "A",
-         ident: "A",
-         span: #0 bytes(0..0),
+     Group {
+         delimiter: None,
+         stream: TokenStream [
+                 ident: "A",
+                 ident: "A",
+                 span: #0 bytes(402..403),
+         ],
+         ],
+         span: #3 bytes(374..376),
64     Group {
65         delimiter: Brace,


66         stream: TokenStream [],
-         span: #0 bytes(0..0),
+         span: #3 bytes(377..379),
69 ]
70 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/input-interpolated/input-interpolated.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/input-interpolated.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/input-interpolated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/input-interpolated" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/input-interpolated/auxiliary"
------------------------------------------
PRINT-BANG INPUT (DISPLAY): A
PRINT-BANG INPUT (DISPLAY): A
PRINT-BANG RE-COLLECTED (DISPLAY):  A 
PRINT-BANG INPUT (DEBUG): TokenStream [
    Group {
        delimiter: None,
        stream: TokenStream [
                ident: "A",
                ident: "A",
                span: #0 bytes(402..403),
        ],
        ],
        span: #3 bytes(269..271),
]
]
PRINT-ATTR INPUT (DISPLAY): const A: u8 = 0;
PRINT-ATTR RE-COLLECTED (DISPLAY): const  A  : u8 = 0 ;
PRINT-ATTR INPUT (DEBUG): TokenStream [
        ident: "const",
        ident: "const",
        span: #3 bytes(315..320),
    Group {
        delimiter: None,
        stream: TokenStream [
            Ident {
            Ident {
                ident: "A",
                span: #0 bytes(402..403),
        ],
        ],
        span: #3 bytes(321..323),
    Punct {
    Punct {
        ch: ':',
        spacing: Alone,
        span: #3 bytes(323..324),
    Ident {
        ident: "u8",
        ident: "u8",
        span: #3 bytes(325..327),
    Punct {
    Punct {
        ch: '=',
        spacing: Alone,
        span: #3 bytes(328..329),
    Literal {
        kind: Integer,
        symbol: "0",
        suffix: None,
        suffix: None,
        span: #3 bytes(330..331),
    Punct {
    Punct {
        ch: ';',
        spacing: Alone,
        span: #3 bytes(331..332),
]
PRINT-DERIVE INPUT (DISPLAY): struct A {
}
}
PRINT-DERIVE RE-COLLECTED (DISPLAY): struct  A  { }
PRINT-DERIVE INPUT (DEBUG): TokenStream [
        ident: "struct",
        ident: "struct",
        span: #3 bytes(367..373),
    Group {
        delimiter: None,
        stream: TokenStream [
            Ident {
            Ident {
                ident: "A",
                span: #0 bytes(402..403),
        ],
        ],
        span: #3 bytes(374..376),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [],
        span: #3 bytes(377..379),
]

------------------------------------------
stderr:
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:07:43
Build completed unsuccessfully in 1:07:43
== clock drift check ==
  local time: Sun Jun  7 06:42:13 UTC 2020
  network time: Sun, 07 Jun 2020 06:42:13 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73084/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73084/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3388) (python)
##[section]Finishing: Finalize Job
