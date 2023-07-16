
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/compiler/rustc_middle/src/ty/sty.rs:1000:9
stack backtrace:
   0:     0x7f745f5f4950 - std::backtrace_rs::backtrace::libunwind::trace::heeafe1f1ea6b4c2f
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f745f5f4950 - std::backtrace_rs::backtrace::trace_unsynchronized::hf08684e78cd6c167
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f745f5f4950 - std::sys_common::backtrace::_print_fmt::had9e99c2c8763a1e
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f745f5f4950 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1b4c432d2a1e6303
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f745f64df2c - core::fmt::write::h87085de871a99231
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/core/src/fmt/mod.rs:1198:17
   5:     0x7f745f5e6015 - std::io::Write::write_fmt::h7635d2f423aa55dc
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/io/mod.rs:1672:15
   6:     0x7f745f5f75e1 - std::sys_common::backtrace::_print::hc003bc1c22b7967b
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f745f5f75e1 - std::sys_common::backtrace::print::h1baa1ab7758e52b0
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f745f5f75e1 - std::panicking::default_hook::{{closure}}::he2f5e84c6ab77817
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/panicking.rs:295:22
   9:     0x7f745f5f72b3 - std::panicking::default_hook::h3f96069db270c68f
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/panicking.rs:314:9
  10:     0x7f745fe1ede4 - rustc_driver[1ce26eb46f30f4d]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f745f5f7db6 - std::panicking::rust_panic_with_hook::hed0c1597bbc695a6
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/panicking.rs:702:17
  12:     0x7f745f5f7bc9 - std::panicking::begin_panic_handler::{{closure}}::h0fc9e6b3154da131
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/panicking.rs:586:13
  13:     0x7f745f5f4e34 - std::sys_common::backtrace::__rust_end_short_backtrace::hb9c2240a67931ff9
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f745f5f7932 - rust_begin_unwind
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/panicking.rs:584:5
  15:     0x7f745f5bbc33 - core::panicking::panic_fmt::h6bda1b0556b509cd
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/core/src/panicking.rs:142:14
  16:     0x7f745f5bbafd - core::panicking::panic::h5c386340a9cd961d
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/core/src/panicking.rs:48:5
  17:     0x7f7461d2526d - rustc_trait_selection[cc111ae68c27c757]::traits::type_known_to_meet_bound_modulo_regions
  18:     0x7f7461525ff3 - <rustc_infer[25160dd56345bf64]::infer::InferCtxtBuilder>::enter::<bool, rustc_ty_utils[3b19a4d748ce0cab]::common_traits::is_item_raw::{closure#0}>
  19:     0x7f7461514c00 - rustc_ty_utils[3b19a4d748ce0cab]::common_traits::is_sized_raw
  20:     0x7f74619647f4 - <rustc_query_system[688aca7ce0d190ff]::dep_graph::graph::DepGraph<rustc_middle[d660d6fda8399c61]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d660d6fda8399c61]::ty::context::TyCtxt, rustc_middle[d660d6fda8399c61]::ty::ParamEnvAnd<rustc_middle[d660d6fda8399c61]::ty::Ty>, bool>
  21:     0x7f74619dd40e - rustc_query_system[688aca7ce0d190ff]::query::plumbing::try_execute_query::<rustc_query_impl[e840a35d18f93fd8]::plumbing::QueryCtxt, rustc_query_system[688aca7ce0d190ff]::query::caches::DefaultCache<rustc_middle[d660d6fda8399c61]::ty::ParamEnvAnd<rustc_middle[d660d6fda8399c61]::ty::Ty>, bool>>
  22:     0x7f74619d353f - <rustc_query_impl[e840a35d18f93fd8]::Queries as rustc_middle[d660d6fda8399c61]::ty::query::QueryEngine>::is_sized_raw
  23:     0x7f7461e70ff9 - <rustc_middle[d660d6fda8399c61]::ty::Ty>::is_sized
  24:     0x7f7461ea4edc - <rustc_middle[d660d6fda8399c61]::ty::layout::LayoutCx<rustc_middle[d660d6fda8399c61]::ty::context::TyCtxt>>::layout_of_uncached
  25:     0x7f7461eb2c2b - rustc_middle[d660d6fda8399c61]::ty::layout::layout_of
  26:     0x7f7461a53fae - rustc_query_system[688aca7ce0d190ff]::query::plumbing::get_query::<rustc_query_impl[e840a35d18f93fd8]::queries::layout_of, rustc_query_impl[e840a35d18f93fd8]::plumbing::QueryCtxt>
  27:     0x7f74619d3760 - <rustc_query_impl[e840a35d18f93fd8]::Queries as rustc_middle[d660d6fda8399c61]::ty::query::QueryEngine>::layout_of
  28:     0x7f7461e53e89 - <core[32c218dbf3427c26]::iter::adapters::GenericShunt<core[32c218dbf3427c26]::iter::adapters::map::Map<core[32c218dbf3427c26]::slice::iter::Iter<rustc_middle[d660d6fda8399c61]::ty::FieldDef>, <rustc_middle[d660d6fda8399c61]::ty::layout::LayoutCx<rustc_middle[d660d6fda8399c61]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[32c218dbf3427c26]::result::Result<core[32c218dbf3427c26]::convert::Infallible, rustc_middle[d660d6fda8399c61]::ty::layout::LayoutError>> as core[32c218dbf3427c26]::iter::traits::iterator::Iterator>::next
  29:     0x7f7461e9816b - <alloc[924ca5cfede8f4b5]::vec::Vec<alloc[924ca5cfede8f4b5]::vec::Vec<rustc_target[5588730283615731]::abi::TyAndLayout<rustc_middle[d660d6fda8399c61]::ty::Ty>>> as alloc[924ca5cfede8f4b5]::vec::spec_from_iter::SpecFromIter<alloc[924ca5cfede8f4b5]::vec::Vec<rustc_target[5588730283615731]::abi::TyAndLayout<rustc_middle[d660d6fda8399c61]::ty::Ty>>, core[32c218dbf3427c26]::iter::adapters::GenericShunt<core[32c218dbf3427c26]::iter::adapters::map::Map<core[32c218dbf3427c26]::slice::iter::Iter<rustc_middle[d660d6fda8399c61]::ty::VariantDef>, <rustc_middle[d660d6fda8399c61]::ty::layout::LayoutCx<rustc_middle[d660d6fda8399c61]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[32c218dbf3427c26]::result::Result<core[32c218dbf3427c26]::convert::Infallible, rustc_middle[d660d6fda8399c61]::ty::layout::LayoutError>>>>::from_iter
  30:     0x7f7461ea4ab2 - <rustc_middle[d660d6fda8399c61]::ty::layout::LayoutCx<rustc_middle[d660d6fda8399c61]::ty::context::TyCtxt>>::layout_of_uncached
  31:     0x7f7461eb2c2b - rustc_middle[d660d6fda8399c61]::ty::layout::layout_of
  32:     0x7f7461a53fae - rustc_query_system[688aca7ce0d190ff]::query::plumbing::get_query::<rustc_query_impl[e840a35d18f93fd8]::queries::layout_of, rustc_query_impl[e840a35d18f93fd8]::plumbing::QueryCtxt>
  33:     0x7f74619d3760 - <rustc_query_impl[e840a35d18f93fd8]::Queries as rustc_middle[d660d6fda8399c61]::ty::query::QueryEngine>::layout_of
  34:     0x7f7460c87f2e - <rustc_lint[29b7bb3d78c0e7f4]::context::LateContext as rustc_middle[d660d6fda8399c61]::ty::layout::LayoutOf>::spanned_layout_of
  35:     0x7f7460c888de - <rustc_lint[29b7bb3d78c0e7f4]::builtin::ClashingExternDeclarations>::structurally_same_type::structurally_same_type_impl::{closure#1}
  36:     0x7f74627ab07c - <rustc_lint[29b7bb3d78c0e7f4]::builtin::ClashingExternDeclarations>::structurally_same_type::structurally_same_type_impl
  37:     0x7f74627ab36b - <rustc_lint[29b7bb3d78c0e7f4]::builtin::ClashingExternDeclarations>::structurally_same_type::structurally_same_type_impl
  38:     0x7f74627ab31d - <rustc_lint[29b7bb3d78c0e7f4]::builtin::ClashingExternDeclarations>::structurally_same_type::structurally_same_type_impl
  39:     0x7f74627abbba - <rustc_lint[29b7bb3d78c0e7f4]::builtin::ClashingExternDeclarations as rustc_lint[29b7bb3d78c0e7f4]::passes::LateLintPass>::check_foreign_item
  40:     0x7f746203a025 - <rustc_lint[29b7bb3d78c0e7f4]::late::LateContextAndPass<rustc_lint[29b7bb3d78c0e7f4]::BuiltinCombinedLateLintPass> as rustc_hir[d9bcc83df3a1d79d]::intravisit::Visitor>::visit_nested_foreign_item
  41:     0x7f746115aa18 - rustc_hir[d9bcc83df3a1d79d]::intravisit::walk_item::<rustc_lint[29b7bb3d78c0e7f4]::late::LateContextAndPass<rustc_lint[29b7bb3d78c0e7f4]::BuiltinCombinedLateLintPass>>
  42:     0x7f74611599ea - rustc_hir[d9bcc83df3a1d79d]::intravisit::walk_mod::<rustc_lint[29b7bb3d78c0e7f4]::late::LateContextAndPass<rustc_lint[29b7bb3d78c0e7f4]::BuiltinCombinedLateLintPass>>
  43:     0x7f746115adcc - rustc_hir[d9bcc83df3a1d79d]::intravisit::walk_item::<rustc_lint[29b7bb3d78c0e7f4]::late::LateContextAndPass<rustc_lint[29b7bb3d78c0e7f4]::BuiltinCombinedLateLintPass>>
  44:     0x7f7462039a0b - <rustc_lint[29b7bb3d78c0e7f4]::late::LateContextAndPass<rustc_lint[29b7bb3d78c0e7f4]::BuiltinCombinedLateLintPass> as rustc_hir[d9bcc83df3a1d79d]::intravisit::Visitor>::visit_nested_item
  45:     0x7f746114f88f - rustc_lint[29b7bb3d78c0e7f4]::late::late_lint_crate::<rustc_lint[29b7bb3d78c0e7f4]::BuiltinCombinedLateLintPass>
  46:     0x7f746202d569 - <rustc_session[4ac326324d613686]::session::Session>::time::<(), rustc_lint[29b7bb3d78c0e7f4]::late::check_crate<rustc_lint[29b7bb3d78c0e7f4]::BuiltinCombinedLateLintPass, rustc_interface[694c7ab70a1d38bc]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  47:     0x7f746202d870 - <rustc_session[4ac326324d613686]::session::Session>::time::<(), rustc_interface[694c7ab70a1d38bc]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  48:     0x7f746201e9e1 - <core[32c218dbf3427c26]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[694c7ab70a1d38bc]::passes::analysis::{closure#5}::{closure#1}> as core[32c218dbf3427c26]::ops::function::FnOnce<()>>::call_once
  49:     0x7f746202f06a - <rustc_session[4ac326324d613686]::session::Session>::time::<(), rustc_interface[694c7ab70a1d38bc]::passes::analysis::{closure#5}>
  50:     0x7f7462019eec - rustc_interface[694c7ab70a1d38bc]::passes::analysis
  51:     0x7f74624e706c - <rustc_query_system[688aca7ce0d190ff]::dep_graph::graph::DepGraph<rustc_middle[d660d6fda8399c61]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d660d6fda8399c61]::ty::context::TyCtxt, (), core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>
  52:     0x7f7462581d1b - rustc_query_system[688aca7ce0d190ff]::query::plumbing::try_execute_query::<rustc_query_impl[e840a35d18f93fd8]::plumbing::QueryCtxt, rustc_query_system[688aca7ce0d190ff]::query::caches::DefaultCache<(), core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>>
  53:     0x7f74625df74e - rustc_query_system[688aca7ce0d190ff]::query::plumbing::get_query::<rustc_query_impl[e840a35d18f93fd8]::queries::analysis, rustc_query_impl[e840a35d18f93fd8]::plumbing::QueryCtxt>
  54:     0x7f7461ffd41e - <rustc_interface[694c7ab70a1d38bc]::passes::QueryContext>::enter::<rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>
  55:     0x7f7461fe109e - <rustc_interface[694c7ab70a1d38bc]::interface::Compiler>::enter::<rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}::{closure#2}, core[32c218dbf3427c26]::result::Result<core[32c218dbf3427c26]::option::Option<rustc_interface[694c7ab70a1d38bc]::queries::Linker>, rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>
  56:     0x7f7461fdd1ff - rustc_span[717885ec7c6cd182]::with_source_map::<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_interface[694c7ab70a1d38bc]::interface::create_compiler_and_run<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}>::{closure#1}>
  57:     0x7f7461ff94b0 - rustc_interface[694c7ab70a1d38bc]::interface::create_compiler_and_run::<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}>
  58:     0x7f746200d5b2 - <scoped_tls[f235d80db834c386]::ScopedKey<rustc_span[717885ec7c6cd182]::SessionGlobals>>::set::<rustc_interface[694c7ab70a1d38bc]::interface::run_compiler<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}>::{closure#0}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>
  59:     0x7f7461fdf78f - std[167b23ae759531ff]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[694c7ab70a1d38bc]::util::run_in_thread_pool_with_globals<rustc_interface[694c7ab70a1d38bc]::interface::run_compiler<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}>::{closure#0}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>::{closure#0}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>
  60:     0x7f7461ff9909 - <<std[167b23ae759531ff]::thread::Builder>::spawn_unchecked_<rustc_interface[694c7ab70a1d38bc]::util::run_in_thread_pool_with_globals<rustc_interface[694c7ab70a1d38bc]::interface::run_compiler<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}>::{closure#0}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>::{closure#0}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>::{closure#1} as core[32c218dbf3427c26]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7f745f601803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h86b1834fb0da834b
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/alloc/src/boxed.rs:1934:9
  62:     0x7f745f601803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h124d05192aaf60e0
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/alloc/src/boxed.rs:1934:9
  63:     0x7f745f601803 - std::sys::unix::thread::Thread::new::thread_start::h01e8d05fb6e030ea
                               at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys/unix/thread.rs:108:17
  64:     0x7f745f517609 - start_thread
  65:     0x7f745f43a133 - clone
  66:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (c2f428d2f 2022-07-14) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [is_sized_raw] computing whether `[&str]` is `Sized`
#1 [layout_of] computing layout of `&[&str]`
#2 [layout_of] computing layout of `Record<'_, '_>`
#3 [analysis] running analysis passes on this crate
end of query stack
warning: `demo` (lib) generated 3 warnings
error: could not compile `demo`; 3 warnings emitted
 ✘ dani@DESKTOP-DUL3MLM  ~/demo   master  rustc -vV                                                                                                                                                                                   (master|…3)
rustc 1.64.0-nightly (c2f428d2f 2022-07-14)
binary: rustc
commit-hash: c2f428d2f3340a0e7d995f4726223db91b93704c
commit-date: 2022-07-14
host: x86_64-unknown-linux-gnu
release: 1.64.0-nightly
LLVM version: 14.0.6
