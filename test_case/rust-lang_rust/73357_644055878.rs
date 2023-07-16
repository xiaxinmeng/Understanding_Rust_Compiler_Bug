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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/34773936-5de2-47ac-b335-8711b084688e.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73357/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73357/merge:refs/remotes/pull/73357/merge
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
.................................................................................................... 1900/10311
.................................................................................................... 2000/10311
...........i..i..................................................................................... 2100/10311
.................................................................................................... 2200/10311
.iiiii.............................................................................................. 2300/10311
.................................................................................................... 2500/10311
.................................................................................................... 2600/10311
.................................................................................................... 2700/10311
.................................................................................................... 2800/10311
---
.................................i...............i.................................................. 5200/10311
.................................................................................................... 5300/10311
.................................................................................i.................. 5400/10311
...........................................................................i........................ 5500/10311
..............................................................................................ii.ii. 5600/10311
.......i...i........................................................................................ 5700/10311
................................................i................................................... 5900/10311
.................................................................................................... 6000/10311
..ii.....................................i.......................................................... 6100/10311
.................................................................................................... 6200/10311
.................................................................................................... 6200/10311
.................................................................................................... 6300/10311
.................................................................ii...i..ii...........i............. 6400/10311
.................................................................................................... 6600/10311
.................................................................................................... 6700/10311
..................................................................................................i. 6800/10311
.ii................................................................................................. 6900/10311
---
.................................................................................................... 8200/10311
.................................................................................................... 8300/10311
...............................................................................................i.... 8400/10311
.................................................................................................... 8500/10311
.................................................iiiiii.iiiiii.i.................................... 8600/10311
......i............................................................................................. 8800/10311
.................................................................................................... 8900/10311
.................................................................................................... 9000/10311
.................................................................................................... 9100/10311
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 193 tests
iiii......i..............ii.i..........i......................i...........i..i................i....i 100/193
.............i.i.i...iii..iiiiiiiiiiiiiiiii.......................iii................ii......

 finished in 5.993
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiiiiiiiiiiii

 finished in 0.163
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 122 tests
F.FFFFFFFFFFFFF.F.....F.F.FFFF.FFFF.FFFFFFFFFFFFF..FFFFFFFFFFFFF..FF...FF.FFFFFFFFFF.FFFF.FFFFFFF.F. 100/122
failures:

---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----


error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" failed to compile: 
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib

------------------------------------------



---- [incremental] incremental/change_add_field/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----

error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" failed to compile: 
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib

------------------------------------------



---- [incremental] incremental/change_crate_dep_kind.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_dep_kind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/change_crate_dep_kind.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Cpanic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 -C panic=unwind

------------------------------------------



---- [incremental] incremental/change_crate_order/main.rs stdout ----

error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" failed to compile: 
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib

------------------------------------------



---- [incremental] incremental/change_name_of_static_in_fn.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [incremental] incremental/change_implementation_cross_crate/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_implementation_cross_crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----

error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" failed to compile: 
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib

------------------------------------------



---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----

error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" failed to compile: 
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib

------------------------------------------



---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [incremental] incremental/change_symbol_export_status.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [incremental] incremental/commandline-args.rs stdout ----

error in revision `rpass3`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=2

------------------------------------------



---- [incremental] incremental/crate_hash_reorder.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [incremental] incremental/dirty_clean.rs stdout ----

error in revision `cfail2`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/dirty_clean.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/dirty_clean.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.46.0-nightly (3acd023b9 2020-06-15) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [incremental] incremental/extern_static/issue-49153.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/extern_static/issue-49153.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/issue-49153.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/auxiliary"
------------------------------------------

------------------------------------------
---
test result: FAILED. 23 passed; 99 failed; 0 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:09:49
Build completed unsuccessfully in 1:09:49
== clock drift check ==
  local time: Mon Jun 15 10:52:42 UTC 2020
  network time: Mon, 15 Jun 2020 10:52:42 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73357/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73357/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3847) (python)
##[section]Finishing: Finalize Job
