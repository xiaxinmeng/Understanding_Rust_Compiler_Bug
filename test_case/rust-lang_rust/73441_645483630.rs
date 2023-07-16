plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/34864ab6-6287-46fa-8434-2ee29a237d4f.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73441/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73441/merge:refs/remotes/pull/73441/merge
---
 ---> 38c164126245
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> afe0ff39f7ee
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 45ef90c713f0
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> d9e5bf90d143
---
####################################################                      72.6%
###################################################################       94.1%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2020-04-22/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
    Updating git repository `https://github.com/alexcrichton/addr2line`
    Updating git repository `https://github.com/gimli-rs/gimli`
    Updating git repository `https://github.com/Frommi/miniz_oxide`
---
   Compiling rustc-demangle v0.1.16
   Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
   Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
   Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
   Compiling gimli v0.21.0 (https://github.com/gimli-rs/gimli#ea707d78)
   Compiling miniz_oxide v0.3.7 (https://github.com/Frommi/miniz_oxide#7f5aedd7)
   Compiling object v0.20.0
   Compiling addr2line v0.12.0 (https://github.com/alexcrichton/addr2line?branch=dep-of-std#94e604b2)
   Compiling term v0.0.0 (/checkout/src/libterm)
   Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
   Compiling unicode-width v0.1.6
   Compiling getopts v0.2.21
---
   Compiling block-padding v0.1.5
   Compiling crossbeam-utils v0.7.2
   Compiling getopts v0.2.21
   Compiling rustc_lexer v0.1.0 (/checkout/src/librustc_lexer)
   Compiling miniz_oxide v0.3.7 (https://github.com/Frommi/miniz_oxide#7f5aedd7)
   Compiling petgraph v0.4.13
   Compiling rustc_version v0.2.3
   Compiling crossbeam-queue v0.1.2
   Compiling unicode-security v0.0.3
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
   Compiling rustc-demangle v0.1.16
   Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
   Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
   Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
   Compiling gimli v0.21.0 (https://github.com/gimli-rs/gimli#ea707d78)
   Compiling miniz_oxide v0.3.7 (https://github.com/Frommi/miniz_oxide#7f5aedd7)
   Compiling object v0.20.0
   Compiling addr2line v0.12.0 (https://github.com/alexcrichton/addr2line?branch=dep-of-std#94e604b2)
   Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
   Compiling term v0.0.0 (/checkout/src/libterm)
   Compiling unicode-width v0.1.6
   Compiling getopts v0.2.21
---
   Compiling block-padding v0.1.5
   Compiling crossbeam-utils v0.7.2
   Compiling getopts v0.2.21
   Compiling rustc_lexer v0.1.0 (/checkout/src/librustc_lexer)
   Compiling miniz_oxide v0.3.7 (https://github.com/Frommi/miniz_oxide#7f5aedd7)
   Compiling petgraph v0.4.13
   Compiling rustc_version v0.2.3
   Compiling crossbeam-queue v0.1.2
   Compiling unicode-security v0.0.3
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
.................................................................................................... 1900/10319
.................................................................................................... 2000/10319
...............i..i................................................................................. 2100/10319
.................................................................................................... 2200/10319
.....iiiii.......................................................................................... 2300/10319
.................................................................................................... 2500/10319
.................................................................................................... 2600/10319
.................................................................................................... 2700/10319
.................................................................................................... 2800/10319
---
.................................................................................................... 6000/10319
.......ii.....................................i..................................................... 6100/10319
.................................................................................................... 6200/10319
.................................................................................................... 6300/10319
......................................................................ii...i..ii...........i........ 6400/10319
.................................................................................................... 6600/10319
.................................................................................................... 6700/10319
.................................................................................................... 6800/10319
.................................................................................................... 6800/10319
....i..ii........................................................................................... 6900/10319
.................................................................................................... 7100/10319
...........................................................i........................................ 7200/10319
.................................................................................................... 7300/10319
.................................................................................................... 7400/10319
---
.................................................................................................... 8200/10319
.................................................................................................... 8300/10319
.................................................................................................... 8400/10319
.i.................................................................................................. 8500/10319
.......................................................iiiiii.iiiiii.i.............................. 8600/10319
............i....................................................................................... 8800/10319
.................................................................................................... 8900/10319
.................................................................................................... 9000/10319
.................................................................................................... 9100/10319
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 193 tests
iiii......i..............ii.i..........i......................i...........i..i................i....i 100/193
.............i.i.i...iii..iiiiiiiiiiiiiiiii.......................iii................ii......

 finished in 6.998
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiiiiiiiiiiii

 finished in 0.165
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii. 100/116
....iiii.....ii.

 finished in 15.879
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 0.931
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-101a02b96cd72ebe

running 0 tests

---
   Compiling itoa v0.4.4
   Compiling getrandom v0.1.14
   Compiling new_debug_unreachable v1.0.3
   Compiling smallvec v1.4.0
   Compiling gimli v0.21.0 (https://github.com/gimli-rs/gimli#ea707d78)
   Compiling ucd-trie v0.1.1
   Compiling fnv v1.0.6
   Compiling adler32 v1.1.0
   Compiling matches v0.1.8
---
   Compiling libnghttp2-sys v0.1.2
   Compiling curl-sys v0.4.25
   Compiling unicode-normalization v0.1.12
   Compiling pest v2.1.0
   Compiling miniz_oxide v0.3.7 (https://github.com/Frommi/miniz_oxide#7f5aedd7)
   Compiling futf v0.1.4
   Compiling textwrap v0.11.0
   Compiling getopts v0.2.21
   Compiling getopts v0.2.21
   Compiling addr2line v0.12.0 (https://github.com/alexcrichton/addr2line?branch=dep-of-std#94e604b2)
   Compiling heck v0.3.0
   Compiling humantime v1.3.0
   Compiling foreign-types v0.3.2
   Compiling itertools v0.8.0
---
    Checking rustc-demangle v0.1.16
    Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
    Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
    Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
    Checking gimli v0.21.0 (https://github.com/gimli-rs/gimli#ea707d78)
    Checking object v0.20.0
    Checking miniz_oxide v0.3.7 (https://github.com/Frommi/miniz_oxide#7f5aedd7)
    Checking addr2line v0.12.0 (https://github.com/alexcrichton/addr2line?branch=dep-of-std#94e604b2)
    Finished release [optimized] target(s) in 19.60s
   Compiling std v0.0.0 (/checkout/src/libstd)
 Documenting proc_macro v0.0.0 (/checkout/src/libproc_macro)
    Finished release [optimized] target(s) in 7.46s
---
Set({"/checkout/src/librustc_parse_format"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_serialize"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
Rustbook (x86_64-unknown-linux-gnu) - edition-guide
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.64s
std/primitive.u64.html:928: broken link fragment `#tymethod.from_u64` pointing to `std/primitive.u64.html`
std/primitive.u32.html:943: broken link fragment `#tymethod.from_u64` pointing to `std/primitive.u32.html`
std/primitive.usize.html:941: broken link fragment `#tymethod.from_u64` pointing to `std/primitive.usize.html`
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:43:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
---
  local time: Wed Jun 17 16:35:48 UTC 2020
  network time: Wed, 17 Jun 2020 16:35:48 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73441/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73441/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3605) (python)
##[section]Finishing: Finalize Job
