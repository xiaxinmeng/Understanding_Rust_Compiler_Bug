plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 33'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1821a3e4-0ddc-465d-9523-7392df52ff09.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72707/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72707/merge:refs/remotes/pull/72707/merge
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
..............................................................................i..................... 1800/10253
.................................................................................................... 1900/10253
.................................................................................................i.. 2000/10253
i................................................................................................... 2100/10253
.......................................................................................iiiii........ 2200/10253
.................................................................................................... 2400/10253
.................................................................................................... 2500/10253
.................................................................................................... 2600/10253
.................................................................................................... 2700/10253
---
...............i...............i.................................................................... 5200/10253
.................................................................................................... 5300/10253
...............................................................i.................................... 5400/10253
........................................................i........................................... 5500/10253
.....................................................................ii.ii........i...i............. 5600/10253
............i....................................................................................... 5800/10253
....................i............................................................................... 5900/10253
.........................................................................ii......................... 6000/10253
............i....................................................................................... 6100/10253
............i....................................................................................... 6100/10253
.................................................................................................... 6200/10253
.................................................................................................... 6300/10253
...................................ii...i..ii...........i........................................... 6400/10253
.................................................................................................... 6600/10253
.................................................................................................... 6700/10253
.................................................................................................... 6700/10253
....................................................................i..ii........................... 6800/10253
.................................................................................................... 7000/10253
.................................................................................................... 7100/10253
......................i............................................................................. 7200/10253
.................................................................................................... 7300/10253
---
.................................................................................................... 8200/10253
.................................................................................................... 8300/10253
..........................................................i......................................... 8400/10253
.................................................................................................... 8500/10253
............iiiiii.iiiiii.i......................................................................... 8600/10253
.................................................................................................... 8800/10253
.................................................................................................... 8900/10253
.................................................................................................... 9000/10253
.................................................................................................... 9100/10253
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 192 tests
iiii......i..............ii.i..........i......................i...........i..i................i....i 100/192
.............i.i.i...iii..iiiiiiiiiiiiiiii.......................iii................ii......

 finished in 6.027
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 41 tests
i.........i................FFFFFF..i..FFF

---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----

error: compilation failed!
error: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/extern-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-generic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Cincremental=tmp/partitioning-tests/extern-generic" "-Zshare-generics=y" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-generic/auxiliary"
------------------------------------------
------------------------------------------
MONO_ITEM fn cgu_generic_function::bar[0]<&str> @@ cgu_generic_function.3a1fbbbh-in-extern_generic.3a1fbbbh.volatile[External]
MONO_ITEM fn cgu_generic_function::foo[0]<&str> @@ cgu_generic_function.3a1fbbbh-in-extern_generic.3a1fbbbh.volatile[External]
MONO_ITEM fn extern_generic::mod1[0]::mod1[0]::user[0] @@ extern_generic.3a1fbbbh-mod1-mod1[Internal]
MONO_ITEM fn extern_generic::mod1[0]::user[0] @@ extern_generic.3a1fbbbh-mod1[Internal]
MONO_ITEM fn extern_generic::mod2[0]::user[0] @@ extern_generic.3a1fbbbh-mod2[Internal]
MONO_ITEM fn extern_generic::mod3[0]::non_user[0] @@ extern_generic.3a1fbbbh-mod3[Internal]
MONO_ITEM fn extern_generic::user[0] @@ extern_generic.3a1fbbbh[Internal]
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 12515 but the index is 877102047', src/librustc_span/symbol.rs:1172:9
thread 'rustc' panicked at 'index out of bounds: the len is 12515 but the index is 877102047', src/librustc_span/symbol.rs:1172:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z share-generics=y -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z share-generics=y -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

------------------------------------------



---- [codegen-units] codegen-units/partitioning/incremental-merging.rs stdout ----

error: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/incremental-merging.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/incremental-merging" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/incremental-merging" "-Ccodegen-units=3" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/incremental-merging/auxiliary"
------------------------------------------
------------------------------------------
MONO_ITEM fn incremental_merging::aaa[0]::foo[0] @@ incremental_merging.3a1fbbbh-aaa--incremental_merging.3a1fbbbh-bbb[External]
MONO_ITEM fn incremental_merging::bbb[0]::foo[0] @@ incremental_merging.3a1fbbbh-aaa--incremental_merging.3a1fbbbh-bbb[External]
MONO_ITEM fn incremental_merging::ccc[0]::foo[0] @@ incremental_merging.3a1fbbbh-ccc[External]
MONO_ITEM fn incremental_merging::ddd[0]::foo[0] @@ incremental_merging.3a1fbbbh-ddd[External]
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 15 but the index is 32747', src/librustc_metadata/creader.rs:130:21
thread 'rustc' panicked at 'index out of bounds: the len is 15 but the index is 32747', src/librustc_metadata/creader.rs:130:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental -C codegen-units=3
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental -C codegen-units=3

------------------------------------------



---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----

error: compilation failed!
status: signal: 11
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/extern-drop-glue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-drop-glue" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/extern-drop-glue" "-Zinline-in-all-cgus" "-Copt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-drop-glue/auxiliary"
------------------------------------------
------------------------------------------
MONO_ITEM fn core::ptr[0]::drop_in_place[0]<cgu_extern_drop_glue::Struct[0]> @@ extern_drop_glue.3a1fbbbh-fallback.cgu[External]
MONO_ITEM fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::LocalStruct[0]> @@ extern_drop_glue.3a1fbbbh-fallback.cgu[External]
MONO_ITEM fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]> @@ extern_drop_glue.3a1fbbbh-fallback.cgu[External]
MONO_ITEM fn extern_drop_glue::mod1[0]::user[0] @@ extern_drop_glue.3a1fbbbh-mod1[External]
MONO_ITEM fn extern_drop_glue::user[0] @@ extern_drop_glue.3a1fbbbh[External]
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----

error: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-drop-glue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-drop-glue" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/local-drop-glue" "-Zinline-in-all-cgus" "-Copt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-drop-glue/auxiliary"
------------------------------------------
------------------------------------------
MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(u32, local_drop_glue::Struct[0])> @@ local_drop_glue.3a1fbbbh-fallback.cgu[Internal]
MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Outer[0]> @@ local_drop_glue.3a1fbbbh-fallback.cgu[External]
MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Struct[0]> @@ local_drop_glue.3a1fbbbh-fallback.cgu[External]
MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]> @@ local_drop_glue.3a1fbbbh-fallback.cgu[External]
MONO_ITEM fn local_drop_glue::mod1[0]::user[0] @@ local_drop_glue.3a1fbbbh-mod1[External]
MONO_ITEM fn local_drop_glue::user[0] @@ local_drop_glue.3a1fbbbh[External]
MONO_ITEM fn local_drop_glue::{{impl}}[0]::drop[0] @@ local_drop_glue.3a1fbbbh-fallback.cgu[External]
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 14715 but the index is 3343447945', src/librustc_span/symbol.rs:1172:9
thread 'rustc' panicked at 'index out of bounds: the len is 14715 but the index is 3343447945', src/librustc_span/symbol.rs:1172:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental -C opt-level=0
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental -C opt-level=0

------------------------------------------



---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----

error: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/inlining-from-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/inlining-from-extern-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/inlining-from-extern-crate" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/inlining-from-extern-crate/auxiliary"
------------------------------------------
------------------------------------------
MONO_ITEM fn cgu_explicit_inlining::always_inlined[0] @@ inlining_from_extern_crate.3a1fbbbh[Internal] inlining_from_extern_crate.3a1fbbbh-mod2[Internal]
MONO_ITEM fn cgu_explicit_inlining::inlined[0] @@ inlining_from_extern_crate.3a1fbbbh[Internal] inlining_from_extern_crate.3a1fbbbh-mod1[Internal]
MONO_ITEM fn inlining_from_extern_crate::mod1[0]::user[0] @@ inlining_from_extern_crate.3a1fbbbh-mod1[External]
MONO_ITEM fn inlining_from_extern_crate::mod2[0]::user[0] @@ inlining_from_extern_crate.3a1fbbbh-mod2[External]
MONO_ITEM fn inlining_from_extern_crate::user[0] @@ inlining_from_extern_crate.3a1fbbbh[External]
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 16 but the index is 32679', src/librustc_metadata/creader.rs:130:21
thread 'rustc' panicked at 'index out of bounds: the len is 16 but the index is 32679', src/librustc_metadata/creader.rs:130:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

------------------------------------------



---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----

error: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-generic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Cincremental=tmp/partitioning-tests/local-generic" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-generic/auxiliary"
------------------------------------------
------------------------------------------
MONO_ITEM fn local_generic::generic[0]<&str> @@ local_generic.3a1fbbbh.volatile[External]
MONO_ITEM fn local_generic::generic[0]<char> @@ local_generic.3a1fbbbh.volatile[External]
MONO_ITEM fn local_generic::generic[0]<u32> @@ local_generic.3a1fbbbh.volatile[External]
MONO_ITEM fn local_generic::generic[0]<u64> @@ local_generic.3a1fbbbh.volatile[External]
MONO_ITEM fn local_generic::mod1[0]::mod1[0]::user[0] @@ local_generic.3a1fbbbh-mod1-mod1[Internal]
MONO_ITEM fn local_generic::mod1[0]::user[0] @@ local_generic.3a1fbbbh-mod1[Internal]
MONO_ITEM fn local_generic::mod2[0]::user[0] @@ local_generic.3a1fbbbh-mod2[Internal]
MONO_ITEM fn local_generic::user[0] @@ local_generic.3a1fbbbh[Internal]
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 15 but the index is 32724', src/librustc_metadata/creader.rs:130:21
thread 'rustc' panicked at 'index out of bounds: the len is 15 but the index is 32724', src/librustc_metadata/creader.rs:130:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

------------------------------------------



---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----

error: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/statics" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/statics" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/statics/auxiliary"
------------------------------------------
------------------------------------------
MONO_ITEM fn statics::function[0] @@ statics.3a1fbbbh[External]
MONO_ITEM fn statics::mod1[0]::function[0] @@ statics.3a1fbbbh-mod1[External]
MONO_ITEM static statics::BAR[0] @@ statics.3a1fbbbh[Internal]
MONO_ITEM static statics::FOO[0] @@ statics.3a1fbbbh[Internal]
MONO_ITEM static statics::function[0]::BAR[0] @@ statics.3a1fbbbh[Internal]
MONO_ITEM static statics::function[0]::FOO[0] @@ statics.3a1fbbbh[Internal]
MONO_ITEM static statics::mod1[0]::BAR[0] @@ statics.3a1fbbbh-mod1[Internal]
MONO_ITEM static statics::mod1[0]::FOO[0] @@ statics.3a1fbbbh-mod1[Internal]
MONO_ITEM static statics::mod1[0]::function[0]::BAR[0] @@ statics.3a1fbbbh-mod1[Internal]
MONO_ITEM static statics::mod1[0]::function[0]::FOO[0] @@ statics.3a1fbbbh-mod1[Internal]
------------------------------------------
stderr:
------------------------------------------
warning: static is never used: `FOO`
---
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental
warning: 8 warnings emitted


------------------------------------------
------------------------------------------


---- [codegen-units] codegen-units/partitioning/shared-generics.rs stdout ----

error: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/shared-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/shared-generics" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zshare-generics=yes" "-Cincremental=tmp/partitioning-tests/shared-generics-exe" "-Copt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/shared-generics/auxiliary"
------------------------------------------
------------------------------------------
MONO_ITEM fn shared_generics::foo[0] @@ shared_generics.3a1fbbbh[External]
MONO_ITEM fn shared_generics_aux::generic_fn[0]<u16> @@ shared_generics_aux.3a1fbbbh-in-shared_generics.3a1fbbbh.volatile[External]
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 16 but the index is 32716', src/librustc_metadata/creader.rs:130:21
thread 'rustc' panicked at 'index out of bounds: the len is 16 but the index is 32716', src/librustc_metadata/creader.rs:130:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z share-generics=yes -C rpath -C debuginfo=0 -C incremental -C opt-level=0
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z share-generics=yes -C rpath -C debuginfo=0 -C incremental -C opt-level=0

------------------------------------------



---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----

error: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/vtable-through-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/vtable-through-const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/vtable-through-const" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/vtable-through-const/auxiliary"
------------------------------------------
------------------------------------------
MONO_ITEM fn core::ptr[0]::drop_in_place[0]<u32> @@ vtable_through_const.7rcbfp3g[Internal]
MONO_ITEM fn vtable_through_const::mod1[0]::Trait1[0]::do_something[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
MONO_ITEM fn vtable_through_const::mod1[0]::Trait1[0]::do_something_else[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
MONO_ITEM fn vtable_through_const::mod1[0]::Trait2[0]::do_something[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
MONO_ITEM fn vtable_through_const::mod1[0]::Trait2[0]::do_something_else[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
MONO_ITEM fn vtable_through_const::mod1[0]::id[0]<char> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
MONO_ITEM fn vtable_through_const::mod1[0]::id[0]<i64> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something_else[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something_else[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
MONO_ITEM fn vtable_through_const::start[0] @@ vtable_through_const.7rcbfp3g[Internal]
------------------------------------------
stderr:
------------------------------------------
warning: trait objects without an explicit `dyn` are deprecated
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:35:36
   |
35 |     pub const TRAIT1_REF: &'static Trait1 = &0u32 as &Trait1;
   |
   = note: `#[warn(bare_trait_objects)]` on by default

warning: trait objects without an explicit `dyn` are deprecated
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:35:55
   |
35 |     pub const TRAIT1_REF: &'static Trait1 = &0u32 as &Trait1;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:36:40
   |
   |
36 |     pub const TRAIT1_GEN_REF: &'static Trait1Gen<u8> = &0u32 as &Trait1Gen<u8>;
   |                                        ^^^^^^^^^^^^^ help: use `dyn`: `dyn Trait1Gen<u8>`
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:36:66
   |
   |
36 |     pub const TRAIT1_GEN_REF: &'static Trait1Gen<u8> = &0u32 as &Trait1Gen<u8>;
   |                                                                  ^^^^^^^^^^^^^ help: use `dyn`: `dyn Trait1Gen<u8>`
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:61:36
   |
   |
61 |     pub const TRAIT2_REF: &'static Trait2 = &0u32 as &Trait2;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:61:55
   |
   |
61 |     pub const TRAIT2_REF: &'static Trait2 = &0u32 as &Trait2;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:62:40
   |
   |
62 |     pub const TRAIT2_GEN_REF: &'static Trait2Gen<u8> = &0u32 as &Trait2Gen<u8>;
   |                                        ^^^^^^^^^^^^^ help: use `dyn`: `dyn Trait2Gen<u8>`
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:62:66
   |
   |
62 |     pub const TRAIT2_GEN_REF: &'static Trait2Gen<u8> = &0u32 as &Trait2Gen<u8>;
   |                                                                  ^^^^^^^^^^^^^ help: use `dyn`: `dyn Trait2Gen<u8>`
warning: constant is never used: `TRAIT2_REF`
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:61:5
   |
   |
61 |     pub const TRAIT2_REF: &'static Trait2 = &0u32 as &Trait2;
   |
   = note: `#[warn(dead_code)]` on by default

warning: constant is never used: `TRAIT2_GEN_REF`
warning: constant is never used: `TRAIT2_GEN_REF`
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:62:5
   |
62 |     pub const TRAIT2_GEN_REF: &'static Trait2Gen<u8> = &0u32 as &Trait2Gen<u8>;

warning: constant is never used: `ID_I64`
  --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:63:5
   |
   |
63 |     pub const ID_I64: fn(i64) -> i64 = id::<i64>;

thread 'rustc' panicked at 'index out of bounds: the len is 15 but the index is 32584', src/librustc_metadata/creader.rs:130:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (313bc7391 2020-05-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental
warning: 11 warnings emitted


------------------------------------------
---
test result: FAILED. 29 passed; 9 failed; 3 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:07:50
== clock drift check ==
  local time: Fri May 29 22:54:51 UTC 2020
  network time: Fri, 29 May 2020 22:54:51 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72707/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72707/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3356) (python)
##[section]Finishing: Finalize Job
