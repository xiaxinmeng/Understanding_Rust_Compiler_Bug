console
thread 'rustc' panicked at 'range end index 2 out of range for slice of length 1', library/core/src/slice/index.rs:73:5
stack backtrace:
   0:     0x7f5bdfb5324d - std::backtrace_rs::backtrace::libunwind::trace::h5ecf6d3c2f2dbed3
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f5bdfb5324d - std::backtrace_rs::backtrace::trace_unsynchronized::h1f34ec78f43478d1
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f5bdfb5324d - std::sys_common::backtrace::_print_fmt::h5eb133ec83ff86d3
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f5bdfb5324d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc8814d47922b5f5b
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f5bdfbaefec - core::fmt::write::h871772ca8ace1475
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/core/src/fmt/mod.rs:1196:17
   5:     0x7f5bdfb449e1 - std::io::Write::write_fmt::he514622e09de3df5
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/io/mod.rs:1654:15
   6:     0x7f5bdfb55f35 - std::sys_common::backtrace::_print::hd629e2d6c07b6e79
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f5bdfb55f35 - std::sys_common::backtrace::print::h6bddafb613b6308d
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f5bdfb55f35 - std::panicking::default_hook::{{closure}}::hf8d6097c4866d273
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/panicking.rs:295:22
   9:     0x7f5bdfb55c56 - std::panicking::default_hook::h44331fd67f31aee4
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/panicking.rs:314:9
  10:     0x7f5be032c6b1 - rustc_driver[7b89ea061d8aedc7]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f5bdfb5660a - std::panicking::rust_panic_with_hook::h51126dbc27bfe5a6
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/panicking.rs:702:17
  12:     0x7f5bdfb56447 - std::panicking::begin_panic_handler::{{closure}}::ha0ed88016ebdcd1e
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/panicking.rs:588:13
  13:     0x7f5bdfb53704 - std::sys_common::backtrace::__rust_end_short_backtrace::h05a84a7ce4d2e529
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f5bdfb56179 - rust_begin_unwind
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/panicking.rs:584:5
  15:     0x7f5bdfb1b443 - core::panicking::panic_fmt::hc7f457b7c7e09a46
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/core/src/panicking.rs:142:14
  16:     0x7f5bdfbb1816 - core::slice::index::slice_end_index_len_fail_rt::h04ca4b0b6ddf070e
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/core/src/slice/index.rs:73:5
  17:     0x7f5bdfba3707 - core::ops::function::FnOnce::call_once::hb4cbec441e0e0d97
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/core/src/ops/function.rs:248:5
  18:     0x7f5bdfbaa516 - core::intrinsics::const_eval_select::h2fab5eaa67d7a905
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/core/src/intrinsics.rs:2375:5
  19:     0x7f5bdfb1b556 - core::slice::index::slice_end_index_len_fail::ha1557d304be1c61e
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/core/src/slice/index.rs:67:9
  20:     0x7f5be129e20f - <rustc_infer[9b42245fee805938]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[5dbb21eaf9590c92]::intravisit::Visitor>::visit_expr
  21:     0x7f5be129cedc - <rustc_infer[9b42245fee805938]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[5dbb21eaf9590c92]::intravisit::Visitor>::visit_expr
  22:     0x7f5be12afe37 - rustc_hir[5dbb21eaf9590c92]::intravisit::walk_block::<rustc_infer[9b42245fee805938]::infer::error_reporting::need_type_info::FindInferSourceVisitor>
  23:     0x7f5be129ceee - <rustc_infer[9b42245fee805938]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[5dbb21eaf9590c92]::intravisit::Visitor>::visit_expr
  24:     0x7f5be1255714 - <rustc_infer[9b42245fee805938]::infer::InferCtxt>::emit_inference_failure_err
  25:     0x7f5be11a72bb - <rustc_infer[9b42245fee805938]::infer::InferCtxt as rustc_trait_selection[af2ed82902f8af73]::traits::error_reporting::InferCtxtPrivExt>::maybe_report_ambiguity
  26:     0x7f5be119af87 - <rustc_infer[9b42245fee805938]::infer::InferCtxt as rustc_trait_selection[af2ed82902f8af73]::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
  27:     0x7f5be18f3cb4 - <rustc_infer[9b42245fee805938]::infer::InferCtxtBuilder>::enter::<&rustc_middle[c4bb946b42085c78]::ty::context::TypeckResults, <rustc_typeck[4b1a1a7c06e44333]::check::inherited::InheritedBuilder>::enter<rustc_typeck[4b1a1a7c06e44333]::check::typeck_with_fallback<rustc_typeck[4b1a1a7c06e44333]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[c4bb946b42085c78]::ty::context::TypeckResults>::{closure#0}>
  28:     0x7f5be188570a - rustc_typeck[4b1a1a7c06e44333]::check::typeck
  29:     0x7f5be29835f0 - <rustc_query_system[a1cbf340ec6d6f93]::dep_graph::graph::DepGraph<rustc_middle[c4bb946b42085c78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[c4bb946b42085c78]::ty::context::TyCtxt, rustc_span[c774ced8c61a5d44]::def_id::LocalDefId, &rustc_middle[c4bb946b42085c78]::ty::context::TypeckResults>
  30:     0x7f5be1eac380 - rustc_query_system[a1cbf340ec6d6f93]::query::plumbing::try_execute_query::<rustc_query_impl[f03dd36fb3d62396]::plumbing::QueryCtxt, rustc_query_system[a1cbf340ec6d6f93]::query::caches::DefaultCache<rustc_span[c774ced8c61a5d44]::def_id::LocalDefId, &rustc_middle[c4bb946b42085c78]::ty::context::TypeckResults>>
  31:     0x7f5be1dd7c0e - <rustc_query_impl[f03dd36fb3d62396]::Queries as rustc_middle[c4bb946b42085c78]::ty::query::QueryEngine>::typeck
  32:     0x7f5be1945708 - <rustc_middle[c4bb946b42085c78]::hir::map::Map>::par_body_owners::<rustc_typeck[4b1a1a7c06e44333]::check::typeck_item_bodies::{closure#0}>
  33:     0x7f5be274218c - rustc_typeck[4b1a1a7c06e44333]::check::typeck_item_bodies
  34:     0x7f5be29a2823 - <rustc_query_system[a1cbf340ec6d6f93]::dep_graph::graph::DepGraph<rustc_middle[c4bb946b42085c78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[c4bb946b42085c78]::ty::context::TyCtxt, (), ()>
  35:     0x7f5be2a4d1d5 - rustc_query_system[a1cbf340ec6d6f93]::query::plumbing::try_execute_query::<rustc_query_impl[f03dd36fb3d62396]::plumbing::QueryCtxt, rustc_query_system[a1cbf340ec6d6f93]::query::caches::DefaultCache<(), ()>>
  36:     0x7f5be2a77331 - rustc_query_system[a1cbf340ec6d6f93]::query::plumbing::get_query::<rustc_query_impl[f03dd36fb3d62396]::queries::typeck_item_bodies, rustc_query_impl[f03dd36fb3d62396]::plumbing::QueryCtxt>
  37:     0x7f5be2778473 - <rustc_session[d1b76e352c1a3b5b]::session::Session>::time::<(), rustc_typeck[4b1a1a7c06e44333]::check_crate::{closure#7}>
  38:     0x7f5be276509b - rustc_typeck[4b1a1a7c06e44333]::check_crate
  39:     0x7f5be251f017 - rustc_interface[94703aee012e7483]::passes::analysis
  40:     0x7f5be299e595 - <rustc_query_system[a1cbf340ec6d6f93]::dep_graph::graph::DepGraph<rustc_middle[c4bb946b42085c78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[c4bb946b42085c78]::ty::context::TyCtxt, (), core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>>
  41:     0x7f5be2a431fd - rustc_query_system[a1cbf340ec6d6f93]::query::plumbing::try_execute_query::<rustc_query_impl[f03dd36fb3d62396]::plumbing::QueryCtxt, rustc_query_system[a1cbf340ec6d6f93]::query::caches::DefaultCache<(), core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>>>
  42:     0x7f5be2a898ee - rustc_query_system[a1cbf340ec6d6f93]::query::plumbing::get_query::<rustc_query_impl[f03dd36fb3d62396]::queries::analysis, rustc_query_impl[f03dd36fb3d62396]::plumbing::QueryCtxt>
  43:     0x7f5be24dc2a7 - <rustc_interface[94703aee012e7483]::passes::QueryContext>::enter::<rustc_driver[7b89ea061d8aedc7]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>>
  44:     0x7f5be24c634f - <rustc_interface[94703aee012e7483]::interface::Compiler>::enter::<rustc_driver[7b89ea061d8aedc7]::run_compiler::{closure#1}::{closure#2}, core[c8091fc9dea0c6cf]::result::Result<core[c8091fc9dea0c6cf]::option::Option<rustc_interface[94703aee012e7483]::queries::Linker>, rustc_errors[ae611370a546922c]::ErrorGuaranteed>>
  45:     0x7f5be24efd3f - rustc_span[c774ced8c61a5d44]::with_source_map::<core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>, rustc_interface[94703aee012e7483]::interface::create_compiler_and_run<core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>, rustc_driver[7b89ea061d8aedc7]::run_compiler::{closure#1}>::{closure#1}>
  46:     0x7f5be24c71e2 - <scoped_tls[a35f89285f520bf3]::ScopedKey<rustc_span[c774ced8c61a5d44]::SessionGlobals>>::set::<rustc_interface[94703aee012e7483]::interface::run_compiler<core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>, rustc_driver[7b89ea061d8aedc7]::run_compiler::{closure#1}>::{closure#0}, core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>>
  47:     0x7f5be24dc93f - std[f19dd7bb03296d5c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[94703aee012e7483]::util::run_in_thread_pool_with_globals<rustc_interface[94703aee012e7483]::interface::run_compiler<core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>, rustc_driver[7b89ea061d8aedc7]::run_compiler::{closure#1}>::{closure#0}, core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>>::{closure#0}, core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>>
  48:     0x7f5be24dca99 - <<std[f19dd7bb03296d5c]::thread::Builder>::spawn_unchecked_<rustc_interface[94703aee012e7483]::util::run_in_thread_pool_with_globals<rustc_interface[94703aee012e7483]::interface::run_compiler<core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>, rustc_driver[7b89ea061d8aedc7]::run_compiler::{closure#1}>::{closure#0}, core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>>::{closure#0}, core[c8091fc9dea0c6cf]::result::Result<(), rustc_errors[ae611370a546922c]::ErrorGuaranteed>>::{closure#1} as core[c8091fc9dea0c6cf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7f5bdfb60533 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb3d14b8461bee867
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/alloc/src/boxed.rs:1951:9
  50:     0x7f5bdfb60533 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h528c1f598bc32c8b
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/alloc/src/boxed.rs:1951:9
  51:     0x7f5bdfb60533 - std::sys::unix::thread::Thread::new::thread_start::h9efddab2b9d9a6d4
                               at /rustc/a6b8c6954829669a5c4fa320c3e6132edf04fcfc/library/std/src/sys/unix/thread.rs:108:17
  52:     0x7f5bdfa5d609 - start_thread
                               at /build/glibc-SzIz7B/glibc-2.31/nptl/pthread_create.c:477:8
  53:     0x7f5bdf980133 - clone
                               at /build/glibc-SzIz7B/glibc-2.31/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  54:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (a6b8c6954 2022-06-03) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
