
❯ cargo fix --edition
[snip]
    Checking halo2_proofs v0.1.0-beta.4 (/home/str4d/dev/rust/zcash/halo2/halo2_proofs)
   Migrating halo2_proofs/src/lib.rs from 2018 edition to 2021
   Compiling gumdrop_derive v0.8.1
   Compiling thiserror-impl v1.0.30
    Checking gumdrop v0.8.1
error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:54:32: could not fully normalize `plonk::lookup::Argument<<plonk::lookup::Argument<<C as pasta_curves::arithmetic::CurveAffine>::ScalarExt> as pasta_curves::arithmetic::CurveAffine>::ScalarExt>`

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1147:9
stack backtrace:
   0:     0x7f128ab6151c - std::backtrace_rs::backtrace::libunwind::trace::h3fea1eb2e0ba2ac9
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f128ab6151c - std::backtrace_rs::backtrace::trace_unsynchronized::h849d83492cbc0d59
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f128ab6151c - std::sys_common::backtrace::_print_fmt::he3179d37290f23d3
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f128ab6151c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h140f6925cad14324
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f128abbf22c - core::fmt::write::h31b9cd1bedd7ea38
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/fmt/mod.rs:1150:17
   5:     0x7f128ab525a5 - std::io::Write::write_fmt::h1fdf66f83f70913e
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/io/mod.rs:1667:15
   6:     0x7f128ab64a80 - std::sys_common::backtrace::_print::he7ac492cd19c3189
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f128ab64a80 - std::sys_common::backtrace::print::hba20f8920229d8e8
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f128ab64a80 - std::panicking::default_hook::{{closure}}::h714d63979ae18678
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:210:50
   9:     0x7f128ab64637 - std::panicking::default_hook::hf1afb64e69563ca8
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:227:9
  10:     0x7f128b339b01 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h045e68b20488b66e
  11:     0x7f128ab65299 - std::panicking::rust_panic_with_hook::h02231a501e274a13
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:628:17
  12:     0x7f128c374f8b - std::panicking::begin_panic::{{closure}}::h777af1b405e6beed
  13:     0x7f128c374ed6 - std::sys_common::backtrace::__rust_end_short_backtrace::h67a7e7853206400c
  14:     0x7f128c375eff - std::panicking::begin_panic::hd769b9ceefdb14a6
  15:     0x7f128c38534d - std::panic::panic_any::he7034fd861bf1917
  16:     0x7f128c3873ce - rustc_errors::HandlerInner::bug::h07aea37442b50e13
  17:     0x7f128c386bf0 - rustc_errors::Handler::bug::hcc6e735d806ec79e
  18:     0x7f128c21260e - rustc_middle::ty::context::tls::with_opt::h04e01420349e359b
  19:     0x7f128c2129d0 - rustc_middle::util::bug::opt_span_bug_fmt::h4a82ba802d444f24
  20:     0x7f128c212946 - rustc_middle::util::bug::bug_fmt::hcf1fd372902c876e
  21:     0x7f128c9c3d0a - rustc_infer::infer::InferCtxtBuilder::enter::h8a82609332438ccf
  22:     0x7f128c9d04a8 - core::ops::function::FnOnce::call_once::hf98335709d8238cc
  23:     0x7f128d46ddaf - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h996037a55653400a
  24:     0x7f128d507b9a - rustc_data_structures::stack::ensure_sufficient_stack::h09c07f58c17605a7
  25:     0x7f128c9d600e - rustc_query_system::query::plumbing::get_query_impl::h1a6c0649c5ddb30e
  26:     0x7f128ca71aaf - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_generic_arg_after_erasing_regions::h9a080611e25ef801
  27:     0x7f128cd8e20b - rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder::normalize_generic_arg_after_erasing_regions::h4a5e623a5f9ea400
  28:     0x7f128cd8e476 - <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty::h225ddd65985624e6
  29:     0x7f128b60bdd2 - <rustc_ty_utils::needs_drop::NeedsDropTypes<F> as core::iter::traits::iterator::Iterator>::next::h7087826c0e8380b2
  30:     0x7f128b602eba - <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next::hd098c85d4190d5ab
  31:     0x7f128b60775c - <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter::h80bc373cacd14a1e
  32:     0x7f128b603506 - core::iter::adapters::process_results::h22c2b36db2b59a59
  33:     0x7f128b60c4b5 - rustc_ty_utils::needs_drop::adt_significant_drop_tys::h5c72b56396dba9f5
  34:     0x7f128d47cffd - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::heb71a11daa2d363b
  35:     0x7f128d51c9da - rustc_data_structures::stack::ensure_sufficient_stack::hb11b8554afb5d568
  36:     0x7f128d3a5ca0 - rustc_query_system::query::plumbing::force_query_with_job::h98c5b82a6efe1896
  37:     0x7f128d34be94 - rustc_query_system::query::plumbing::get_query_impl::h190c6fd7bd23a572
  38:     0x7f128bb7818c - rustc_query_system::query::plumbing::get_query::h3fba9f7d1804ce48
  39:     0x7f128b60af4d - rustc_ty_utils::needs_drop::has_significant_drop_raw::hd7f1b4b110235212
  40:     0x7f128ca5138f - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hfef521bc64664170
  41:     0x7f128ca9d61a - rustc_data_structures::stack::ensure_sufficient_stack::hade48fba7b3bb2bb
  42:     0x7f128c9efcb4 - rustc_query_system::query::plumbing::get_query_impl::h96bce355c1983d62
  43:     0x7f128bb7922f - rustc_query_system::query::plumbing::get_query::hf1549911cb874bfc
  44:     0x7f128b51f8ee - rustc_middle::ty::util::<impl rustc_middle::ty::TyS>::has_significant_drop::h73b4c825c4ccb993
  45:     0x7f128d082c28 - rustc_typeck::check::upvar::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::analyze_closure::h209d1f3aff7b0595
  46:     0x7f128c594879 - rustc_hir::intravisit::walk_expr::hc2c05fe08769e282
  47:     0x7f128c5931dd - rustc_hir::intravisit::walk_expr::hc2c05fe08769e282
  48:     0x7f128c64c914 - <rustc_typeck::check::upvar::InferBorrowKindVisitor as rustc_hir::intravisit::Visitor>::visit_expr::h6410406813120c58
  49:     0x7f128c596169 - rustc_hir::intravisit::walk_expr::hc2c05fe08769e282
  50:     0x7f128c64c914 - <rustc_typeck::check::upvar::InferBorrowKindVisitor as rustc_hir::intravisit::Visitor>::visit_expr::h6410406813120c58
  51:     0x7f128c579bca - rustc_typeck::check::upvar::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::closure_analyze::he38f50bb079f1258
  52:     0x7f128c5f692d - rustc_infer::infer::InferCtxtBuilder::enter::h5192e9567560c6af
  53:     0x7f128c59d9b4 - rustc_typeck::check::typeck::ha7bb30975bb47ca6
  54:     0x7f128d47458a - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hb4aaab7d57cb4bf4
  55:     0x7f128d5130aa - rustc_data_structures::stack::ensure_sufficient_stack::h63ae52df0ead7d8f
  56:     0x7f128d3a52f1 - rustc_query_system::query::plumbing::force_query_with_job::h8fbdba5993d908b0
  57:     0x7f128c9de220 - rustc_query_system::query::plumbing::get_query_impl::h4ffc561cbec4e552
  58:     0x7f128ca6fb1b - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck::h508dd00c2355268b
  59:     0x7f128c59dcd8 - rustc_typeck::check::typeck::ha7bb30975bb47ca6
  60:     0x7f128d47458a - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hb4aaab7d57cb4bf4
  61:     0x7f128d5130aa - rustc_data_structures::stack::ensure_sufficient_stack::h63ae52df0ead7d8f
  62:     0x7f128d3a52f1 - rustc_query_system::query::plumbing::force_query_with_job::h8fbdba5993d908b0
  63:     0x7f128c9de220 - rustc_query_system::query::plumbing::get_query_impl::h4ffc561cbec4e552
  64:     0x7f128ca6fb1b - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck::h508dd00c2355268b
  65:     0x7f128c5bea87 - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners::h19a12f38b112685c
  66:     0x7f128d09898c - rustc_typeck::check::typeck_item_bodies::hfd0c71c244038ff3
  67:     0x7f128d46eed0 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h9e6154181a71a0f3
  68:     0x7f128d39aea4 - rustc_query_system::query::plumbing::force_query_with_job::h2ef444aa2eadce7a
  69:     0x7f128d368f04 - rustc_query_system::query::plumbing::get_query_impl::h6a95e4b1154f34f1
  70:     0x7f128d4c6650 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck_item_bodies::hdcd851568e3c0aeb
  71:     0x7f128d0b257f - rustc_session::utils::<impl rustc_session::session::Session>::time::h45748f8a3a0f0350
  72:     0x7f128d0a46b7 - rustc_typeck::check_crate::hb80848c920cf28e8
  73:     0x7f128ce70133 - rustc_interface::passes::analysis::hcd758edec2335b9e
  74:     0x7f128d459a30 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h34003e747609006c
  75:     0x7f128d51cae5 - rustc_data_structures::stack::ensure_sufficient_stack::hb213c0190b92ec18
  76:     0x7f128d3a10ab - rustc_query_system::query::plumbing::force_query_with_job::h648670b229bdc950
  77:     0x7f128d3865b2 - rustc_query_system::query::plumbing::get_query_impl::he088956570c68f6d
  78:     0x7f128d4c43dd - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis::hdd4e495c3d55dc4d
  79:     0x7f128ce64bc7 - rustc_interface::passes::QueryContext::enter::h87e76f1930f25935
  80:     0x7f128ce4bca1 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hff7e36ca6c2085f0
  81:     0x7f128ce3902c - rustc_span::with_source_map::he4ccdc3d435ea73b
  82:     0x7f128ce4b5cc - scoped_tls::ScopedKey<T>::set::hf1cc160e90acdba2
  83:     0x7f128ce3a73b - std::sys_common::backtrace::__rust_begin_short_backtrace::h0b3064789b6c03d4
  84:     0x7f128ce37905 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h3907954af9fcbd87
  85:     0x7f128ab718e3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf2a5e508372e2387
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/alloc/src/boxed.rs:1636:9
  86:     0x7f128ab718e3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h46642f5b75478456
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/alloc/src/boxed.rs:1636:9
  87:     0x7f128ab718e3 - std::sys::unix::thread::Thread::new::thread_start::h1fbdb50adbde1cab
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys/unix/thread.rs:106:17
  88:     0x7f128aaa1609 - start_thread
                               at /build/glibc-sMfBJT/glibc-2.31/nptl/pthread_create.c:477:8
  89:     0x7f128a9b4163 - clone
  90:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.1 (59eed8a2a 2021-11-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [normalize_generic_arg_after_erasing_regions] normalizing `plonk::lookup::Argument<<plonk::lookup::Argument<<C as pasta_curves::arithmetic::CurveAffine>::ScalarExt> as pasta_curves::arithmetic::CurveAffine>::ScalarExt>`
#1 [adt_significant_drop_tys] computing when `plonk::VerifyingKey` has a significant destructor
#2 [has_significant_drop_raw] computing whether `plonk::VerifyingKey<C>` has a significant drop
#3 [typeck] type-checking `plonk::keygen::keygen_pk`
#4 [typeck] type-checking `plonk::keygen::keygen_pk::{closure#0}`
#5 [typeck_item_bodies] type-checking all item bodies
#6 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `halo2_proofs`
warning: build failed, waiting for other jobs to finish...
error: build failed
