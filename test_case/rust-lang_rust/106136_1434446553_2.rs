
thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [Binder(TraitPredicate(<RBuilder_ as std::marker::Sized>, polarity:Positive), []), Binder(TraitPredicate(<RBuilder_ as std::ops::FnOnce<(&'this std::boxed::Box<fn(&'a ())>,)>>, polarity:Positive), [Region(BrNamed(DefId(0:21 ~ scratch[473e]::{impl#0}::'this), 'this))]), Binder(TraitPredicate(<Error_ as std::marker::Sized>, polarity:Positive), []), Binder(ProjectionPredicate(AliasTy { substs: [RBuilder_, (&'this std::boxed::Box<fn(&'a ())>,)], def_id: DefId(2:2923 ~ core[f0a8]::ops::function::FnOnce::Output) }, Term::Ty(std::pin::Pin<std::boxed::Box<(dyn std::future::Future<Output = std::result::Result<&'this u8, Error_>> + std::marker::Send + 'this)>>)), [Region(BrNamed(DefId(0:21 ~ scratch[473e]::{impl#0}::'this), 'this))])], reveal: UserFacing, constness: NotConst }, value: Normalize { value: [async fn body@src/main.rs:43:69: 45:14] } } }
- dep-node: type_op_normalize_ty(7eb331c2f1b84397-a4e29389866d0526)', /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/compiler/rustc_query_system/src/dep_graph/graph.rs:316:9
stack backtrace:
   0:     0x7fc90bf6a9ea - std::backtrace_rs::backtrace::libunwind::trace::h3a20221a56d872e6
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fc90bf6a9ea - std::backtrace_rs::backtrace::trace_unsynchronized::h23863182fd4b4481
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc90bf6a9ea - std::sys_common::backtrace::_print_fmt::hde799c83b2fa8ddb
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fc90bf6a9ea - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb1ed906e28a16738
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fc90bfcaafe - core::fmt::write::hee7bc8dd2696f088
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/core/src/fmt/mod.rs:1232:17
   5:     0x7fc90bf5b005 - std::io::Write::write_fmt::h031e716f6e3d1f66
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/io/mod.rs:1684:15
   6:     0x7fc90bf6a7b5 - std::sys_common::backtrace::_print::h8cea91a8dc3abdb3
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fc90bf6a7b5 - std::sys_common::backtrace::print::hcd9f90783d8afb1f
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fc90bf6d54f - std::panicking::default_hook::{{closure}}::hdf7c561ea2419acd
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/panicking.rs:267:22
   9:     0x7fc90bf6d28b - std::panicking::default_hook::h9669759e65745add
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/panicking.rs:286:9
  10:     0x7fc90f2b8c44 - <rustc_driver_impl[cf5be2d75a70ff02]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[f0a84df7292b9247]::ops::function::FnOnce<(&core[f0a84df7292b9247]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7fc90bf6dd8a - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h71e54ddf4b01f8ef
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/alloc/src/boxed.rs:2002:9
  12:     0x7fc90bf6dd8a - std::panicking::rust_panic_with_hook::h96419a4a3670d907
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/panicking.rs:692:13
  13:     0x7fc90bf6db09 - std::panicking::begin_panic_handler::{{closure}}::h17c261a3ff4b53aa
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/panicking.rs:579:13
  14:     0x7fc90bf6ae8c - std::sys_common::backtrace::__rust_end_short_backtrace::h6f10911888b9e113
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/sys_common/backtrace.rs:137:18
  15:     0x7fc90bf6d812 - rust_begin_unwind
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/panicking.rs:575:5
  16:     0x7fc90bfc74a3 - core::panicking::panic_fmt::hdb483708f9fb42ab
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/core/src/panicking.rs:64:14
  17:     0x7fc90e67daa5 - <rustc_query_system[9c20c97984254eda]::dep_graph::graph::DepGraph<rustc_middle[bd03a598433729a4]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[bd03a598433729a4]::ty::context::TyCtxt, rustc_middle[bd03a598433729a4]::infer::canonical::Canonical<rustc_middle[bd03a598433729a4]::ty::ParamEnvAnd<rustc_middle[bd03a598433729a4]::traits::query::type_op::Normalize<rustc_middle[bd03a598433729a4]::ty::Ty>>>, core[f0a84df7292b9247]::result::Result<&rustc_middle[bd03a598433729a4]::infer::canonical::Canonical<rustc_middle[bd03a598433729a4]::infer::canonical::QueryResponse<rustc_middle[bd03a598433729a4]::ty::Ty>>, rustc_middle[bd03a598433729a4]::traits::query::NoSolution>>
  18:     0x7fc90e67cc0d - rustc_query_system[9c20c97984254eda]::query::plumbing::try_execute_query::<rustc_query_impl[e25e6056fa54423]::queries::type_op_normalize_ty, rustc_query_impl[e25e6056fa54423]::plumbing::QueryCtxt>
  19:     0x7fc90ec4bc8e - <rustc_query_impl[e25e6056fa54423]::Queries as rustc_middle[bd03a598433729a4]::ty::query::QueryEngine>::type_op_normalize_ty
  20:     0x7fc90e113eb8 - <rustc_middle[bd03a598433729a4]::ty::ParamEnvAnd<rustc_middle[bd03a598433729a4]::traits::query::type_op::Normalize<rustc_middle[bd03a598433729a4]::ty::Ty>> as rustc_trait_selection[42a1777adec956b8]::traits::query::type_op::TypeOp>::fully_perform
  21:     0x7fc90e0e7f5c - rustc_borrowck[1b2a3a49d27f4d4b]::type_check::free_region_relations::create
  22:     0x7fc90e0b0d63 - rustc_borrowck[1b2a3a49d27f4d4b]::type_check::type_check
  23:     0x7fc90e0a0dca - rustc_borrowck[1b2a3a49d27f4d4b]::nll::compute_regions
  24:     0x7fc90e083ca0 - rustc_borrowck[1b2a3a49d27f4d4b]::do_mir_borrowck
  25:     0x7fc90e06d3bf - rustc_borrowck[1b2a3a49d27f4d4b]::mir_borrowck
  26:     0x7fc90e06ca7b - <rustc_borrowck[1b2a3a49d27f4d4b]::provide::{closure#0} as core[f0a84df7292b9247]::ops::function::FnOnce<(rustc_middle[bd03a598433729a4]::ty::context::TyCtxt, rustc_span[ff2b83d94cd5766c]::def_id::LocalDefId)>>::call_once
  27:     0x7fc90db8f020 - <rustc_query_system[9c20c97984254eda]::dep_graph::graph::DepGraph<rustc_middle[bd03a598433729a4]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[bd03a598433729a4]::ty::context::TyCtxt, rustc_span[ff2b83d94cd5766c]::def_id::LocalDefId, &rustc_middle[bd03a598433729a4]::mir::query::BorrowCheckResult>
  28:     0x7fc90db8de42 - rustc_query_system[9c20c97984254eda]::query::plumbing::try_execute_query::<rustc_query_impl[e25e6056fa54423]::queries::mir_borrowck, rustc_query_impl[e25e6056fa54423]::plumbing::QueryCtxt>
  29:     0x7fc90ec438ed - <rustc_query_impl[e25e6056fa54423]::Queries as rustc_middle[bd03a598433729a4]::ty::query::QueryEngine>::mir_borrowck
  30:     0x7fc90e163861 - <rustc_borrowck[1b2a3a49d27f4d4b]::type_check::TypeChecker>::prove_closure_bounds
  31:     0x7fc90e14247c - <rustc_borrowck[1b2a3a49d27f4d4b]::type_check::TypeChecker>::typeck_mir
  32:     0x7fc90e0b10dc - rustc_borrowck[1b2a3a49d27f4d4b]::type_check::type_check
  33:     0x7fc90e0a0dca - rustc_borrowck[1b2a3a49d27f4d4b]::nll::compute_regions
  34:     0x7fc90e083ca0 - rustc_borrowck[1b2a3a49d27f4d4b]::do_mir_borrowck
  35:     0x7fc90e06d3bf - rustc_borrowck[1b2a3a49d27f4d4b]::mir_borrowck
  36:     0x7fc90e06ca7b - <rustc_borrowck[1b2a3a49d27f4d4b]::provide::{closure#0} as core[f0a84df7292b9247]::ops::function::FnOnce<(rustc_middle[bd03a598433729a4]::ty::context::TyCtxt, rustc_span[ff2b83d94cd5766c]::def_id::LocalDefId)>>::call_once
  37:     0x7fc90db8f020 - <rustc_query_system[9c20c97984254eda]::dep_graph::graph::DepGraph<rustc_middle[bd03a598433729a4]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[bd03a598433729a4]::ty::context::TyCtxt, rustc_span[ff2b83d94cd5766c]::def_id::LocalDefId, &rustc_middle[bd03a598433729a4]::mir::query::BorrowCheckResult>
  38:     0x7fc90db8de42 - rustc_query_system[9c20c97984254eda]::query::plumbing::try_execute_query::<rustc_query_impl[e25e6056fa54423]::queries::mir_borrowck, rustc_query_impl[e25e6056fa54423]::plumbing::QueryCtxt>
  39:     0x7fc90fb52005 - rustc_query_system[9c20c97984254eda]::query::plumbing::force_query::<rustc_query_impl[e25e6056fa54423]::queries::mir_borrowck, rustc_query_impl[e25e6056fa54423]::plumbing::QueryCtxt, rustc_middle[bd03a598433729a4]::dep_graph::dep_node::DepKind>
  40:     0x7fc90fc86361 - rustc_query_impl[e25e6056fa54423]::plumbing::force_from_dep_node::<rustc_query_impl[e25e6056fa54423]::queries::mir_borrowck>
  41:     0x7fc90d40b852 - <rustc_query_system[9c20c97984254eda]::dep_graph::graph::DepGraph<rustc_middle[bd03a598433729a4]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[e25e6056fa54423]::plumbing::QueryCtxt>
  42:     0x7fc90d40b7ed - <rustc_query_system[9c20c97984254eda]::dep_graph::graph::DepGraph<rustc_middle[bd03a598433729a4]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[e25e6056fa54423]::plumbing::QueryCtxt>
  43:     0x7fc90d40b5c1 - <rustc_query_system[9c20c97984254eda]::dep_graph::graph::DepGraph<rustc_middle[bd03a598433729a4]::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl[e25e6056fa54423]::plumbing::QueryCtxt>
  44:     0x7fc90e82f86d - rustc_query_system[9c20c97984254eda]::query::plumbing::ensure_must_run::<rustc_query_impl[e25e6056fa54423]::queries::check_mod_item_types, rustc_query_impl[e25e6056fa54423]::plumbing::QueryCtxt>
  45:     0x7fc90ec42df9 - <rustc_query_impl[e25e6056fa54423]::Queries as rustc_middle[bd03a598433729a4]::ty::query::QueryEngine>::check_mod_item_types
  46:     0x7fc90e6f9f21 - <rustc_session[130bf05e2988f5fc]::session::Session>::time::<(), rustc_hir_analysis[7e2943bc0d9db6d0]::check_crate::{closure#6}>
  47:     0x7fc90e6f73fd - rustc_hir_analysis[7e2943bc0d9db6d0]::check_crate
  48:     0x7fc90e6ea9d4 - rustc_interface[e19372daae8a4d61]::passes::analysis
  49:     0x7fc90ea4b57a - <rustc_query_system[9c20c97984254eda]::dep_graph::graph::DepGraph<rustc_middle[bd03a598433729a4]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[bd03a598433729a4]::ty::context::TyCtxt, (), core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>>
  50:     0x7fc90ea4abe9 - rustc_query_system[9c20c97984254eda]::query::plumbing::try_execute_query::<rustc_query_impl[e25e6056fa54423]::queries::analysis, rustc_query_impl[e25e6056fa54423]::plumbing::QueryCtxt>
  51:     0x7fc90ec3b50a - <rustc_query_impl[e25e6056fa54423]::Queries as rustc_middle[bd03a598433729a4]::ty::query::QueryEngine>::analysis
  52:     0x7fc90e94ce79 - <rustc_middle[bd03a598433729a4]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[cf5be2d75a70ff02]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>>
  53:     0x7fc90e4c6a19 - <rustc_interface[e19372daae8a4d61]::interface::Compiler>::enter::<rustc_driver_impl[cf5be2d75a70ff02]::run_compiler::{closure#1}::{closure#2}, core[f0a84df7292b9247]::result::Result<core[f0a84df7292b9247]::option::Option<rustc_interface[e19372daae8a4d61]::queries::Linker>, rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>>
  54:     0x7fc90e4c4b04 - rustc_span[ff2b83d94cd5766c]::with_source_map::<core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>, rustc_interface[e19372daae8a4d61]::interface::run_compiler<core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>, rustc_driver_impl[cf5be2d75a70ff02]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  55:     0x7fc90e4c4614 - <scoped_tls[94748c71a964531c]::ScopedKey<rustc_span[ff2b83d94cd5766c]::SessionGlobals>>::set::<rustc_interface[e19372daae8a4d61]::interface::run_compiler<core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>, rustc_driver_impl[cf5be2d75a70ff02]::run_compiler::{closure#1}>::{closure#0}, core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>>
  56:     0x7fc90e4c3d12 - std[f1e96740c585910a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e19372daae8a4d61]::util::run_in_thread_pool_with_globals<rustc_interface[e19372daae8a4d61]::interface::run_compiler<core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>, rustc_driver_impl[cf5be2d75a70ff02]::run_compiler::{closure#1}>::{closure#0}, core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>>
  57:     0x7fc90e4c3abc - <<std[f1e96740c585910a]::thread::Builder>::spawn_unchecked_<rustc_interface[e19372daae8a4d61]::util::run_in_thread_pool_with_globals<rustc_interface[e19372daae8a4d61]::interface::run_compiler<core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>, rustc_driver_impl[cf5be2d75a70ff02]::run_compiler::{closure#1}>::{closure#0}, core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f0a84df7292b9247]::result::Result<(), rustc_errors[580ee8295c6eeddb]::ErrorGuaranteed>>::{closure#1} as core[f0a84df7292b9247]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7fc90bf77d23 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h6f3268749d03f717
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/alloc/src/boxed.rs:1988:9
  59:     0x7fc90bf77d23 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he030b34c5a5b9772
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/alloc/src/boxed.rs:1988:9
  60:     0x7fc90bf77d23 - std::sys::unix::thread::Thread::new::thread_start::h3f952212e35a76cc
                               at /rustc/2d14db321b043ffc579a7461464c88d7e3f54f83/library/std/src/sys/unix/thread.rs:108:17
  61:     0x7fc90bd07bb5 - <unknown>
  62:     0x7fc90bd89d90 - <unknown>
  63:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (2d14db321 2023-02-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_op_normalize_ty] normalizing `[async fn body@src/main.rs:43:69: 45:14]`
#1 [mir_borrowck] borrow-checking `<impl at src/main.rs:26:9: 39:58>::try_build_or_recover::{closure#0}`
#2 [mir_borrowck] borrow-checking `<impl at src/main.rs:26:9: 39:58>::try_build_or_recover`
#3 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `scratch`

