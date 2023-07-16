
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/02072b482a8b5357f7fb5e5637444ae30e423c40\compiler\rustc_hir\src\definitions.rs:452:14
stack backtrace:
   0:     0x7ffd2ba19c3f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4e35ab114ce29e2e
   1:     0x7ffd2ba44aea - core::fmt::write::h83a1b8a9a204aefd
   2:     0x7ffd2ba0b538 - <std::io::IoSlice as core::fmt::Debug>::fmt::h1ddade45cf8208fa
   3:     0x7ffd2ba1d556 - std::panicking::take_hook::h2b301730edd12368
   4:     0x7ffd2ba1cf35 - std::panicking::take_hook::h2b301730edd12368
   5:     0x7ffd2bffd5de - <rustc_ast_lowering[d6169e6a755f4cc5]::item::ItemLowerer as rustc_ast[a9e976c478ba2cc6]::visit::Visitor>::visit_attribute
   6:     0x7ffd2ba1de69 - std::panicking::rust_panic_with_hook::hcafb622216ee7b39
   7:     0x7ffd2ba1d8df - rust_begin_unwind
   8:     0x7ffd2ba1a587 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4e35ab114ce29e2e
   9:     0x7ffd2ba1d869 - rust_begin_unwind
  10:     0x7ffd2ba78f20 - core::panicking::panic_fmt::h642c5216f3cb43f3
  11:     0x7ffd2ba78e6c - core::panicking::panic::h4fc2634ac0de5286
  12:     0x7ffd3045391a - <rustc_middle[fc5481cb4acc12cb]::ty::context::TyCtxt>::def_path_hash_to_def_id
  13:     0x7ffd3054714e - <rustc_query_system[7858547ed395856b]::dep_graph::dep_node::DepNode<rustc_middle[fc5481cb4acc12cb]::dep_graph::dep_node::DepKind> as rustc_middle[fc5481cb4acc12cb]::dep_graph::dep_node::DepNodeExt>::extract_def_id
  14:     0x7ffd2f7bdd0e - rustc_query_impl[e09ea31778da6781]::query_callbacks::hir_owner
  15:     0x7ffd30438613 - <rustc_middle[fc5481cb4acc12cb]::ty::context::TyCtxt as rustc_query_system[7858547ed395856b]::dep_graph::DepContext>::try_force_from_dep_node
  16:     0x7ffd2f7f40fb - rustc_query_impl[e09ea31778da6781]::query_callbacks::diagnostic_hir_wf_check
  17:     0x7ffd2f7f40d7 - rustc_query_impl[e09ea31778da6781]::query_callbacks::diagnostic_hir_wf_check
  18:     0x7ffd2f7f40d7 - rustc_query_impl[e09ea31778da6781]::query_callbacks::diagnostic_hir_wf_check
  19:     0x7ffd2f7f40d7 - rustc_query_impl[e09ea31778da6781]::query_callbacks::diagnostic_hir_wf_check
  20:     0x7ffd2f7f40d7 - rustc_query_impl[e09ea31778da6781]::query_callbacks::diagnostic_hir_wf_check
  21:     0x7ffd2f7f40d7 - rustc_query_impl[e09ea31778da6781]::query_callbacks::diagnostic_hir_wf_check
  22:     0x7ffd2f7cc40d - rustc_query_impl[e09ea31778da6781]::query_callbacks::diagnostic_hir_wf_check
  23:     0x7ffd2f4f876c - <rustc_mir_dataflow[29483ab09f03f9e6]::framework::engine::RustcMirAttrs>::output_path
  24:     0x7ffd2f64b552 - <rustc_mir_dataflow[29483ab09f03f9e6]::framework::engine::RustcMirAttrs>::output_path
  25:     0x7ffd2c1f1a01 - <rustc_interface[f7d5b064990b81b2]::passes::boxed_resolver::BoxedResolver>::to_resolver_outputs
  26:     0x7ffd2c147af6 - rustc_interface[f7d5b064990b81b2]::passes::analysis
  27:     0x7ffd2f75282b - <rustc_span[778bd905e75c315a]::def_id::DefIndex as rustc_query_impl[e09ea31778da6781]::profiling_support::SpecIntoSelfProfilingString>::spec_to_self_profile_string
  28:     0x7ffd2f827a6e - rustc_query_impl[e09ea31778da6781]::query_callbacks::diagnostic_hir_wf_check
  29:     0x7ffd2f6f6561 - <rustc_query_impl[e09ea31778da6781]::queries::diagnostic_hir_wf_check as rustc_query_system[7858547ed395856b]::query::config::QueryDescription<rustc_query_impl[e09ea31778da6781]::plumbing::QueryCtxt>>::describe
  30:     0x7ffd2f5f0591 - <rustc_mir_dataflow[29483ab09f03f9e6]::framework::engine::RustcMirAttrs>::output_path
  31:     0x7ffd2f726d22 - <rustc_query_impl[e09ea31778da6781]::Queries as rustc_middle[fc5481cb4acc12cb]::ty::query::QueryEngine>::try_mark_green
  32:     0x7ffd2c06d62a - <rustc_driver[12b05176f1464f4f]::args::Error as core[16a571ebe10d9bcf]::fmt::Debug>::fmt
  33:     0x7ffd2c02362c - <rustc_middle[fc5481cb4acc12cb]::ty::SymbolName as core[16a571ebe10d9bcf]::fmt::Display>::fmt
  34:     0x7ffd2c014e1b - <rustc_ast_passes[12f57c553c3231c3]::node_count::NodeCounter as rustc_ast[a9e976c478ba2cc6]::visit::Visitor>::visit_ident
  35:     0x7ffd2c041141 - <rustc_middle[fc5481cb4acc12cb]::ty::SymbolName as core[16a571ebe10d9bcf]::fmt::Display>::fmt
  36:     0x7ffd2c020643 - rustc_driver[12b05176f1464f4f]::pretty::print_after_hir_lowering
  37:     0x7ffd2c0884b8 - <rustc_driver[12b05176f1464f4f]::args::Error as core[16a571ebe10d9bcf]::fmt::Debug>::fmt
  38:     0x7ffd2ba2b8ec - std::sys::windows::thread::Thread::new::h10c3f3613eb8c5b7
  39:     0x7ffdb36a54e0 - BaseThreadInitThunk
  40:     0x7ffdb420485b - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0 (02072b482 2022-01-11) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
