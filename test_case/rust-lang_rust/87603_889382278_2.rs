
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/decoder.rs:914:54
stack backtrace:
   0:     0x7fa5c682ef50 - std::backtrace_rs::backtrace::libunwind::trace::h99dbb39dca18857d
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7fa5c682ef50 - std::backtrace_rs::backtrace::trace_unsynchronized::h832861927e9cfedf
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fa5c682ef50 - std::sys_common::backtrace::_print_fmt::h3d18154c77dcf310
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7fa5c682ef50 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he312f4ad5b9bb346
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7fa5c689cbbc - core::fmt::write::h9a6d9c74526a6c1b
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/core/src/fmt/mod.rs:1115:17
   5:     0x7fa5c6820835 - std::io::Write::write_fmt::h6aced00850e8186f
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/io/mod.rs:1665:15
   6:     0x7fa5c6832c6b - std::sys_common::backtrace::_print::h65d996766de40da4
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7fa5c6832c6b - std::sys_common::backtrace::print::h40df9727e635f303
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7fa5c6832c6b - std::panicking::default_hook::{{closure}}::hd2da4327dea91a51
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/panicking.rs:208:50
   9:     0x7fa5c6832741 - std::panicking::default_hook::h3d55120ad6ada158
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/panicking.rs:225:9
  10:     0x7fa5c700f901 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h98690b446fb8b04c
  11:     0x7fa5c6833499 - std::panicking::rust_panic_with_hook::hf85dd0bb545e3b55
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/panicking.rs:626:17
  12:     0x7fa5c6832f27 - std::panicking::begin_panic_handler::{{closure}}::h736ae969434da9fa
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/panicking.rs:517:13
  13:     0x7fa5c682f42c - std::sys_common::backtrace::__rust_end_short_backtrace::h6133bb80b1d6c3e0
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/sys_common/backtrace.rs:141:18
  14:     0x7fa5c6832eb9 - rust_begin_unwind
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/panicking.rs:515:5
  15:     0x7fa5c67fdf81 - core::panicking::panic_fmt::hcf5f6d96e1dd7099
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/core/src/panicking.rs:92:14
  16:     0x7fa5c67fdecd - core::panicking::panic::hd695e3b1d0dd4ef4
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/core/src/panicking.rs:50:5
  17:     0x7fa5c8784030 - rustc_metadata::rmeta::decoder::<impl rustc_metadata::creader::CrateMetadataRef>::get_generics::h68c08440e7cd5899
  18:     0x7fa5c87cb5ef - rustc_metadata::rmeta::decoder::cstore_impl::provide_extern::generics_of::h244f00f04acaacca
  19:     0x7fa5c9086953 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hbf2718b46234167a
  20:     0x7fa5c911e86b - rustc_data_structures::stack::ensure_sufficient_stack::hf46a3fbe68f576af
  21:     0x7fa5c8fe045f - rustc_query_system::query::plumbing::force_query_with_job::h2bfcd51ed620fd2d
  22:     0x7fa5c86c186c - rustc_query_system::query::plumbing::get_query_impl::hf1da9a1f89479b58
  23:     0x7fa5c8712398 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::generics_of::h7d6cd789cb1fe6e1
  24:     0x7fa5c8a5b662 - <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path::had496e111b42bcb5
  25:     0x7fa5c8a5a513 - <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path::had496e111b42bcb5
  26:     0x7fa5c7f1f3c6 - <rustc_middle::ty::instance::Instance as core::fmt::Display>::fmt::h0c5745d36262db61
  27:     0x7fa5c7650169 - std::thread::local::LocalKey<T>::with::hbd544926e90f691f
  28:     0x7fa5c84f6031 - rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider::h7ddc05955f65f7a5
  29:     0x7fa5c909238a - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hfbb1047c89a80758
  30:     0x7fa5c911c362 - rustc_data_structures::stack::ensure_sufficient_stack::hcceeb0761b87a384
  31:     0x7fa5c868f676 - rustc_query_system::query::plumbing::get_query_impl::h2435f0c43c4516eb
  32:     0x7fa5c8713f8c - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw::h4226693088f14f86
  33:     0x7fa5c84f2d57 - rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider::h4526da75a715c4b0
  34:     0x7fa5c9069aaa - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h17179f278314b8b2
  35:     0x7fa5c9112df2 - rustc_data_structures::stack::ensure_sufficient_stack::h33596c9a274600d8
  36:     0x7fa5c86a0703 - rustc_query_system::query::plumbing::get_query_impl::h6a92f4db05971a2b
  37:     0x7fa5c87140b0 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw::h07a7f193859e039e
  38:     0x7fa5c84f2c4d - rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider::h4526da75a715c4b0
  39:     0x7fa5c9069aaa - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h17179f278314b8b2
  40:     0x7fa5c9112df2 - rustc_data_structures::stack::ensure_sufficient_stack::h33596c9a274600d8
  41:     0x7fa5c86a0703 - rustc_query_system::query::plumbing::get_query_impl::h6a92f4db05971a2b
  42:     0x7fa5c87140b0 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw::h07a7f193859e039e
  43:     0x7fa5c8a14fec - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id::h287aca02c5aa6ea8
  44:     0x7fa5c8a26e6d - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::h5d552cf5930c6db9
  45:     0x7fa5c899b727 - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const::h0713350ea9193001
  46:     0x7fa5c898419d - rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with::hca04719a4d747983
  47:     0x7fa5c899bf91 - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty::hb12b624657a28536
  48:     0x7fa5c8199e74 - <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize::hd00b5584c1c503e7
  49:     0x7fa5c818c55d - rustc_infer::infer::InferCtxtBuilder::enter::h1977b323bd1b8e82
  50:     0x7fa5c819b258 - core::ops::function::FnOnce::call_once::hed15947f8a72ca50
  51:     0x7fa5c907b2a9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h7c96b1ae6a0659f5
  52:     0x7fa5c911ba2a - rustc_data_structures::stack::ensure_sufficient_stack::hc1f425b3dea81e44
  53:     0x7fa5c86a3544 - rustc_query_system::query::plumbing::get_query_impl::h71fbb143b181a189
  54:     0x7fa5c8715f9f - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_generic_arg_after_erasing_regions::ha93a4a19b0df1346
  55:     0x7fa5c8a4513e - rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder::normalize_generic_arg_after_erasing_regions::haa4528007f503d15
  56:     0x7fa5c8a45316 - <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty::hf7fc10de5afb32ce
  57:     0x7fa5c8314d9b - <rustc_ty_utils::needs_drop::NeedsDropTypes<F> as core::iter::traits::iterator::Iterator>::next::hbe3b069c7ea189eb
  58:     0x7fa5c8dd0840 - <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter::h7c458704c5fca606
  59:     0x7fa5c8dd3d6a - rustc_ty_utils::needs_drop::adt_drop_tys::hf8c0c830032c130a
  60:     0x7fa5c90694d7 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h147859078023ffd9
  61:     0x7fa5c911125a - rustc_data_structures::stack::ensure_sufficient_stack::h1b2a3507d420b6f6
  62:     0x7fa5c8fe3e2d - rustc_query_system::query::plumbing::force_query_with_job::h54896ff3c3f02ce5
  63:     0x7fa5c8f904ee - rustc_query_system::query::plumbing::get_query_impl::h200c2b627bda90f2
  64:     0x7fa5c90d238b - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::adt_drop_tys::h028e54a47c502c4d
  65:     0x7fa5c8313eba - rustc_ty_utils::needs_drop::needs_drop_raw::hf646c9e2ba63273e
  66:     0x7fa5c9072729 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h4b8d93b2f373dac4
  67:     0x7fa5c911561a - rustc_data_structures::stack::ensure_sufficient_stack::h5d925c0394a673f1
  68:     0x7fa5c86955fd - rustc_query_system::query::plumbing::get_query_impl::h3fccbd456814bf05
  69:     0x7fa5c87150df - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::needs_drop_raw::h60f3e573f9bb9749
  70:     0x7fa5c8465824 - rustc_middle::ty::util::<impl rustc_middle::ty::TyS>::needs_drop::h10074a318e5dfcca
  71:     0x7fa5c848e38e - rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::schedule_drop::h02803014ba5802db
  72:     0x7fa5c846d8c6 - rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block_stmts::h9fbf7921344b21f6
  73:     0x7fa5c848c18b - rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope::h2500d92deda042e7
  74:     0x7fa5c84777f5 - rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::expr_into_dest::hae152d11e0bdfc7f
  75:     0x7fa5c848d2dc - rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope::h595a11472b5b8edd
  76:     0x7fa5c8476e5b - rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::expr_into_dest::hae152d11e0bdfc7f
  77:     0x7fa5c848d2dc - rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope::h595a11472b5b8edd
  78:     0x7fa5c8476e5b - rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::expr_into_dest::hae152d11e0bdfc7f
  79:     0x7fa5c8468f98 - rustc_mir_build::build::construct_fn::h1a20768377866ac0
  80:     0x7fa5c84c7192 - rustc_infer::infer::InferCtxtBuilder::enter::h9390c08ed3a27e85
  81:     0x7fa5c846760b - rustc_mir_build::build::mir_built::hb54154e5f15b1cfb
  82:     0x7fa5c9078459 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h69fa311e07c37bff
  83:     0x7fa5c9113903 - rustc_data_structures::stack::ensure_sufficient_stack::h39066a387a6b8535
  84:     0x7fa5c86afaf5 - rustc_query_system::query::plumbing::get_query_impl::ha122417aca8a1f7a
  85:     0x7fa5c871287f - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_built::h26a32b68ddb2fe1a
  86:     0x7fa5c8556037 - rustc_mir::transform::check_unsafety::unsafety_check_result::ha914a8ddcd5e7c0c
  87:     0x7fa5c854f46e - core::ops::function::FnOnce::call_once::hed6bfea6d13fa8ad
  88:     0x7fa5c908d108 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::he2db4770d36cb639
  89:     0x7fa5c911b02a - rustc_data_structures::stack::ensure_sufficient_stack::hb607e0890c8c1e60
  90:     0x7fa5c8fe5045 - rustc_query_system::query::plumbing::force_query_with_job::h5f5116486341f957
  91:     0x7fa5c86bbac6 - rustc_query_system::query::plumbing::get_query_impl::hd5b5359cf1415bcd
  92:     0x7fa5c8713acb - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::unsafety_check_result::h37531d273210bdac
  93:     0x7fa5c8506e57 - rustc_mir::transform::mir_const::hb581d590b604d428
  94:     0x7fa5c9078459 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h69fa311e07c37bff
  95:     0x7fa5c9113903 - rustc_data_structures::stack::ensure_sufficient_stack::h39066a387a6b8535
  96:     0x7fa5c86afaf5 - rustc_query_system::query::plumbing::get_query_impl::ha122417aca8a1f7a
  97:     0x7fa5c871293f - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_const::h0ad6b879d8ecfac4
  98:     0x7fa5c8507f26 - rustc_mir::transform::mir_promoted::ha5f0e20517ad025b
  99:     0x7fa5c907670c - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h5f961300f940fe48
 100:     0x7fa5c911dc23 - rustc_data_structures::stack::ensure_sufficient_stack::he67a762b8327000a
 101:     0x7fa5c86bf6a8 - rustc_query_system::query::plumbing::get_query_impl::heae5809a396aee72
 102:     0x7fa5c8712c8f - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_promoted::h2df66bb6ee036a5f
 103:     0x7fa5c860f5f0 - rustc_mir::borrow_check::mir_borrowck::h8eb6c1f0b14d33e1
 104:     0x7fa5c8605ac8 - core::ops::function::FnOnce::call_once::h6f2854470b5c806a
 105:     0x7fa5c9087548 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hc278890e4dec1421
 106:     0x7fa5c911766a - rustc_data_structures::stack::ensure_sufficient_stack::h7df3ec8b2591229c
 107:     0x7fa5c8fe6c95 - rustc_query_system::query::plumbing::force_query_with_job::h6f86ffe1c88de4c6
 108:     0x7fa5c86932b3 - rustc_query_system::query::plumbing::get_query_impl::h332a4151e7422444
 109:     0x7fa5c8713eab - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck::h3eeeb2427f584737
 110:     0x7fa5c811ee61 - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners::h6bb5f491d39e5713
 111:     0x7fa5c8b2a049 - rustc_interface::passes::analysis::h50d0de1731ee4f37
 112:     0x7fa5c7a03773 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::ha410030de379e7fa
 113:     0x7fa5c79ca017 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h7d88f9b0801b7005
 114:     0x7fa5c79d8f79 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::hfb5ebbe42af10190
 115:     0x7fa5c7a60ebe - rustc_data_structures::stack::ensure_sufficient_stack::h41e0719d654b87ab
 116:     0x7fa5c781eaed - rustc_query_system::query::plumbing::force_query_with_job::h739ef3432ad7844a
 117:     0x7fa5c8fce8d6 - rustc_query_system::query::plumbing::get_query_impl::hfb328957edb27639
 118:     0x7fa5c90cefdd - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis::h1bafb686f4c64bd1
 119:     0x7fa5c8b17337 - rustc_interface::passes::QueryContext::enter::hc03ebce69d470d05
 120:     0x7fa5c8afe717 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h9ff1bb326f6cbdde
 121:     0x7fa5c8aedbbc - rustc_span::with_source_map::h35aa4a6792e2f0de
 122:     0x7fa5c8aff46a - rustc_interface::interface::create_compiler_and_run::h600b31664ea7498c
 123:     0x7fa5c8af1d49 - scoped_tls::ScopedKey<T>::set::h95fded8ea1ce829a
 124:     0x7fa5c8aee18a - std::sys_common::backtrace::__rust_begin_short_backtrace::hcb9f52ef891273be
 125:     0x7fa5c8aed475 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h7d639d5804de8241
 126:     0x7fa5c683fa27 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h09f167e08fc945ff
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/alloc/src/boxed.rs:1572:9
 127:     0x7fa5c683fa27 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf814fe85eeb118e0
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/alloc/src/boxed.rs:1572:9
 128:     0x7fa5c683fa27 - std::sys::unix::thread::Thread::new::thread_start::hb71b17934c5f5e68
                               at /rustc/b70888601af92f6cdc0364abab3446e418b91d36/library/std/src/sys/unix/thread.rs:91:17
 129:     0x7fa5c6779259 - start_thread
 130:     0x7fa5c668e5e3 - __GI___clone
 131:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (b70888601 2021-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [generics_of] computing generics of `sb3::S::s`
#1 [eval_to_allocation_raw] const-evaluating + checking `sb3::S::s::{constant#0}`
#2 [eval_to_const_value_raw] simplifying constant for the type system `sb3::S::s::{constant#0}`
#3 [eval_to_const_value_raw] simplifying constant for the type system `sb3::S::s::{constant#0}`
#4 [normalize_generic_arg_after_erasing_regions] normalizing `[T; _]`
#5 [adt_drop_tys] computing when `sb3::S` needs drop
#6 [needs_drop_raw] computing whether `sb3::S<u8, 2_usize>` needs drop
#7 [mir_built] building MIR for `main`
#8 [unsafety_check_result] unsafety-checking `main`
#9 [mir_const] processing MIR for `main`
#10 [mir_promoted] processing `main`
#11 [mir_borrowck] borrow-checking `main`
#12 [analysis] running analysis passes on this crate
end of query stack
