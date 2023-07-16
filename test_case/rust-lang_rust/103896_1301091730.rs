plain
Successfully built 1976875ba223
Successfully tagged rust-ci:latest
Built container sha256:1976875ba223e2478d2aef576da2a877270228df1204e0c2051de798ae335466
Uploading finished image to https://ci-caches.rust-lang.org/docker/5df1423b252af8fe6e1c1d4bfe9ecee415e5079d069fe81f116b5415ff8f3fff1d8d7f6f046a60e231d4eebdb0071aabb1a1f0159ce874f5e50913c8f8929e6e
upload failed: - to s3://rust-lang-ci-sccache2/docker/5df1423b252af8fe6e1c1d4bfe9ecee415e5079d069fe81f116b5415ff8f3fff1d8d7f6f046a60e231d4eebdb0071aabb1a1f0159ce874f5e50913c8f8929e6e Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
 finished in 0.682 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 158 tests
.FFF..FFFFFFF.FF....F..........F..FFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFF.FFFFFFFFFFFFF 88/158
FFFFFFF..FFFFFFFFFFFFFFF.FFFF..FFFFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFF

---- [incremental] src/test/incremental/callee_caller_cross_crate/b.rs stdout ----


error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" failed to compile: 
status: signal: 6 (SIGABRT)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
  --> /checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs:13:18
   |
LL | pub fn function1(x: u32) {
   |                  ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default


thread 'rustc' panicked at 'Illegal read of: 4', /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:450:25
stack backtrace:
   0:     0x7f518d5335fe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf1a5c151b0bf15cc
   1:     0x7f518d59c3d8 - core::fmt::write::h5d3ceb707eb5bd2b
   2:     0x7f518d524fc1 - std::io::Write::write_fmt::h2fdaa6da7aeef04e
   3:     0x7f518d533401 - std::sys_common::backtrace::print::h1e2c38bc6703916b
   4:     0x7f518d536794 - std::panicking::default_hook::{{closure}}::h0c11215faf86863f
   5:     0x7f518d536459 - std::panicking::default_hook::hd212652be1f8b573
   6:     0x7f518df1d914 - rustc_driver[57b008d7081ed021]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f518d536ee4 - std::panicking::rust_panic_with_hook::h873a59f9b923c205
   8:     0x7f518d536c47 - std::panicking::begin_panic_handler::{{closure}}::h4c38e620680273e0
   9:     0x7f518d533b34 - std::sys_common::backtrace::__rust_end_short_backtrace::h56e2c2bb7c058a75
  10:     0x7f518d536912 - rust_begin_unwind
  11:     0x7f518d4e8903 - core::panicking::panic_fmt::h03172d468c91bd36
  12:     0x7f518e117e2f - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::read_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  13:     0x7f518e069a15 - rustc_interface[21d0af606740888b]::callbacks::track_span_parent
  14:     0x7f518f83f663 - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::new
  15:     0x7f518f84e37d - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span as rustc_serialize[ac305cc1bfce1cc8]::serialize::Decodable<rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::CacheDecoder>>::decode
  16:     0x7f518fae1f36 - <rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  17:     0x7f518fa520e0 - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::try_load_from_disk::<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  18:     0x7f518f81a89a - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::{closure#0}, core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  19:     0x7f518fcdfe49 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  20:     0x7f518fc35cdf - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  21:     0x7f518fd59736 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::def_span, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  22:     0x7f518f8d30cc - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::def_span
  23:     0x7f51905eda85 - <rustc_lint[8e834ee9670b1c41]::builtin::MissingDoc>::check_missing_docs_attrs
  24:     0x7f51906079cd - <rustc_lint[8e834ee9670b1c41]::BuiltinCombinedLateLintPass as rustc_lint[8e834ee9670b1c41]::passes::LateLintPass>::check_crate
  25:     0x7f518e0a36e8 - rustc_lint[8e834ee9670b1c41]::late::late_lint_crate::<rustc_lint[8e834ee9670b1c41]::BuiltinCombinedLateLintPass>
  26:     0x7f518e034d98 - <rustc_session[d1a1c616c203ddee]::session::Session>::time::<(), rustc_lint[8e834ee9670b1c41]::late::check_crate<rustc_lint[8e834ee9670b1c41]::BuiltinCombinedLateLintPass, rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  27:     0x7f518e10b1e2 - rustc_data_structures[47612a4fe5c4127f]::sync::join::<rustc_lint[8e834ee9670b1c41]::late::check_crate<rustc_lint[8e834ee9670b1c41]::BuiltinCombinedLateLintPass, rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[8e834ee9670b1c41]::late::check_crate<rustc_lint[8e834ee9670b1c41]::BuiltinCombinedLateLintPass, rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  28:     0x7f518e035120 - <rustc_session[d1a1c616c203ddee]::session::Session>::time::<(), rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  29:     0x7f518e1175f5 - std[ec5cda99b532d39e]::panic::catch_unwind::<core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>, ()>
  30:     0x7f518e0db3a5 - <core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}> as core[33d90562097e449e]::ops::function::FnOnce<()>>::call_once
  31:     0x7f518e117716 - std[ec5cda99b532d39e]::panic::catch_unwind::<core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}>, ()>
  32:     0x7f518e036f06 - <rustc_session[d1a1c616c203ddee]::session::Session>::time::<(), rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}>
  33:     0x7f518e06fdfc - rustc_interface[21d0af606740888b]::passes::analysis
  34:     0x7f518f80de84 - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, (), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  35:     0x7f518fa1e870 - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, (), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  36:     0x7f518fc63584 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<(), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>>
  37:     0x7f518fd595db - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::analysis, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  38:     0x7f518f89b0da - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::analysis
  39:     0x7f518df34277 - <rustc_interface[21d0af606740888b]::passes::QueryContext>::enter::<rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  40:     0x7f518df8f1fa - <rustc_interface[21d0af606740888b]::interface::Compiler>::enter::<rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}::{closure#2}, core[33d90562097e449e]::result::Result<core[33d90562097e449e]::option::Option<rustc_interface[21d0af606740888b]::queries::Linker>, rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  41:     0x7f518df1f06e - rustc_span[cdc3c7d4b7c7a50d]::with_source_map::<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  42:     0x7f518df8655c - <scoped_tls[75ae3d54078cc351]::ScopedKey<rustc_span[cdc3c7d4b7c7a50d]::SessionGlobals>>::set::<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  43:     0x7f518df3e8c9 - std[ec5cda99b532d39e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  44:     0x7f518df8ac58 - std[ec5cda99b532d39e]::panic::catch_unwind::<core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<<std[ec5cda99b532d39e]::thread::Builder>::spawn_unchecked_<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  45:     0x7f518df2f4da - <<std[ec5cda99b532d39e]::thread::Builder>::spawn_unchecked_<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#1} as core[33d90562097e449e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:     0x7f518d5432fe - std::sys::unix::thread::Thread::new::thread_start::h95192eaad66925cf
  47:     0x7f518d2dfb43 - <unknown>
  48:     0x7f518d371a00 - <unknown>
  49:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (fad7cba3a 2022-11-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'Illegal read of: 112', /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:450:25
stack backtrace:
   0:     0x7f518d5335fe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf1a5c151b0bf15cc
   1:     0x7f518d59c3d8 - core::fmt::write::h5d3ceb707eb5bd2b
   2:     0x7f518d524fc1 - std::io::Write::write_fmt::h2fdaa6da7aeef04e
   3:     0x7f518d533401 - std::sys_common::backtrace::print::h1e2c38bc6703916b
   4:     0x7f518d536794 - std::panicking::default_hook::{{closure}}::h0c11215faf86863f
   5:     0x7f518d536459 - std::panicking::default_hook::hd212652be1f8b573
   6:     0x7f518df1d914 - rustc_driver[57b008d7081ed021]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f518d536ee4 - std::panicking::rust_panic_with_hook::h873a59f9b923c205
   8:     0x7f518d536c47 - std::panicking::begin_panic_handler::{{closure}}::h4c38e620680273e0
   9:     0x7f518d533b34 - std::sys_common::backtrace::__rust_end_short_backtrace::h56e2c2bb7c058a75
  10:     0x7f518d536912 - rust_begin_unwind
  11:     0x7f518d4e8903 - core::panicking::panic_fmt::h03172d468c91bd36
  12:     0x7f518f7fdedf - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::read_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  13:     0x7f518fa4b6c4 - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::create_query_frame::<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId>
  14:     0x7f518fb7f329 - <rustc_query_impl[e0fc262c3faa6e8d]::query_structs::def_span::{closure#0}::{closure#0} as core[33d90562097e449e]::ops::function::FnOnce<(rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId)>>::call_once
  15:     0x7f518fbc9e31 - <rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::QueryState<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  16:     0x7f518f88e334 - <rustc_query_impl[e0fc262c3faa6e8d]::Queries>::try_collect_active_jobs
  17:     0x7f518fb55359 - rustc_query_system[3c23253b2c0ae6f7]::query::job::print_query_stack::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  18:     0x7f518e0dbfa1 - rustc_interface[21d0af606740888b]::interface::try_print_query_stack
  19:     0x7f518df1e6b5 - rustc_driver[57b008d7081ed021]::report_ice
  20:     0x7f518d536ee4 - std::panicking::rust_panic_with_hook::h873a59f9b923c205
  21:     0x7f518d536c47 - std::panicking::begin_panic_handler::{{closure}}::h4c38e620680273e0
  22:     0x7f518d533b34 - std::sys_common::backtrace::__rust_end_short_backtrace::h56e2c2bb7c058a75
  23:     0x7f518d536912 - rust_begin_unwind
  24:     0x7f518d4e8903 - core::panicking::panic_fmt::h03172d468c91bd36
  25:     0x7f518e117e2f - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::read_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  26:     0x7f518e069a15 - rustc_interface[21d0af606740888b]::callbacks::track_span_parent
  27:     0x7f518f83f663 - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::new
  28:     0x7f518f84e37d - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span as rustc_serialize[ac305cc1bfce1cc8]::serialize::Decodable<rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::CacheDecoder>>::decode
  29:     0x7f518fae1f36 - <rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  30:     0x7f518fa520e0 - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::try_load_from_disk::<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  31:     0x7f518f81a89a - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::{closure#0}, core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  32:     0x7f518fcdfe49 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  33:     0x7f518fc35cdf - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  34:     0x7f518fd59736 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::def_span, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  35:     0x7f518f8d30cc - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::def_span
  36:     0x7f51905eda85 - <rustc_lint[8e834ee9670b1c41]::builtin::MissingDoc>::check_missing_docs_attrs
  37:     0x7f51906079cd - <rustc_lint[8e834ee9670b1c41]::BuiltinCombinedLateLintPass as rustc_lint[8e834ee9670b1c41]::passes::LateLintPass>::check_crate
  38:     0x7f518e0a36e8 - rustc_lint[8e834ee9670b1c41]::late::late_lint_crate::<rustc_lint[8e834ee9670b1c41]::BuiltinCombinedLateLintPass>
  39:     0x7f518e034d98 - <rustc_session[d1a1c616c203ddee]::session::Session>::time::<(), rustc_lint[8e834ee9670b1c41]::late::check_crate<rustc_lint[8e834ee9670b1c41]::BuiltinCombinedLateLintPass, rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  40:     0x7f518e10b1e2 - rustc_data_structures[47612a4fe5c4127f]::sync::join::<rustc_lint[8e834ee9670b1c41]::late::check_crate<rustc_lint[8e834ee9670b1c41]::BuiltinCombinedLateLintPass, rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[8e834ee9670b1c41]::late::check_crate<rustc_lint[8e834ee9670b1c41]::BuiltinCombinedLateLintPass, rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  41:     0x7f518e035120 - <rustc_session[d1a1c616c203ddee]::session::Session>::time::<(), rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  42:     0x7f518e1175f5 - std[ec5cda99b532d39e]::panic::catch_unwind::<core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>, ()>
  43:     0x7f518e0db3a5 - <core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}> as core[33d90562097e449e]::ops::function::FnOnce<()>>::call_once
  44:     0x7f518e117716 - std[ec5cda99b532d39e]::panic::catch_unwind::<core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}::{closure#1}>, ()>
  45:     0x7f518e036f06 - <rustc_session[d1a1c616c203ddee]::session::Session>::time::<(), rustc_interface[21d0af606740888b]::passes::analysis::{closure#5}>
  46:     0x7f518e06fdfc - rustc_interface[21d0af606740888b]::passes::analysis
  47:     0x7f518f80de84 - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, (), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  48:     0x7f518fa1e870 - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, (), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  49:     0x7f518fc63584 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<(), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>>
  50:     0x7f518fd595db - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::analysis, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  51:     0x7f518f89b0da - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::analysis
  52:     0x7f518df34277 - <rustc_interface[21d0af606740888b]::passes::QueryContext>::enter::<rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  53:     0x7f518df8f1fa - <rustc_interface[21d0af606740888b]::interface::Compiler>::enter::<rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}::{closure#2}, core[33d90562097e449e]::result::Result<core[33d90562097e449e]::option::Option<rustc_interface[21d0af606740888b]::queries::Linker>, rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  54:     0x7f518df1f06e - rustc_span[cdc3c7d4b7c7a50d]::with_source_map::<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  55:     0x7f518df8655c - <scoped_tls[75ae3d54078cc351]::ScopedKey<rustc_span[cdc3c7d4b7c7a50d]::SessionGlobals>>::set::<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  56:     0x7f518df3e8c9 - std[ec5cda99b532d39e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  57:     0x7f518df8ac58 - std[ec5cda99b532d39e]::panic::catch_unwind::<core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<<std[ec5cda99b532d39e]::thread::Builder>::spawn_unchecked_<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  58:     0x7f518df2f4da - <<std[ec5cda99b532d39e]::thread::Builder>::spawn_unchecked_<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#1} as core[33d90562097e449e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  59:     0x7f518d5432fe - std::sys::unix::thread::Thread::new::thread_start::h95192eaad66925cf
  60:     0x7f518d2dfb43 - <unknown>
  61:     0x7f518d371a00 - <unknown>
  62:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (fad7cba3a 2022-11-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [incremental] src/test/incremental/change_private_impl_method_cc/struct_point.rs stdout ----

error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" failed to compile: 
status: signal: 6 (SIGABRT)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Illegal read of: 8', /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:450:25
stack backtrace:
   0:     0x7f14cdb685fe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf1a5c151b0bf15cc
   1:     0x7f14cdbd13d8 - core::fmt::write::h5d3ceb707eb5bd2b
   2:     0x7f14cdb59fc1 - std::io::Write::write_fmt::h2fdaa6da7aeef04e
   3:     0x7f14cdb68401 - std::sys_common::backtrace::print::h1e2c38bc6703916b
   4:     0x7f14cdb6b794 - std::panicking::default_hook::{{closure}}::h0c11215faf86863f
   5:     0x7f14cdb6b459 - std::panicking::default_hook::hd212652be1f8b573
   6:     0x7f14ce552914 - rustc_driver[57b008d7081ed021]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f14cdb6bee4 - std::panicking::rust_panic_with_hook::h873a59f9b923c205
   8:     0x7f14cdb6bc47 - std::panicking::begin_panic_handler::{{closure}}::h4c38e620680273e0
   9:     0x7f14cdb68b34 - std::sys_common::backtrace::__rust_end_short_backtrace::h56e2c2bb7c058a75
  10:     0x7f14cdb6b912 - rust_begin_unwind
  11:     0x7f14cdb1d903 - core::panicking::panic_fmt::h03172d468c91bd36
  12:     0x7f14ce74ce2f - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::read_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  13:     0x7f14ce69ea15 - rustc_interface[21d0af606740888b]::callbacks::track_span_parent
  14:     0x7f14cfe74663 - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::new
  15:     0x7f14cfe8337d - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span as rustc_serialize[ac305cc1bfce1cc8]::serialize::Decodable<rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::CacheDecoder>>::decode
  16:     0x7f14d0116f36 - <rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  17:     0x7f14d00870e0 - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::try_load_from_disk::<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  18:     0x7f14cfe4f89a - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::{closure#0}, core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  19:     0x7f14d0314e49 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  20:     0x7f14d026acdf - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  21:     0x7f14d038e736 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::def_span, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  22:     0x7f14cff080cc - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::def_span
  23:     0x7f14d11dafbb - <rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt>::span_of_impl
  24:     0x7f14cee4c833 - <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::res_to_ty
  25:     0x7f14cee6764f - <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ast_ty_to_ty_inner
  26:     0x7f14cee66c8b - <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ast_ty_to_ty_inner
  27:     0x7f14cedda763 - <core[33d90562097e449e]::iter::adapters::map::Map<core[33d90562097e449e]::iter::adapters::enumerate::Enumerate<core[33d90562097e449e]::slice::iter::Iter<rustc_hir[bbd9f233ad650c6b]::hir::Ty>>, <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ty_of_fn::{closure#0}::{closure#0}> as core[33d90562097e449e]::iter::traits::iterator::Iterator>::fold::<(), core[33d90562097e449e]::iter::traits::iterator::Iterator::for_each::call<rustc_middle[ef6340a3bb70e437]::ty::Ty, <alloc[f52f70c57e28c8ae]::vec::Vec<rustc_middle[ef6340a3bb70e437]::ty::Ty> as alloc[f52f70c57e28c8ae]::vec::spec_extend::SpecExtend<rustc_middle[ef6340a3bb70e437]::ty::Ty, core[33d90562097e449e]::iter::adapters::map::Map<core[33d90562097e449e]::iter::adapters::enumerate::Enumerate<core[33d90562097e449e]::slice::iter::Iter<rustc_hir[bbd9f233ad650c6b]::hir::Ty>>, <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ty_of_fn::{closure#0}::{closure#0}>>>::spec_extend::{closure#0}>::{closure#0}>
  28:     0x7f14ceea4e4b - <alloc[f52f70c57e28c8ae]::vec::Vec<rustc_middle[ef6340a3bb70e437]::ty::Ty> as alloc[f52f70c57e28c8ae]::vec::spec_from_iter::SpecFromIter<rustc_middle[ef6340a3bb70e437]::ty::Ty, core[33d90562097e449e]::iter::adapters::map::Map<core[33d90562097e449e]::iter::adapters::enumerate::Enumerate<core[33d90562097e449e]::slice::iter::Iter<rustc_hir[bbd9f233ad650c6b]::hir::Ty>>, <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ty_of_fn::{closure#0}::{closure#0}>>>::from_iter
  29:     0x7f14cee69bf9 - <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ty_of_fn
  30:     0x7f14cedc06a9 - rustc_hir_analysis[778fd5058864ec45]::collect::infer_return_ty_for_fn_sig
  31:     0x7f14cedcedec - rustc_hir_analysis[778fd5058864ec45]::collect::fn_sig
  32:     0x7f14cfe3bda5 - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_middle[ef6340a3bb70e437]::ty::sty::Binder<rustc_middle[ef6340a3bb70e437]::ty::sty::FnSig>>::{closure#0}, rustc_middle[ef6340a3bb70e437]::ty::sty::Binder<rustc_middle[ef6340a3bb70e437]::ty::sty::FnSig>>
  33:     0x7f14d0012054 - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_middle[ef6340a3bb70e437]::ty::sty::Binder<rustc_middle[ef6340a3bb70e437]::ty::sty::FnSig>>
  34:     0x7f14d026a4ee - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_middle[ef6340a3bb70e437]::ty::sty::Binder<rustc_middle[ef6340a3bb70e437]::ty::sty::FnSig>>>
  35:     0x7f14d038df6b - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::fn_sig, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  36:     0x7f14cfef1a7c - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::fn_sig
  37:     0x7f14cedb7d8b - <rustc_hir_analysis[778fd5058864ec45]::collect::CollectItemTypesVisitor as rustc_hir[bbd9f233ad650c6b]::intravisit::Visitor>::visit_impl_item
  38:     0x7f14cecb0110 - <rustc_middle[ef6340a3bb70e437]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[778fd5058864ec45]::collect::CollectItemTypesVisitor>
  39:     0x7f14cedb54dd - rustc_hir_analysis[778fd5058864ec45]::collect::collect_mod_item_types
  40:     0x7f14cfe3bbbd - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId, ()>::{closure#0}, ()>
  41:     0x7f14d0010f0f - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId, ()>
  42:     0x7f14d02b4955 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::VecCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId, ()>>
  43:     0x7f14d036170a - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::collect_mod_item_types, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  44:     0x7f14cfef81e0 - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::collect_mod_item_types
  45:     0x7f14cecaf59a - <rustc_middle[ef6340a3bb70e437]::hir::map::Map>::for_each_module::<rustc_hir_analysis[778fd5058864ec45]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  46:     0x7f14ced466ab - <rustc_session[d1a1c616c203ddee]::session::Session>::track_errors::<rustc_hir_analysis[778fd5058864ec45]::check_crate::{closure#0}, ()>
  47:     0x7f14cee8c5fb - rustc_hir_analysis[778fd5058864ec45]::check_crate
  48:     0x7f14ce6a4d61 - rustc_interface[21d0af606740888b]::passes::analysis
  49:     0x7f14cfe42e84 - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, (), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  50:     0x7f14d0053870 - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, (), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  51:     0x7f14d0298584 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<(), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>>
  52:     0x7f14d038e5db - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::analysis, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  53:     0x7f14cfed00da - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::analysis
  54:     0x7f14ce569277 - <rustc_interface[21d0af606740888b]::passes::QueryContext>::enter::<rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  55:     0x7f14ce5c41fa - <rustc_interface[21d0af606740888b]::interface::Compiler>::enter::<rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}::{closure#2}, core[33d90562097e449e]::result::Result<core[33d90562097e449e]::option::Option<rustc_interface[21d0af606740888b]::queries::Linker>, rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  56:     0x7f14ce55406e - rustc_span[cdc3c7d4b7c7a50d]::with_source_map::<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  57:     0x7f14ce5bb55c - <scoped_tls[75ae3d54078cc351]::ScopedKey<rustc_span[cdc3c7d4b7c7a50d]::SessionGlobals>>::set::<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  58:     0x7f14ce5738c9 - std[ec5cda99b532d39e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  59:     0x7f14ce5bfc58 - std[ec5cda99b532d39e]::panic::catch_unwind::<core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<<std[ec5cda99b532d39e]::thread::Builder>::spawn_unchecked_<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  60:     0x7f14ce5644da - <<std[ec5cda99b532d39e]::thread::Builder>::spawn_unchecked_<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#1} as core[33d90562097e449e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7f14cdb782fe - std::sys::unix::thread::Thread::new::thread_start::h95192eaad66925cf
  62:     0x7f14cd914b43 - <unknown>
  63:     0x7f14cd9a6a00 - <unknown>
  64:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (fad7cba3a 2022-11-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/callbacks.rs:21:24
stack backtrace:
   0:     0x7f14cdb685fe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf1a5c151b0bf15cc
   1:     0x7f14cdbd13d8 - core::fmt::write::h5d3ceb707eb5bd2b
   2:     0x7f14cdb59fc1 - std::io::Write::write_fmt::h2fdaa6da7aeef04e
   3:     0x7f14cdb68401 - std::sys_common::backtrace::print::h1e2c38bc6703916b
   4:     0x7f14cdb6b794 - std::panicking::default_hook::{{closure}}::h0c11215faf86863f
   5:     0x7f14cdb6b459 - std::panicking::default_hook::hd212652be1f8b573
   6:     0x7f14ce552914 - rustc_driver[57b008d7081ed021]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f14cdb6bee4 - std::panicking::rust_panic_with_hook::h873a59f9b923c205
   8:     0x7f14cdb6bc47 - std::panicking::begin_panic_handler::{{closure}}::h4c38e620680273e0
   9:     0x7f14cdb68b34 - std::sys_common::backtrace::__rust_end_short_backtrace::h56e2c2bb7c058a75
  10:     0x7f14cdb6b912 - rust_begin_unwind
  11:     0x7f14cdb1d903 - core::panicking::panic_fmt::h03172d468c91bd36
  12:     0x7f14cdb1dc13 - core::result::unwrap_failed::hf8a774f71cc5ca89
  13:     0x7f14ce69ea86 - rustc_interface[21d0af606740888b]::callbacks::track_span_parent
  14:     0x7f14cfe74663 - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::new
  15:     0x7f14cfe8337d - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span as rustc_serialize[ac305cc1bfce1cc8]::serialize::Decodable<rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::CacheDecoder>>::decode
  16:     0x7f14d0116f36 - <rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  17:     0x7f14d00870e0 - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::try_load_from_disk::<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  18:     0x7f14cfe4f89a - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::{closure#0}, core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  19:     0x7f14d0314e49 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  20:     0x7f14d026acdf - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  21:     0x7f14d038e736 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::def_span, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  22:     0x7f14cff080cc - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::def_span
  23:     0x7f14cff94883 - <rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId as rustc_query_impl[e0fc262c3faa6e8d]::keys::Key>::default_span
  24:     0x7f14cff94787 - <rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId as rustc_query_impl[e0fc262c3faa6e8d]::keys::Key>::default_span
  25:     0x7f14d0080111 - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::create_query_frame::<rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId>
  26:     0x7f14d01b29d3 - <rustc_query_impl[e0fc262c3faa6e8d]::query_structs::collect_mod_item_types::{closure#0}::{closure#0} as core[33d90562097e449e]::ops::function::FnOnce<(rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId)>>::call_once
  27:     0x7f14d01fec6d - <rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::QueryState<rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  28:     0x7f14cfec3334 - <rustc_query_impl[e0fc262c3faa6e8d]::Queries>::try_collect_active_jobs
  29:     0x7f14d026b1fc - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  30:     0x7f14d038e736 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::def_span, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  31:     0x7f14cff080cc - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::def_span
  32:     0x7f14d12a40b2 - <rustc_middle[ef6340a3bb70e437]::ty::print::pretty::FmtPrinter as rustc_middle[ef6340a3bb70e437]::ty::print::Printer>::print_def_path
  33:     0x7f14d12b65f1 - <rustc_middle[ef6340a3bb70e437]::ty::print::pretty::FmtPrinter as rustc_middle[ef6340a3bb70e437]::ty::print::Printer>::default_print_def_path
  34:     0x7f14d12a4004 - <rustc_middle[ef6340a3bb70e437]::ty::print::pretty::FmtPrinter as rustc_middle[ef6340a3bb70e437]::ty::print::Printer>::print_def_path
  35:     0x7f14d11bf44e - <rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt>::def_path_str_with_substs
  36:     0x7f14d11956f7 - rustc_middle[ef6340a3bb70e437]::query::descs::fn_sig
  37:     0x7f14d0080470 - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::create_query_frame::<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId>
  38:     0x7f14d01b41e6 - <rustc_query_impl[e0fc262c3faa6e8d]::query_structs::fn_sig::{closure#0}::{closure#0} as core[33d90562097e449e]::ops::function::FnOnce<(rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId)>>::call_once
  39:     0x7f14d01fee31 - <rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::QueryState<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  40:     0x7f14cfec3334 - <rustc_query_impl[e0fc262c3faa6e8d]::Queries>::try_collect_active_jobs
  41:     0x7f14d018a359 - rustc_query_system[3c23253b2c0ae6f7]::query::job::print_query_stack::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  42:     0x7f14ce710fa1 - rustc_interface[21d0af606740888b]::interface::try_print_query_stack
  43:     0x7f14ce5536b5 - rustc_driver[57b008d7081ed021]::report_ice
  44:     0x7f14cdb6bee4 - std::panicking::rust_panic_with_hook::h873a59f9b923c205
  45:     0x7f14cdb6bc47 - std::panicking::begin_panic_handler::{{closure}}::h4c38e620680273e0
  46:     0x7f14cdb68b34 - std::sys_common::backtrace::__rust_end_short_backtrace::h56e2c2bb7c058a75
  47:     0x7f14cdb6b912 - rust_begin_unwind
  48:     0x7f14cdb1d903 - core::panicking::panic_fmt::h03172d468c91bd36
  49:     0x7f14ce74ce2f - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::read_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  50:     0x7f14ce69ea15 - rustc_interface[21d0af606740888b]::callbacks::track_span_parent
  51:     0x7f14cfe74663 - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::new
  52:     0x7f14cfe8337d - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span as rustc_serialize[ac305cc1bfce1cc8]::serialize::Decodable<rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::CacheDecoder>>::decode
  53:     0x7f14d0116f36 - <rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  54:     0x7f14d00870e0 - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::try_load_from_disk::<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  55:     0x7f14cfe4f89a - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::{closure#0}, core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  56:     0x7f14d0314e49 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>
  57:     0x7f14d026acdf - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  58:     0x7f14d038e736 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::def_span, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  59:     0x7f14cff080cc - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::def_span
  60:     0x7f14d11dafbb - <rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt>::span_of_impl
  61:     0x7f14cee4c833 - <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::res_to_ty
  62:     0x7f14cee6764f - <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ast_ty_to_ty_inner
  63:     0x7f14cee66c8b - <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ast_ty_to_ty_inner
  64:     0x7f14cedda763 - <core[33d90562097e449e]::iter::adapters::map::Map<core[33d90562097e449e]::iter::adapters::enumerate::Enumerate<core[33d90562097e449e]::slice::iter::Iter<rustc_hir[bbd9f233ad650c6b]::hir::Ty>>, <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ty_of_fn::{closure#0}::{closure#0}> as core[33d90562097e449e]::iter::traits::iterator::Iterator>::fold::<(), core[33d90562097e449e]::iter::traits::iterator::Iterator::for_each::call<rustc_middle[ef6340a3bb70e437]::ty::Ty, <alloc[f52f70c57e28c8ae]::vec::Vec<rustc_middle[ef6340a3bb70e437]::ty::Ty> as alloc[f52f70c57e28c8ae]::vec::spec_extend::SpecExtend<rustc_middle[ef6340a3bb70e437]::ty::Ty, core[33d90562097e449e]::iter::adapters::map::Map<core[33d90562097e449e]::iter::adapters::enumerate::Enumerate<core[33d90562097e449e]::slice::iter::Iter<rustc_hir[bbd9f233ad650c6b]::hir::Ty>>, <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ty_of_fn::{closure#0}::{closure#0}>>>::spec_extend::{closure#0}>::{closure#0}>
  65:     0x7f14ceea4e4b - <alloc[f52f70c57e28c8ae]::vec::Vec<rustc_middle[ef6340a3bb70e437]::ty::Ty> as alloc[f52f70c57e28c8ae]::vec::spec_from_iter::SpecFromIter<rustc_middle[ef6340a3bb70e437]::ty::Ty, core[33d90562097e449e]::iter::adapters::map::Map<core[33d90562097e449e]::iter::adapters::enumerate::Enumerate<core[33d90562097e449e]::slice::iter::Iter<rustc_hir[bbd9f233ad650c6b]::hir::Ty>>, <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ty_of_fn::{closure#0}::{closure#0}>>>::from_iter
  66:     0x7f14cee69bf9 - <dyn rustc_hir_analysis[778fd5058864ec45]::astconv::AstConv>::ty_of_fn
  67:     0x7f14cedc06a9 - rustc_hir_analysis[778fd5058864ec45]::collect::infer_return_ty_for_fn_sig
  68:     0x7f14cedcedec - rustc_hir_analysis[778fd5058864ec45]::collect::fn_sig
  69:     0x7f14cfe3bda5 - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_middle[ef6340a3bb70e437]::ty::sty::Binder<rustc_middle[ef6340a3bb70e437]::ty::sty::FnSig>>::{closure#0}, rustc_middle[ef6340a3bb70e437]::ty::sty::Binder<rustc_middle[ef6340a3bb70e437]::ty::sty::FnSig>>
  70:     0x7f14d0012054 - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_middle[ef6340a3bb70e437]::ty::sty::Binder<rustc_middle[ef6340a3bb70e437]::ty::sty::FnSig>>
  71:     0x7f14d026a4ee - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, rustc_middle[ef6340a3bb70e437]::ty::sty::Binder<rustc_middle[ef6340a3bb70e437]::ty::sty::FnSig>>>
  72:     0x7f14d038df6b - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::fn_sig, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  73:     0x7f14cfef1a7c - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::fn_sig
  74:     0x7f14cedb7d8b - <rustc_hir_analysis[778fd5058864ec45]::collect::CollectItemTypesVisitor as rustc_hir[bbd9f233ad650c6b]::intravisit::Visitor>::visit_impl_item
  75:     0x7f14cecb0110 - <rustc_middle[ef6340a3bb70e437]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[778fd5058864ec45]::collect::CollectItemTypesVisitor>
  76:     0x7f14cedb54dd - rustc_hir_analysis[778fd5058864ec45]::collect::collect_mod_item_types
  77:     0x7f14cfe3bbbd - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId, ()>::{closure#0}, ()>
  78:     0x7f14d0010f0f - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId, ()>
  79:     0x7f14d02b4955 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::VecCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId, ()>>
  80:     0x7f14d036170a - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::collect_mod_item_types, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  81:     0x7f14cfef81e0 - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::collect_mod_item_types
  82:     0x7f14cecaf59a - <rustc_middle[ef6340a3bb70e437]::hir::map::Map>::for_each_module::<rustc_hir_analysis[778fd5058864ec45]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  83:     0x7f14ced466ab - <rustc_session[d1a1c616c203ddee]::session::Session>::track_errors::<rustc_hir_analysis[778fd5058864ec45]::check_crate::{closure#0}, ()>
  84:     0x7f14cee8c5fb - rustc_hir_analysis[778fd5058864ec45]::check_crate
  85:     0x7f14ce6a4d61 - rustc_interface[21d0af606740888b]::passes::analysis
  86:     0x7f14cfe42e84 - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, (), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  87:     0x7f14d0053870 - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, (), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  88:     0x7f14d0298584 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<(), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>>
  89:     0x7f14d038e5db - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::analysis, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  90:     0x7f14cfed00da - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::analysis
  91:     0x7f14ce569277 - <rustc_interface[21d0af606740888b]::passes::QueryContext>::enter::<rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  92:     0x7f14ce5c41fa - <rustc_interface[21d0af606740888b]::interface::Compiler>::enter::<rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}::{closure#2}, core[33d90562097e449e]::result::Result<core[33d90562097e449e]::option::Option<rustc_interface[21d0af606740888b]::queries::Linker>, rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  93:     0x7f14ce55406e - rustc_span[cdc3c7d4b7c7a50d]::with_source_map::<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  94:     0x7f14ce5bb55c - <scoped_tls[75ae3d54078cc351]::ScopedKey<rustc_span[cdc3c7d4b7c7a50d]::SessionGlobals>>::set::<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  95:     0x7f14ce5738c9 - std[ec5cda99b532d39e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  96:     0x7f14ce5bfc58 - std[ec5cda99b532d39e]::panic::catch_unwind::<core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<<std[ec5cda99b532d39e]::thread::Builder>::spawn_unchecked_<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  97:     0x7f14ce5644da - <<std[ec5cda99b532d39e]::thread::Builder>::spawn_unchecked_<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#1} as core[33d90562097e449e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  98:     0x7f14cdb782fe - std::sys::unix::thread::Thread::new::thread_start::h95192eaad66925cf
  99:     0x7f14cd914b43 - <unknown>
 100:     0x7f14cd9a6a00 - <unknown>
 101:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (fad7cba3a 2022-11-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
thread panicked while processing panic. aborting.
------------------------------------------



---- [incremental] src/test/incremental/change_private_fn_cc/struct_point.rs stdout ----

error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" failed to compile: 
status: signal: 6 (SIGABRT)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Illegal read of: 7', /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:450:25
stack backtrace:
   0:     0x7ff2e9bb35fe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf1a5c151b0bf15cc
   1:     0x7ff2e9c1c3d8 - core::fmt::write::h5d3ceb707eb5bd2b
   2:     0x7ff2e9ba4fc1 - std::io::Write::write_fmt::h2fdaa6da7aeef04e
   3:     0x7ff2e9bb3401 - std::sys_common::backtrace::print::h1e2c38bc6703916b
   4:     0x7ff2e9bb6794 - std::panicking::default_hook::{{closure}}::h0c11215faf86863f
   5:     0x7ff2e9bb6459 - std::panicking::default_hook::hd212652be1f8b573
   6:     0x7ff2ea59d914 - rustc_driver[57b008d7081ed021]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff2e9bb6ee4 - std::panicking::rust_panic_with_hook::h873a59f9b923c205
   8:     0x7ff2e9bb6c47 - std::panicking::begin_panic_handler::{{closure}}::h4c38e620680273e0
   9:     0x7ff2e9bb3b34 - std::sys_common::backtrace::__rust_end_short_backtrace::h56e2c2bb7c058a75
  10:     0x7ff2e9bb6912 - rust_begin_unwind
  11:     0x7ff2e9b68903 - core::panicking::panic_fmt::h03172d468c91bd36
  12:     0x7ff2ea797e2f - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::read_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  13:     0x7ff2ea6e9a15 - rustc_interface[21d0af606740888b]::callbacks::track_span_parent
  14:     0x7ff2ebebf663 - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>::new
  15:     0x7ff2ebece37d - <rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span as rustc_serialize[ac305cc1bfce1cc8]::serialize::Decodable<rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::CacheDecoder>>::decode
  16:     0x7ff2ec23fa3b - <core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span> as rustc_serialize[ac305cc1bfce1cc8]::serialize::Decodable<rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::CacheDecoder>>::decode
  17:     0x7ff2ec15e9f0 - <rustc_query_impl[e0fc262c3faa6e8d]::on_disk_cache::OnDiskCache>::try_load_query_result::<core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  18:     0x7ff2ec0d1e00 - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::try_load_from_disk::<core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  19:     0x7ff2ebe97e3a - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>::{closure#0}, core[33d90562097e449e]::option::Option<core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>>
  20:     0x7ff2ec35a50b - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>
  21:     0x7ff2ec2a3df5 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId, core[33d90562097e449e]::option::Option<rustc_span[cdc3c7d4b7c7a50d]::span_encoding::Span>>>
  22:     0x7ff2ec38aef6 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::def_ident_span, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  23:     0x7ff2ebf5387c - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::def_ident_span
  24:     0x7ff2ed14c6f0 - <rustc_middle[ef6340a3bb70e437]::ty::FieldDef>::ident
  25:     0x7ff2eab00dab - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_kind
  26:     0x7ff2eaa9e171 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  27:     0x7ff2eaaff682 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  28:     0x7ff2eaae1684 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_overloaded_binop
  29:     0x7ff2eaae10e7 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_binop
  30:     0x7ff2eab01211 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_kind
  31:     0x7ff2eaa9e171 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  32:     0x7ff2eaaff682 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  33:     0x7ff2eaae1684 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_overloaded_binop
  34:     0x7ff2eaae10e7 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_binop
  35:     0x7ff2eab01211 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_kind
  36:     0x7ff2eaa9e171 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  37:     0x7ff2eaaff682 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  38:     0x7ff2eaa9f9dc - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_return_expr
  39:     0x7ff2eab01c04 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_kind
  40:     0x7ff2eaa9e171 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  41:     0x7ff2eaaff682 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  42:     0x7ff2eaabc78b - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_stmt
  43:     0x7ff2eaabcd77 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_block_with_expected
  44:     0x7ff2eab005eb - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_kind
  45:     0x7ff2eaa9e171 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  46:     0x7ff2eaaff682 - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  47:     0x7ff2eaa9f9dc - <rustc_hir_typeck[65c03f36466171dd]::fn_ctxt::FnCtxt>::check_return_expr
  48:     0x7ff2eac07962 - rustc_hir_typeck[65c03f36466171dd]::check::check_fn
  49:     0x7ff2eac2a0e3 - <rustc_hir_typeck[65c03f36466171dd]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[65c03f36466171dd]::typeck_with_fallback<rustc_hir_typeck[65c03f36466171dd]::typeck::{closure#0}>::{closure#1}, &rustc_middle[ef6340a3bb70e437]::ty::context::TypeckResults>
  50:     0x7ff2eaca8acc - rustc_hir_typeck[65c03f36466171dd]::typeck
  51:     0x7ff2ebe8683d - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId, &rustc_middle[ef6340a3bb70e437]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[ef6340a3bb70e437]::ty::context::TypeckResults>
  52:     0x7ff2ec059d0c - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId, &rustc_middle[ef6340a3bb70e437]::ty::context::TypeckResults>
  53:     0x7ff2ec2fc044 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::VecCache<rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId, &rustc_middle[ef6340a3bb70e437]::ty::context::TypeckResults>>
  54:     0x7ff2ec267e05 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::force_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::typeck, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  55:     0x7ff2ec0fd84d - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::force_from_dep_node::<rustc_query_impl[e0fc262c3faa6e8d]::queries::typeck>
  56:     0x7ff2ebe541bd - <rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepContext>::try_force_from_dep_node
  57:     0x7ff2ec02fd9f - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  58:     0x7ff2ebfe4f62 - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  59:     0x7ff2ec3806d4 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, (), ()>
  60:     0x7ff2ec2ee720 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<(), ()>>
  61:     0x7ff2ec3a0aab - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::typeck_item_bodies, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  62:     0x7ff2ebf440ba - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::typeck_item_bodies
  63:     0x7ff2ead8ca8b - <rustc_session[d1a1c616c203ddee]::session::Session>::time::<(), rustc_hir_analysis[778fd5058864ec45]::check_crate::{closure#7}>
  64:     0x7ff2eaed7733 - rustc_hir_analysis[778fd5058864ec45]::check_crate
  65:     0x7ff2ea6efd61 - rustc_interface[21d0af606740888b]::passes::analysis
  66:     0x7ff2ebe8de84 - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::with_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, (), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  67:     0x7ff2ec09e870 - <rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef6340a3bb70e437]::ty::context::TyCtxt, (), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  68:     0x7ff2ec2e3584 - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::try_execute_query::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_query_system[3c23253b2c0ae6f7]::query::caches::DefaultCache<(), core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>>
  69:     0x7ff2ec3d95db - rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::get_query::<rustc_query_impl[e0fc262c3faa6e8d]::queries::analysis, rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  70:     0x7ff2ebf1b0da - <rustc_query_impl[e0fc262c3faa6e8d]::Queries as rustc_middle[ef6340a3bb70e437]::ty::query::QueryEngine>::analysis
  71:     0x7ff2ea5b4277 - <rustc_interface[21d0af606740888b]::passes::QueryContext>::enter::<rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  72:     0x7ff2ea60f1fa - <rustc_interface[21d0af606740888b]::interface::Compiler>::enter::<rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}::{closure#2}, core[33d90562097e449e]::result::Result<core[33d90562097e449e]::option::Option<rustc_interface[21d0af606740888b]::queries::Linker>, rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  73:     0x7ff2ea59f06e - rustc_span[cdc3c7d4b7c7a50d]::with_source_map::<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  74:     0x7ff2ea60655c - <scoped_tls[75ae3d54078cc351]::ScopedKey<rustc_span[cdc3c7d4b7c7a50d]::SessionGlobals>>::set::<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  75:     0x7ff2ea5be8c9 - std[ec5cda99b532d39e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  76:     0x7ff2ea60ac58 - std[ec5cda99b532d39e]::panic::catch_unwind::<core[33d90562097e449e]::panic::unwind_safe::AssertUnwindSafe<<std[ec5cda99b532d39e]::thread::Builder>::spawn_unchecked_<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>
  77:     0x7ff2ea5af4da - <<std[ec5cda99b532d39e]::thread::Builder>::spawn_unchecked_<rustc_interface[21d0af606740888b]::util::run_in_thread_pool_with_globals<rustc_interface[21d0af606740888b]::interface::run_compiler<core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>, rustc_driver[57b008d7081ed021]::run_compiler::{closure#1}>::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33d90562097e449e]::result::Result<(), rustc_errors[67902e98b2bfd2e0]::ErrorGuaranteed>>::{closure#1} as core[33d90562097e449e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  78:     0x7ff2e9bc32fe - std::sys::unix::thread::Thread::new::thread_start::h95192eaad66925cf
  79:     0x7ff2e995fb43 - <unknown>
  80:     0x7ff2e99f1a00 - <unknown>
  81:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (fad7cba3a 2022-11-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'Illegal read of: 267', /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:450:25
stack backtrace:
   0:     0x7ff2e9bb35fe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf1a5c151b0bf15cc
   1:     0x7ff2e9c1c3d8 - core::fmt::write::h5d3ceb707eb5bd2b
   2:     0x7ff2e9ba4fc1 - std::io::Write::write_fmt::h2fdaa6da7aeef04e
   3:     0x7ff2e9bb3401 - std::sys_common::backtrace::print::h1e2c38bc6703916b
   4:     0x7ff2e9bb6794 - std::panicking::default_hook::{{closure}}::h0c11215faf86863f
   5:     0x7ff2e9bb6459 - std::panicking::default_hook::hd212652be1f8b573
   6:     0x7ff2ea59d914 - rustc_driver[57b008d7081ed021]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff2e9bb6ee4 - std::panicking::rust_panic_with_hook::h873a59f9b923c205
   8:     0x7ff2e9bb6c47 - std::panicking::begin_panic_handler::{{closure}}::h4c38e620680273e0
   9:     0x7ff2e9bb3b34 - std::sys_common::backtrace::__rust_end_short_backtrace::h56e2c2bb7c058a75
  10:     0x7ff2e9bb6912 - rust_begin_unwind
  11:     0x7ff2e9b68903 - core::panicking::panic_fmt::h03172d468c91bd36
  12:     0x7ff2ebe7dedf - <rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind as rustc_query_system[3c23253b2c0ae6f7]::dep_graph::DepKind>::read_deps::<<rustc_query_system[3c23253b2c0ae6f7]::dep_graph::graph::DepGraph<rustc_middle[ef6340a3bb70e437]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  13:     0x7ff2ebfdf8dc - <rustc_span[cdc3c7d4b7c7a50d]::def_id::DefId as rustc_query_impl[e0fc262c3faa6e8d]::keys::Key>::default_span
  14:     0x7ff2ebfdf787 - <rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId as rustc_query_impl[e0fc262c3faa6e8d]::keys::Key>::default_span
  15:     0x7ff2ec0cb111 - rustc_query_impl[e0fc262c3faa6e8d]::plumbing::create_query_frame::<rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId>
  16:     0x7ff2ec1ff253 - <rustc_query_impl[e0fc262c3faa6e8d]::query_structs::typeck::{closure#0}::{closure#0} as core[33d90562097e449e]::ops::function::FnOnce<(rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt, rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId)>>::call_once
  17:     0x7ff2ec249c6d - <rustc_query_system[3c23253b2c0ae6f7]::query::plumbing::QueryState<rustc_span[cdc3c7d4b7c7a50d]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
  18:     0x7ff2ebf0e334 - <rustc_query_impl[e0fc262c3faa6e8d]::Queries>::try_collect_active_jobs
  19:     0x7ff2ec1d5359 - rustc_query_system[3c23253b2c0ae6f7]::query::job::print_query_stack::<rustc_query_impl[e0fc262c3faa6e8d]::plumbing::QueryCtxt>
