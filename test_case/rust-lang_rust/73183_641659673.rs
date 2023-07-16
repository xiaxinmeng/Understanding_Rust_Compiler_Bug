plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 8'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bf00d5aa-669e-441f-98ea-77fa19530351.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73183/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73183/merge:refs/remotes/pull/73183/merge
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
..............................i...............i..................................................... 5200/10292
.................................................................................................... 5300/10292
..............................................................................i..................... 5400/10292
........................................................................i........................... 5500/10292
.........................................................................................ii.ii...... 5600/10292
..i...i............................................................................................. 5700/10292
........................................i........................................................... 5900/10292
..............................................................................................ii.... 6000/10292
.................................i.................................................................. 6100/10292
.................................................................................................... 6200/10292
.................................................................................................... 6200/10292
.................................................................................................... 6300/10292
........................................................ii...i..ii...........i...................... 6400/10292
.................................................................................................... 6600/10292
.................................................................................................... 6700/10292
.................................................................................................... 6700/10292
.........................................................................................i..ii...... 6800/10292
.................................................................................................... 7000/10292
.................................................................................................... 7100/10292
...........................................i........................................................ 7200/10292
.................................................................................................... 7300/10292
---
.................................................................................................... 8200/10292
.................................................................................................... 8300/10292
...................................................................................i................ 8400/10292
.................................................................................................... 8500/10292
.....................................iiiiii.iiiiii.i................................................ 8600/10292
.................................................................................................... 8800/10292
.................................................................................................... 8900/10292
.................................................................................................... 9000/10292
.................................................................................................... 9100/10292
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 193 tests
iiii......i..............ii.i..........i......................i...........i..i................i....i 100/193
.............i.i.i...iii..iiiiiiiiiiiiiiiii.......................iii................ii......

 finished in 5.467
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiiiiiiiiiiii

 finished in 0.142
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 14.548
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 0.822
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-101a02b96cd72ebe

running 0 tests

---
 Documenting alloc v0.0.0 (/checkout/src/liballoc)
error: `Ord` is both a trait and a derive macro
   --> src/liballoc/slice.rs:214:55
    |
214 |     /// For example, while [`f64`] doesn't implement [`Ord`] because `NaN != NaN`, we can use
    |
note: the lint level is defined here
   --> src/liballoc/lib.rs:72:9
    |
    |
72  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
help: to link to the trait, prefix with the item type
    |
    |
214 |     /// For example, while [`f64`] doesn't implement [`trait@Ord`] because `NaN != NaN`, we can use
help: to link to the derive macro, add an exclamation mark
    |
    |
214 |     /// For example, while [`f64`] doesn't implement [`Ord!`] because `NaN != NaN`, we can use

error: `Default` is both a trait and a derive macro
    --> src/liballoc/vec.rs:1468:21
     |
     |
1468 |     /// to use the [`Default`] trait to generate values, you can pass
     |                     ^^^^^^^^^ ambiguous link
     |
help: to link to the trait, prefix with the item type
     |
1468 |     /// to use the [`trait@Default`] trait to generate values, you can pass
help: to link to the derive macro, add an exclamation mark
     |
1468 |     /// to use the [`Default!`] trait to generate values, you can pass
     |                     ^^^^^^^^^^
     |                     ^^^^^^^^^^

error: `PartialEq` is both a trait and a derive macro
    --> src/liballoc/vec.rs:1740:10
     |
1740 |     /// [`PartialEq`] trait implementation.
     |
help: to link to the trait, prefix with the item type
     |
     |
1740 |     /// [`trait@PartialEq`] trait implementation.
help: to link to the derive macro, add an exclamation mark
     |
     |
1740 |     /// [`PartialEq!`] trait implementation.

error: aborting due to 3 previous errors

error: Could not document `alloc`.
error: Could not document `alloc`.

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name alloc src/liballoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc --generate-redirect-pages -Z unstable-options --resource-suffix 1.46.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-1abd3f6528394a8d.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-0c3e39163eae6476.rmeta` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "-Z" "unstable-options" "--resource-suffix" "1.46.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:25:49
Build completed unsuccessfully in 1:25:49
== clock drift check ==
  local time: Wed Jun 10 01:07:21 UTC 2020
  network time: Wed, 10 Jun 2020 01:07:21 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73183/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73183/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4012) (python)
##[section]Finishing: Finalize Job
