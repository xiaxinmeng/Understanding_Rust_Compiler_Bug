`
error: internal compiler error: /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_const_eval/src/interpret/eval_context.rs:373:21: expected type differs from actual type.
                                expected: dyn Trait<T>
                                actual: i32
 --> /home/matthias/vcs/github/glacier2/fixed/110683-2.rs:2:1
  |
2 | / fn foo<T>() -> dyn Trait<T>
3 | | where
4 | |     dyn Trait<T>: Sized,
  | |________________________^

thread 'rustc' panicked at 'Box<dyn Any>', /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:994:33
stack backtrace:
   0:     0x7faa3b507d30 - std::backtrace_rs::backtrace::libunwind::trace::h82a01fdfaabd5a1a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7faa3b507d30 - std::backtrace_rs::backtrace::trace_unsynchronized::hffa6767100d66420
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7faa3b507d30 - std::sys_common::backtrace::_print_fmt::h74f6c3d0a9a9ce5b
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7faa3b507d30 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9e52926a6e2127e
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7faa3b5ad7f8 - core::fmt::rt::Argument::fmt::hc4c26a7b0473f845
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/fmt/rt.rs:138:9
   5:     0x7faa3b5ad7f8 - core::fmt::write::he3860e7dd81c785d
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/fmt/mod.rs:1105:21
   6:     0x7faa3b512c6f - std::io::Write::write_fmt::h1d46a19a46248b80
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/io/mod.rs:1712:15
   7:     0x7faa3b507b35 - std::sys_common::backtrace::_print::h446dc5d7159c27aa
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x7faa3b507b35 - std::sys_common::backtrace::print::h8f7c186ef5b763ba
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x7faa3b529924 - std::panicking::default_hook::{{closure}}::h89a81a9b45f95793
  10:     0x7faa3b5295f3 - std::panicking::default_hook::h95b864b81403d0a0
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:288:9
  11:     0x7faa3dd7c545 - <alloc[32b43e9536c01a3]::boxed::Box<dyn for<'a, 'b> core[c5d5d662f7508502]::ops::function::Fn<(&'a core[c5d5d662f7508502]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[c5d5d662f7508502]::marker::Send + core[c5d5d662f7508502]::marker::Sync> as core[c5d5d662f7508502]::ops::function::Fn<(&core[c5d5d662f7508502]::panic::panic_info::PanicInfo,)>>::call
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1976:9
  12:     0x7faa3dd7c545 - rustc_driver_impl[53efafbd9bff7be4]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver_impl/src/lib.rs:1216:17
  13:     0x7faa3b52a1e2 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::he8f582d27e234b1d
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1976:9
  14:     0x7faa3b52a1e2 - std::panicking::rust_panic_with_hook::hc2808e8c83026a9a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:695:13
  15:     0x7faa3f19fd03 - std[b1ead81ad33b16ee]::panicking::begin_panic::<rustc_errors[13dec1ce28b8d9e2]::ExplicitBug>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:611:9
  16:     0x7faa3f188a66 - std[b1ead81ad33b16ee]::sys_common::backtrace::__rust_end_short_backtrace::<std[b1ead81ad33b16ee]::panicking::begin_panic<rustc_errors[13dec1ce28b8d9e2]::ExplicitBug>::{closure#0}, !>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:150:18
  17:     0x7faa3f3bee26 - std[b1ead81ad33b16ee]::panicking::begin_panic::<rustc_errors[13dec1ce28b8d9e2]::ExplicitBug>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:610:12
  18:     0x7faa3f2001c4 - std[b1ead81ad33b16ee]::panic::panic_any::<rustc_errors[13dec1ce28b8d9e2]::ExplicitBug>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:61:5
  19:     0x7faa3f2001c4 - <rustc_errors[13dec1ce28b8d9e2]::HandlerInner>::span_bug::<rustc_span[3855b6ad9a182eb9]::span_encoding::Span, &alloc[32b43e9536c01a3]::string::String>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:1581:9
  20:     0x7faa3f1ffef7 - <rustc_errors[13dec1ce28b8d9e2]::Handler>::span_bug::<rustc_span[3855b6ad9a182eb9]::span_encoding::Span, &alloc[32b43e9536c01a3]::string::String>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:994:9
  21:     0x7faa3f338d1e - rustc_middle[845fc97cf5814d51]::util::bug::opt_span_bug_fmt::<rustc_span[3855b6ad9a182eb9]::span_encoding::Span>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/util/bug.rs:34:40
  22:     0x7faa3f338d84 - rustc_middle[845fc97cf5814d51]::ty::context::tls::with_opt::<rustc_middle[845fc97cf5814d51]::util::bug::opt_span_bug_fmt<rustc_span[3855b6ad9a182eb9]::span_encoding::Span>::{closure#0}, !>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:154:36
  23:     0x7faa3f333a5e - rustc_middle[845fc97cf5814d51]::ty::context::tls::with_context_opt::<rustc_middle[845fc97cf5814d51]::ty::context::tls::with_opt<rustc_middle[845fc97cf5814d51]::util::bug::opt_span_bug_fmt<rustc_span[3855b6ad9a182eb9]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:100:18
  24:     0x7faa3f332695 - rustc_middle[845fc97cf5814d51]::ty::context::tls::with_opt::<rustc_middle[845fc97cf5814d51]::util::bug::opt_span_bug_fmt<rustc_span[3855b6ad9a182eb9]::span_encoding::Span>::{closure#0}, !>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:154:5
  25:     0x7faa3f332695 - rustc_middle[845fc97cf5814d51]::util::bug::opt_span_bug_fmt::<rustc_span[3855b6ad9a182eb9]::span_encoding::Span>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/util/bug.rs:31:5
  26:     0x7faa3f332695 - rustc_middle[845fc97cf5814d51]::util::bug::span_bug_fmt::<rustc_span[3855b6ad9a182eb9]::span_encoding::Span>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/util/bug.rs:22:5
  27:     0x7faa3f16d1f0 - rustc_const_eval[965bf4a413e5589a]::interpret::eval_context::from_known_layout::<<rustc_const_eval[965bf4a413e5589a]::interpret::eval_context::InterpCx<rustc_mir_transform[739760d2f9109a0c]::const_prop::ConstPropMachine>>::const_val_to_op::{closure#1}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_const_eval/src/interpret/eval_context.rs:373:21
  28:     0x7faa3f16d1f0 - <rustc_const_eval[965bf4a413e5589a]::interpret::eval_context::InterpCx<rustc_mir_transform[739760d2f9109a0c]::const_prop::ConstPropMachine>>::const_val_to_op
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_const_eval/src/interpret/operand.rs:643:22
  29:     0x7faa3f16eaa9 - <rustc_const_eval[965bf4a413e5589a]::interpret::eval_context::InterpCx<rustc_mir_transform[739760d2f9109a0c]::const_prop::ConstPropMachine>>::eval_mir_constant
  30:     0x7faa3f16ca73 - <rustc_const_eval[965bf4a413e5589a]::interpret::eval_context::InterpCx<rustc_mir_transform[739760d2f9109a0c]::const_prop::ConstPropMachine>>::eval_operand
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_const_eval/src/interpret/operand.rs:565:17
  31:     0x7faa3f1566f5 - <rustc_const_eval[965bf4a413e5589a]::interpret::eval_context::InterpCx<rustc_mir_transform[739760d2f9109a0c]::const_prop::ConstPropMachine>>::eval_rvalue_into_place
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_const_eval/src/interpret/step.rs:172:26
  32:     0x7faa3f40f8ba - <rustc_mir_transform[739760d2f9109a0c]::const_prop_lint::ConstPropagator as rustc_middle[845fc97cf5814d51]::mir::visit::Visitor>::visit_assign::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/const_prop_lint.rs:604:47
  33:     0x7faa3f40f8ba - <rustc_mir_transform[739760d2f9109a0c]::const_prop_lint::ConstPropagator>::use_ecx::<<rustc_mir_transform[739760d2f9109a0c]::const_prop_lint::ConstPropagator as rustc_middle[845fc97cf5814d51]::mir::visit::Visitor>::visit_assign::{closure#0}, ()>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/const_prop_lint.rs:264:15
  34:     0x7faa3f40f8ba - <rustc_mir_transform[739760d2f9109a0c]::const_prop_lint::ConstPropagator as rustc_middle[845fc97cf5814d51]::mir::visit::Visitor>::visit_assign
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/const_prop_lint.rs:603:20
  35:     0x7faa3f410fc5 - <rustc_mir_transform[739760d2f9109a0c]::const_prop_lint::ConstPropagator as rustc_middle[845fc97cf5814d51]::mir::visit::Visitor>::super_statement
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/mir/visit.rs:370:25
  36:     0x7faa3f410fc5 - <rustc_mir_transform[739760d2f9109a0c]::const_prop_lint::ConstPropagator as rustc_middle[845fc97cf5814d51]::mir::visit::Visitor>::visit_statement
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/const_prop_lint.rs:633:9
  37:     0x7faa3f4132ce - <rustc_mir_transform[739760d2f9109a0c]::const_prop_lint::ConstPropagator as rustc_middle[845fc97cf5814d51]::mir::visit::Visitor>::super_basic_block_data
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/mir/visit.rs:299:21
  38:     0x7faa3f4132ce - <rustc_mir_transform[739760d2f9109a0c]::const_prop_lint::ConstPropagator as rustc_middle[845fc97cf5814d51]::mir::visit::Visitor>::visit_basic_block_data
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/const_prop_lint.rs:702:9
  39:     0x7faa3f40e590 - <rustc_mir_transform[739760d2f9109a0c]::const_prop_lint::ConstPropagator as rustc_middle[845fc97cf5814d51]::mir::visit::Visitor>::visit_body
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/const_prop_lint.rs:579:13
  40:     0x7faa3f409c78 - <rustc_mir_transform[739760d2f9109a0c]::const_prop_lint::ConstProp as rustc_mir_transform[739760d2f9109a0c]::pass_manager::MirLint>::run_lint
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/const_prop_lint.rs:119:9
  41:     0x7faa3f3d3c15 - rustc_mir_transform[739760d2f9109a0c]::pass_manager::run_passes_inner
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/pass_manager.rs:124:13
  42:     0x7faa3f28e326 - rustc_mir_transform[739760d2f9109a0c]::pass_manager::run_passes_no_validate
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/pass_manager.rs:73:5
  43:     0x7faa3f28e326 - rustc_mir_transform[739760d2f9109a0c]::run_runtime_lowering_passes
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/lib.rs:504:5
  44:     0x7faa3f28e326 - rustc_mir_transform[739760d2f9109a0c]::run_analysis_to_runtime_passes
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/lib.rs:462:5
  45:     0x7faa3f28d8b5 - rustc_mir_transform[739760d2f9109a0c]::mir_drops_elaborated_and_const_checked
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_mir_transform/src/lib.rs:437:5
  46:     0x7faa3f952b90 - <rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked as rustc_query_system[3c70281160596a91]::query::config::QueryConfig<rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>>::compute
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:524:21
  47:     0x7faa3f952b90 - rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr::<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:464:72
  48:     0x7faa3f952b90 - rustc_middle[845fc97cf5814d51]::ty::context::tls::enter_context::<rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:82:9
  49:     0x7faa3f952b90 - <std[b1ead81ad33b16ee]::thread::local::LocalKey<core[c5d5d662f7508502]::cell::Cell<*const ()>>>::try_with::<rustc_middle[845fc97cf5814d51]::ty::context::tls::enter_context<rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/local.rs:270:16
  50:     0x7faa3f952b90 - <std[b1ead81ad33b16ee]::thread::local::LocalKey<core[c5d5d662f7508502]::cell::Cell<*const ()>>>::with::<rustc_middle[845fc97cf5814d51]::ty::context::tls::enter_context<rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/local.rs:246:9
  51:     0x7faa3f952b90 - rustc_middle[845fc97cf5814d51]::ty::context::tls::enter_context::<rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:79:9
  52:     0x7faa3f952b90 - <rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt as rustc_query_system[3c70281160596a91]::query::QueryContext>::start_query::<rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:127:13
  53:     0x7faa3f952b90 - rustc_middle[845fc97cf5814d51]::ty::context::tls::with_related_context::<<rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt as rustc_query_system[3c70281160596a91]::query::QueryContext>::start_query<rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:133:9
  54:     0x7faa3f952b90 - rustc_middle[845fc97cf5814d51]::ty::context::tls::with_context::<rustc_middle[845fc97cf5814d51]::ty::context::tls::with_related_context<<rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt as rustc_query_system[3c70281160596a91]::query::QueryContext>::start_query<rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:111:36
  55:     0x7faa3f952b90 - rustc_middle[845fc97cf5814d51]::ty::context::tls::with_context_opt::<rustc_middle[845fc97cf5814d51]::ty::context::tls::with_context<rustc_middle[845fc97cf5814d51]::ty::context::tls::with_related_context<<rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt as rustc_query_system[3c70281160596a91]::query::QueryContext>::start_query<rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:100:18
  56:     0x7faa3f952b90 - rustc_middle[845fc97cf5814d51]::ty::context::tls::with_context::<rustc_middle[845fc97cf5814d51]::ty::context::tls::with_related_context<<rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt as rustc_query_system[3c70281160596a91]::query::QueryContext>::start_query<rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:111:5
  57:     0x7faa3f952b90 - rustc_middle[845fc97cf5814d51]::ty::context::tls::with_related_context::<<rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt as rustc_query_system[3c70281160596a91]::query::QueryContext>::start_query<rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:124:5
  58:     0x7faa3f952b90 - <rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt as rustc_query_system[3c70281160596a91]::query::QueryContext>::start_query::<rustc_middle[845fc97cf5814d51]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[3c70281160596a91]::query::plumbing::execute_job_non_incr<rustc_query_impl[8836e06ed595946d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[8836e06ed595946d]::plumbing::QueryCtxt>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:112:9
