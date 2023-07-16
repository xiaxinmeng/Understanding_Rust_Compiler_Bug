
error[E0658]: associated const equality is incomplete
 --> src/lib.rs:5:35
  |
5 | fn main<T, B: TraitWAssocConst<T, A = { 1 }>>() {}
  |                                   ^^^^^^^^^
  |
  = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
  = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable

error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:885:9: type parameter `T/#1` (T/1) out of range when substituting, substs=[]

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7feab567356a - std::backtrace_rs::backtrace::libunwind::trace::h96288e3e31cede0c
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7feab567356a - std::backtrace_rs::backtrace::trace_unsynchronized::h136276fcc921ebd2
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7feab567356a - std::sys_common::backtrace::_print_fmt::hed1407344e09c3c5
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7feab567356a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hef11c1a0e2ccbd67
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7feab56d3a3e - core::fmt::write::h9d34c1ab82e0e30d
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/core/src/fmt/mod.rs:1232:17
   5:     0x7feab5663b35 - std::io::Write::write_fmt::h583f259fb2d64e4c
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/io/mod.rs:1684:15
   6:     0x7feab5673335 - std::sys_common::backtrace::_print::h384b5154a89856a1
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7feab5673335 - std::sys_common::backtrace::print::he3caeede32862700
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7feab56760ff - std::panicking::default_hook::{{closure}}::hb37cdcd49b812817
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/panicking.rs:267:22
   9:     0x7feab5675e3b - std::panicking::default_hook::h89fa77719af1b8c7
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/panicking.rs:286:9
  10:     0x7feab898d824 - rustc_driver_impl[996a7fa2764f51ac]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7feab567693a - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hcdabce48acce76a1
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/alloc/src/boxed.rs:2002:9
  12:     0x7feab567693a - std::panicking::rust_panic_with_hook::h420ef2d822c4b069
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/panicking.rs:692:13
  13:     0x7feab8f0e1c1 - std[cd98010995c1c111]::panicking::begin_panic::<rustc_errors[aee54022d4af98b8]::ExplicitBug>::{closure#0}
  14:     0x7feab8f0e0d6 - std[cd98010995c1c111]::sys_common::backtrace::__rust_end_short_backtrace::<std[cd98010995c1c111]::panicking::begin_panic<rustc_errors[aee54022d4af98b8]::ExplicitBug>::{closure#0}, !>
  15:     0x7feab8e90a96 - std[cd98010995c1c111]::panicking::begin_panic::<rustc_errors[aee54022d4af98b8]::ExplicitBug>
  16:     0x7feab8f2b4b6 - std[cd98010995c1c111]::panic::panic_any::<rustc_errors[aee54022d4af98b8]::ExplicitBug>
  17:     0x7feab8f29686 - <rustc_errors[aee54022d4af98b8]::HandlerInner>::bug::<&alloc[d5203ca4bbacc4a0]::string::String>
  18:     0x7feab8f29360 - <rustc_errors[aee54022d4af98b8]::Handler>::bug::<&alloc[d5203ca4bbacc4a0]::string::String>
  19:     0x7feab8f4838b - rustc_middle[dd44835e384dc8fc]::util::bug::opt_span_bug_fmt::<rustc_span[c7747b3aedb35767]::span_encoding::Span>::{closure#0}
  20:     0x7feab8f4764a - rustc_middle[dd44835e384dc8fc]::ty::context::tls::with_opt::<rustc_middle[dd44835e384dc8fc]::util::bug::opt_span_bug_fmt<rustc_span[c7747b3aedb35767]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0x7feab8f47616 - rustc_middle[dd44835e384dc8fc]::ty::context::tls::with_context_opt::<rustc_middle[dd44835e384dc8fc]::ty::context::tls::with_opt<rustc_middle[dd44835e384dc8fc]::util::bug::opt_span_bug_fmt<rustc_span[c7747b3aedb35767]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0x7feab8f482d6 - rustc_middle[dd44835e384dc8fc]::util::bug::opt_span_bug_fmt::<rustc_span[c7747b3aedb35767]::span_encoding::Span>
  23:     0x7feab6cd2b43 - rustc_middle[dd44835e384dc8fc]::util::bug::bug_fmt
  24:     0x7feab8f46bae - <rustc_middle[dd44835e384dc8fc]::ty::subst::SubstFolder>::type_param_out_of_range
  25:     0x7feab6a7e040 - <rustc_middle[dd44835e384dc8fc]::ty::subst::SubstFolder as rustc_middle[dd44835e384dc8fc]::ty::fold::TypeFolder>::fold_ty
  26:     0x7feab734fd61 - rustc_ty_utils[1cc88f9ece2150ce]::instance::inner_resolve_instance
  27:     0x7feab7240923 - <rustc_query_system[1e3d87d6bb1db1ab]::dep_graph::graph::DepGraph<rustc_middle[dd44835e384dc8fc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[dd44835e384dc8fc]::ty::context::TyCtxt, rustc_middle[dd44835e384dc8fc]::ty::ParamEnvAnd<(rustc_span[c7747b3aedb35767]::def_id::DefId, &rustc_middle[dd44835e384dc8fc]::ty::list::List<rustc_middle[dd44835e384dc8fc]::ty::subst::GenericArg>)>, core[c2ed83ad2149f92c]::result::Result<core[c2ed83ad2149f92c]::option::Option<rustc_middle[dd44835e384dc8fc]::ty::instance::Instance>, rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>>
  28:     0x7feab723d18a - rustc_query_system[1e3d87d6bb1db1ab]::query::plumbing::get_query::<rustc_query_impl[f65b45c65392aba7]::queries::resolve_instance, rustc_query_impl[f65b45c65392aba7]::plumbing::QueryCtxt, rustc_middle[dd44835e384dc8fc]::dep_graph::dep_node::DepKind>
  29:     0x7feab6f7ef0a - <rustc_middle[dd44835e384dc8fc]::ty::instance::Instance>::resolve_opt_const_arg
  30:     0x7feab6d308ec - <rustc_middle[dd44835e384dc8fc]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  31:     0x7feab6d301b3 - <rustc_middle[dd44835e384dc8fc]::ty::consts::Const>::eval
  32:     0x7feab6d062fb - <rustc_trait_selection[fb65b6bdc136294c]::traits::project::AssocTypeNormalizer as rustc_middle[dd44835e384dc8fc]::ty::fold::TypeFolder>::fold_const
  33:     0x7feab6fc799a - <rustc_middle[dd44835e384dc8fc]::ty::sty::Binder<rustc_middle[dd44835e384dc8fc]::ty::PredicateKind> as rustc_middle[dd44835e384dc8fc]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[fb65b6bdc136294c]::traits::project::AssocTypeNormalizer>
  34:     0x7feab6fc7118 - <rustc_trait_selection[fb65b6bdc136294c]::traits::project::AssocTypeNormalizer as rustc_middle[dd44835e384dc8fc]::ty::fold::FallibleTypeFolder>::try_fold_predicate
  35:     0x7feab7d20e0e - <alloc[d5203ca4bbacc4a0]::vec::Vec<rustc_middle[dd44835e384dc8fc]::ty::Predicate> as alloc[d5203ca4bbacc4a0]::vec::spec_from_iter::SpecFromIter<rustc_middle[dd44835e384dc8fc]::ty::Predicate, core[c2ed83ad2149f92c]::iter::adapters::GenericShunt<core[c2ed83ad2149f92c]::iter::adapters::map::Map<alloc[d5203ca4bbacc4a0]::vec::into_iter::IntoIter<rustc_middle[dd44835e384dc8fc]::ty::Predicate>, <alloc[d5203ca4bbacc4a0]::vec::Vec<rustc_middle[dd44835e384dc8fc]::ty::Predicate> as rustc_middle[dd44835e384dc8fc]::ty::fold::TypeFoldable>::try_fold_with<rustc_trait_selection[fb65b6bdc136294c]::traits::project::AssocTypeNormalizer>::{closure#0}>, core[c2ed83ad2149f92c]::result::Result<core[c2ed83ad2149f92c]::convert::Infallible, !>>>>::from_iter
  36:     0x7feab71f9561 - rustc_trait_selection[fb65b6bdc136294c]::traits::do_normalize_predicates
  37:     0x7feab71f67aa - rustc_trait_selection[fb65b6bdc136294c]::traits::normalize_param_env_or_error
  38:     0x7feab71f55a6 - rustc_ty_utils[1cc88f9ece2150ce]::ty::param_env
  39:     0x7feab71fff2a - <rustc_query_system[1e3d87d6bb1db1ab]::dep_graph::graph::DepGraph<rustc_middle[dd44835e384dc8fc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[dd44835e384dc8fc]::ty::context::TyCtxt, rustc_span[c7747b3aedb35767]::def_id::DefId, rustc_middle[dd44835e384dc8fc]::ty::ParamEnv>
  40:     0x7feab71f38be - rustc_query_system[1e3d87d6bb1db1ab]::query::plumbing::get_query::<rustc_query_impl[f65b45c65392aba7]::queries::param_env, rustc_query_impl[f65b45c65392aba7]::plumbing::QueryCtxt, rustc_middle[dd44835e384dc8fc]::dep_graph::dep_node::DepKind>
  41:     0x7feab7609265 - rustc_hir_analysis[fafe05592f4b0540]::check::wfcheck::check_item_fn
  42:     0x7feab7604897 - rustc_hir_analysis[fafe05592f4b0540]::check::wfcheck::check_well_formed
  43:     0x7feab70e2c87 - <rustc_query_system[1e3d87d6bb1db1ab]::dep_graph::graph::DepGraph<rustc_middle[dd44835e384dc8fc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[dd44835e384dc8fc]::ty::context::TyCtxt, rustc_hir[1050a9e997bec054]::hir_id::OwnerId, ()>
  44:     0x7feab70e1d8e - rustc_query_system[1e3d87d6bb1db1ab]::query::plumbing::get_query::<rustc_query_impl[f65b45c65392aba7]::queries::check_well_formed, rustc_query_impl[f65b45c65392aba7]::plumbing::QueryCtxt, rustc_middle[dd44835e384dc8fc]::dep_graph::dep_node::DepKind>
  45:     0x7feab7ce62e6 - rustc_data_structures[24302e2764d485f4]::sync::par_for_each_in::<&[rustc_hir[1050a9e997bec054]::hir::ImplItemId], <rustc_middle[dd44835e384dc8fc]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[fafe05592f4b0540]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>
  46:     0x7feab7ce5f92 - rustc_hir_analysis[fafe05592f4b0540]::check::wfcheck::check_mod_type_wf
  47:     0x7feab7224c57 - <rustc_query_system[1e3d87d6bb1db1ab]::dep_graph::graph::DepGraph<rustc_middle[dd44835e384dc8fc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[dd44835e384dc8fc]::ty::context::TyCtxt, rustc_span[c7747b3aedb35767]::def_id::LocalDefId, ()>
  48:     0x7feab7ec6823 - rustc_query_system[1e3d87d6bb1db1ab]::query::plumbing::try_execute_query::<rustc_query_impl[f65b45c65392aba7]::queries::check_mod_type_wf, rustc_query_impl[f65b45c65392aba7]::plumbing::QueryCtxt>
  49:     0x7feab7ec612d - <rustc_query_impl[f65b45c65392aba7]::Queries as rustc_middle[dd44835e384dc8fc]::ty::query::QueryEngine>::check_mod_type_wf
  50:     0x7feab7ddb658 - rustc_data_structures[24302e2764d485f4]::sync::par_for_each_in::<&[rustc_hir[1050a9e997bec054]::hir_id::OwnerId], <rustc_middle[dd44835e384dc8fc]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[fafe05592f4b0540]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  51:     0x7feab7ddb49a - <rustc_session[8b3af3d4e67ba61e]::session::Session>::track_errors::<rustc_hir_analysis[fafe05592f4b0540]::check_crate::{closure#5}, ()>
  52:     0x7feab7dd8da2 - rustc_hir_analysis[fafe05592f4b0540]::check_crate
  53:     0x7feab7dd3034 - rustc_interface[82796a5553273f34]::passes::analysis
  54:     0x7feab812c0f6 - <rustc_query_system[1e3d87d6bb1db1ab]::dep_graph::graph::DepGraph<rustc_middle[dd44835e384dc8fc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[dd44835e384dc8fc]::ty::context::TyCtxt, (), core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>>
  55:     0x7feab812b701 - rustc_query_system[1e3d87d6bb1db1ab]::query::plumbing::try_execute_query::<rustc_query_impl[f65b45c65392aba7]::queries::analysis, rustc_query_impl[f65b45c65392aba7]::plumbing::QueryCtxt>
  56:     0x7feab832143a - <rustc_query_impl[f65b45c65392aba7]::Queries as rustc_middle[dd44835e384dc8fc]::ty::query::QueryEngine>::analysis
  57:     0x7feab802dad7 - <rustc_interface[82796a5553273f34]::passes::QueryContext>::enter::<rustc_driver_impl[996a7fa2764f51ac]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>>
  58:     0x7feab7b7a602 - <rustc_interface[82796a5553273f34]::interface::Compiler>::enter::<rustc_driver_impl[996a7fa2764f51ac]::run_compiler::{closure#1}::{closure#2}, core[c2ed83ad2149f92c]::result::Result<core[c2ed83ad2149f92c]::option::Option<rustc_interface[82796a5553273f34]::queries::Linker>, rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>>
  59:     0x7feab7b75694 - rustc_span[c7747b3aedb35767]::with_source_map::<core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>, rustc_interface[82796a5553273f34]::interface::run_compiler<core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>, rustc_driver_impl[996a7fa2764f51ac]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  60:     0x7feab7b751a4 - <scoped_tls[2e89e8a69196fbc0]::ScopedKey<rustc_span[c7747b3aedb35767]::SessionGlobals>>::set::<rustc_interface[82796a5553273f34]::interface::run_compiler<core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>, rustc_driver_impl[996a7fa2764f51ac]::run_compiler::{closure#1}>::{closure#0}, core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>>
  61:     0x7feab7b748a2 - std[cd98010995c1c111]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[82796a5553273f34]::util::run_in_thread_pool_with_globals<rustc_interface[82796a5553273f34]::interface::run_compiler<core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>, rustc_driver_impl[996a7fa2764f51ac]::run_compiler::{closure#1}>::{closure#0}, core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>>
  62:     0x7feab823927a - <<std[cd98010995c1c111]::thread::Builder>::spawn_unchecked_<rustc_interface[82796a5553273f34]::util::run_in_thread_pool_with_globals<rustc_interface[82796a5553273f34]::interface::run_compiler<core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>, rustc_driver_impl[996a7fa2764f51ac]::run_compiler::{closure#1}>::{closure#0}, core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c2ed83ad2149f92c]::result::Result<(), rustc_errors[aee54022d4af98b8]::ErrorGuaranteed>>::{closure#1} as core[c2ed83ad2149f92c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  63:     0x7feab5680933 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h125a8995d683d283
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/alloc/src/boxed.rs:1988:9
  64:     0x7feab5680933 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc8d7e704b7a14a23
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/alloc/src/boxed.rs:1988:9
  65:     0x7feab5680933 - std::sys::unix::thread::Thread::new::thread_start::hc8f4f2c9b0ce4632
                               at /rustc/8996ea93b6e554148c4286e62b613f12a3ee505c/library/std/src/sys/unix/thread.rs:108:17
  66:     0x7feab53b6b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  67:     0x7feab5448a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  68:                0x0 - <unknown>

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (8996ea93b 2023-02-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [resolve_instance] resolving instance `main::{constant#0}`
#1 [param_env] computing normalized predicates of `main`
#2 [check_well_formed] checking that `main` is well-formed
#3 [check_mod_type_wf] checking that types are well-formed in top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
