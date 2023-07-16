plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 5'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200604.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200604.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4a2787fd-3b91-4868-aefa-c660f151743f.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73293/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73293/merge:refs/remotes/pull/73293/merge
---
 ---> 29a56a071ad9
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> eb826cd6a4d7
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 9841042138f8
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 00b49f7048de
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
.................................................................................................... 1900/10308
.................................................................................................... 2000/10308
...........i..i..................................................................................... 2100/10308
.................................................................................................... 2200/10308
.iiiii.............................................................................................. 2300/10308
.................................................................................................... 2500/10308
.................................................................................................... 2600/10308
.................................................................................................... 2700/10308
.................................................................................................... 2800/10308
---
.................................i...............i.................................................. 5200/10308
.................................................................................................... 5300/10308
.................................................................................i.................. 5400/10308
...........................................................................i........................ 5500/10308
..............................................................................................ii.ii. 5600/10308
.......i...i........................................................................................ 5700/10308
................................................i................................................... 5900/10308
.................................................................................................... 6000/10308
..ii.....................................i.......................................................... 6100/10308
.................................................................................................... 6200/10308
.................................................................................................... 6200/10308
.................................................................................................... 6300/10308
.................................................................ii...i..ii...........i............. 6400/10308
.................................................................................................... 6600/10308
.................................................................................................... 6700/10308
..................................................................................................i. 6800/10308
.ii................................................................................................. 6900/10308
---
.................................................................................................... 8200/10308
.................................................................................................... 8300/10308
..................................................................................................i. 8400/10308
.................................................................................................... 8500/10308
....................................................iiiiii.iiiiii.i................................. 8600/10308
.........i.......................................................................................... 8800/10308
.................................................................................................... 8900/10308
.................................................................................................... 9000/10308
.................................................................................................... 9100/10308
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 193 tests
iiii......i..............ii.i..........i......................i...........i..i................i....i 100/193
.............i.i.i...iii..iiiiiiiiiiiiiiiii.......................iii................ii......

 finished in 5.146
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiiiiiiiiiiii

 finished in 0.138
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 13.755
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
   Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
error[E0624]: associated function `parse_item` is private
  --> src/librustc_expand/parse/tests.rs:32:52
   |
32 |     new_parser_from_source_str(sess, name, source).parse_item()

error[E0624]: associated function `parse_expr` is private
  --> src/librustc_expand/parse/tests.rs:42:58
   |
   |
42 |     with_error_checking_parse(source_str, &sess(), |p| p.parse_expr())

error[E0624]: associated function `parse_item` is private
  --> src/librustc_expand/parse/tests.rs:47:58
   |
   |
47 |     with_error_checking_parse(source_str, &sess(), |p| p.parse_item())

error[E0624]: associated function `parse_expr` is private
   --> src/librustc_expand/parse/tests.rs:271:56
    |
    |
271 |         new_parser_from_source_str(sess, name, source).parse_expr()

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0624`.
For more information about this error, try `rustc --explain E0624`.
error: could not compile `rustc_expand`.

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_expand" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:13:11
Build completed unsuccessfully in 1:13:11
== clock drift check ==
  local time: Fri Jun 12 23:35:08 UTC 2020
  network time: Fri, 12 Jun 2020 23:35:08 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73293/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73293/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3765) (python)
##[section]Finishing: Finalize Job
