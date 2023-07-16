
   Compiling report-bug v0.1.0 (/home/tfukaya/codes/report-bug)
error[E0107]: this function takes 2 generic arguments but 1 generic argument was supplied
  --> src/main.rs:32:13
   |
32 |     let b = add::<i32>(x, y); // ICE
   |             ^^^   --- supplied 1 generic argument
   |             |
   |             expected 2 generic arguments
   |
note: function defined here, with 2 generic parameters: `T`, `U`
  --> src/main.rs:20:4
   |
20 | fn add<T, U>(x: T, y: T) -> T
   |    ^^^ -  -
help: add missing generic argument
   |
32 |     let b = add::<i32, U>(x, y); // ICE
   |                      +++

error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:853:17: ErrorReporting Overflow should not reach `report_selection_err` call

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
stack backtrace:
   0:     0x7f1f6239ee8c - std::backtrace_rs::backtrace::libunwind::trace::h09f7e4e089375279
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f1f6239ee8c - std::backtrace_rs::backtrace::trace_unsynchronized::h1ec96f1c7087094e
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f1f6239ee8c - std::sys_common::backtrace::_print_fmt::h317b71fc9a5cf964
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f1f6239ee8c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he3555b48e7dfe7f0
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f1f623fc55c - core::fmt::write::h513b07ca38f4fb1b
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/fmt/mod.rs:1149:17
   5:     0x7f1f6238f0d5 - std::io::Write::write_fmt::haf8c932b52111354
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/io/mod.rs:1697:15
   6:     0x7f1f623a1f30 - std::sys_common::backtrace::_print::h195c38364780a303
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f1f623a1f30 - std::sys_common::backtrace::print::hc09dfdea923b6730
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f1f623a1f30 - std::panicking::default_hook::{{closure}}::hb2e38ec0d91046a3
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/panicking.rs:211:50
   9:     0x7f1f623a1ae5 - std::panicking::default_hook::h60284635b0ad54a8
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/panicking.rs:228:9
  10:     0x7f1f62b4baa1 - rustc_driver[6ad7101a8fabd86e]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f1f623a2749 - std::panicking::rust_panic_with_hook::ha677a669fb275654
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/panicking.rs:610:17
  12:     0x7f1f63c3dfcb - std[b2a2bae5f6ca25d7]::panicking::begin_panic::<rustc_errors[5befc41d352b3f14]::ExplicitBug>::{closure#0}
  13:     0x7f1f63c3df66 - std[b2a2bae5f6ca25d7]::sys_common::backtrace::__rust_end_short_backtrace::<std[b2a2bae5f6ca25d7]::panicking::begin_panic<rustc_errors[5befc41d352b3f14]::ExplicitBug>::{closure#0}, !>
  14:     0x7f1f63c4135f - std[b2a2bae5f6ca25d7]::panicking::begin_panic::<rustc_errors[5befc41d352b3f14]::ExplicitBug>
  15:     0x7f1f63c4c4ad - std[b2a2bae5f6ca25d7]::panic::panic_any::<rustc_errors[5befc41d352b3f14]::ExplicitBug>
  16:     0x7f1f63c4e140 - <rustc_errors[5befc41d352b3f14]::HandlerInner>::bug
  17:     0x7f1f63c4d960 - <rustc_errors[5befc41d352b3f14]::Handler>::bug
  18:     0x7f1f63ab701e - rustc_middle[26370726012d1bf2]::ty::context::tls::with_opt::<rustc_middle[26370726012d1bf2]::util::bug::opt_span_bug_fmt<rustc_span[c29ad455a793da3]::span_encoding::Span>::{closure#0}, ()>
  19:     0x7f1f63ab75e0 - rustc_middle[26370726012d1bf2]::util::bug::opt_span_bug_fmt::<rustc_span[c29ad455a793da3]::span_encoding::Span>
  20:     0x7f1f63ab7556 - rustc_middle[26370726012d1bf2]::util::bug::bug_fmt
  21:     0x7f1f63966415 - <rustc_infer[c3769667aa5ba053]::infer::InferCtxt as rustc_trait_selection[e183c0ec72eef9d2]::traits::error_reporting::InferCtxtExt>::report_selection_error
  22:     0x7f1f6396be97 - <rustc_infer[c3769667aa5ba053]::infer::InferCtxt as rustc_trait_selection[e183c0ec72eef9d2]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  23:     0x7f1f6396193b - <rustc_infer[c3769667aa5ba053]::infer::InferCtxt as rustc_trait_selection[e183c0ec72eef9d2]::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
  24:     0x7f1f63ebd4fd - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  25:     0x7f1f63ebff9d - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::structurally_resolved_type
  26:     0x7f1f63eb0c2f - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::check_call
  27:     0x7f1f63ecda95 - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  28:     0x7f1f63eb735b - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:     0x7f1f6498c049 - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::check_decl_initializer
  30:     0x7f1f63ec2b47 - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::check_decl_local
  31:     0x7f1f63ec2e04 - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::check_stmt
  32:     0x7f1f63ec3a61 - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  33:     0x7f1f63eb735b - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7f1f63eb83d7 - <rustc_typeck[17f4c348c1653d0a]::check::fn_ctxt::FnCtxt>::check_return_expr
  35:     0x7f1f63f8fd00 - rustc_typeck[17f4c348c1653d0a]::check::check::check_fn
  36:     0x7f1f63f376bd - <rustc_infer[c3769667aa5ba053]::infer::InferCtxtBuilder>::enter::<&rustc_middle[26370726012d1bf2]::ty::context::TypeckResults, <rustc_typeck[17f4c348c1653d0a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[17f4c348c1653d0a]::check::typeck_with_fallback<rustc_typeck[17f4c348c1653d0a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[26370726012d1bf2]::ty::context::TypeckResults>::{closure#0}>
  37:     0x7f1f63f0e953 - rustc_typeck[17f4c348c1653d0a]::check::typeck
  38:     0x7f1f64d364c3 - <rustc_query_system[3616b1a045b1904c]::dep_graph::graph::DepGraph<rustc_middle[26370726012d1bf2]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[26370726012d1bf2]::ty::context::TyCtxt, rustc_span[c29ad455a793da3]::def_id::LocalDefId, &rustc_middle[26370726012d1bf2]::ty::context::TypeckResults>
  39:     0x7f1f64cbca04 - rustc_data_structures[a6a643d2c4658522]::stack::ensure_sufficient_stack::<(&rustc_middle[26370726012d1bf2]::ty::context::TypeckResults, rustc_query_system[3616b1a045b1904c]::dep_graph::graph::DepNodeIndex), rustc_query_system[3616b1a045b1904c]::query::plumbing::execute_job<rustc_query_impl[4fd9530611cf4d08]::plumbing::QueryCtxt, rustc_span[c29ad455a793da3]::def_id::LocalDefId, &rustc_middle[26370726012d1bf2]::ty::context::TypeckResults>::{closure#3}>
  40:     0x7f1f642a752c - rustc_query_system[3616b1a045b1904c]::query::plumbing::try_execute_query::<rustc_query_impl[4fd9530611cf4d08]::plumbing::QueryCtxt, rustc_query_system[3616b1a045b1904c]::query::caches::DefaultCache<rustc_span[c29ad455a793da3]::def_id::LocalDefId, &rustc_middle[26370726012d1bf2]::ty::context::TypeckResults>>
  41:     0x7f1f6430e30c - <rustc_query_impl[4fd9530611cf4d08]::Queries as rustc_middle[26370726012d1bf2]::ty::query::QueryEngine>::typeck
  42:     0x7f1f63f64712 - <rustc_middle[26370726012d1bf2]::hir::map::Map>::par_body_owners::<rustc_typeck[17f4c348c1653d0a]::check::typeck_item_bodies::{closure#0}>
  43:     0x7f1f649c755c - rustc_typeck[17f4c348c1653d0a]::check::typeck_item_bodies
  44:     0x7f1f64d58a54 - <rustc_query_system[3616b1a045b1904c]::dep_graph::graph::DepGraph<rustc_middle[26370726012d1bf2]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[26370726012d1bf2]::ty::context::TyCtxt, (), ()>
  45:     0x7f1f64c2c46f - rustc_query_system[3616b1a045b1904c]::query::plumbing::try_execute_query::<rustc_query_impl[4fd9530611cf4d08]::plumbing::QueryCtxt, rustc_query_system[3616b1a045b1904c]::query::caches::DefaultCache<(), ()>>
  46:     0x7f1f64cd0c48 - <rustc_query_impl[4fd9530611cf4d08]::Queries as rustc_middle[26370726012d1bf2]::ty::query::QueryEngine>::typeck_item_bodies
  47:     0x7f1f649bc25f - <rustc_session[f32185a6c19f3ee0]::session::Session>::time::<(), rustc_typeck[17f4c348c1653d0a]::check_crate::{closure#7}>
  48:     0x7f1f649b9639 - rustc_typeck[17f4c348c1653d0a]::check_crate
  49:     0x7f1f647218e0 - rustc_interface[2c1d684caca70d53]::passes::analysis
  50:     0x7f1f64d53bb0 - <rustc_query_system[3616b1a045b1904c]::dep_graph::graph::DepGraph<rustc_middle[26370726012d1bf2]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[26370726012d1bf2]::ty::context::TyCtxt, (), core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>>
  51:     0x7f1f64cb7bd3 - rustc_data_structures[a6a643d2c4658522]::stack::ensure_sufficient_stack::<(core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>, rustc_query_system[3616b1a045b1904c]::dep_graph::graph::DepNodeIndex), rustc_query_system[3616b1a045b1904c]::query::plumbing::execute_job<rustc_query_impl[4fd9530611cf4d08]::plumbing::QueryCtxt, (), core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>>::{closure#3}>
  52:     0x7f1f64c2279c - rustc_query_system[3616b1a045b1904c]::query::plumbing::try_execute_query::<rustc_query_impl[4fd9530611cf4d08]::plumbing::QueryCtxt, rustc_query_system[3616b1a045b1904c]::query::caches::DefaultCache<(), core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>>>
  53:     0x7f1f64cd0265 - <rustc_query_impl[4fd9530611cf4d08]::Queries as rustc_middle[26370726012d1bf2]::ty::query::QueryEngine>::analysis
  54:     0x7f1f64715699 - <rustc_interface[2c1d684caca70d53]::passes::QueryContext>::enter::<rustc_driver[6ad7101a8fabd86e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>>
  55:     0x7f1f646ecfd7 - <rustc_interface[2c1d684caca70d53]::interface::Compiler>::enter::<rustc_driver[6ad7101a8fabd86e]::run_compiler::{closure#1}::{closure#2}, core[7435606f1c80df2a]::result::Result<core[7435606f1c80df2a]::option::Option<rustc_interface[2c1d684caca70d53]::queries::Linker>, rustc_errors[5befc41d352b3f14]::ErrorReported>>
  56:     0x7f1f646e8ed4 - rustc_span[c29ad455a793da3]::with_source_map::<core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>, rustc_interface[2c1d684caca70d53]::interface::create_compiler_and_run<core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>, rustc_driver[6ad7101a8fabd86e]::run_compiler::{closure#1}>::{closure#1}>
  57:     0x7f1f646ede2f - <scoped_tls[66f88778b59ce329]::ScopedKey<rustc_span[c29ad455a793da3]::SessionGlobals>>::set::<rustc_interface[2c1d684caca70d53]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[2c1d684caca70d53]::interface::run_compiler<core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>, rustc_driver[6ad7101a8fabd86e]::run_compiler::{closure#1}>::{closure#0}, core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>>::{closure#0}::{closure#0}, core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>>
  58:     0x7f1f646ebf65 - std[b2a2bae5f6ca25d7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2c1d684caca70d53]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[2c1d684caca70d53]::interface::run_compiler<core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>, rustc_driver[6ad7101a8fabd86e]::run_compiler::{closure#1}>::{closure#0}, core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>>::{closure#0}, core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>>
  59:     0x7f1f6471615a - <<std[b2a2bae5f6ca25d7]::thread::Builder>::spawn_unchecked<rustc_interface[2c1d684caca70d53]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[2c1d684caca70d53]::interface::run_compiler<core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>, rustc_driver[6ad7101a8fabd86e]::run_compiler::{closure#1}>::{closure#0}, core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>>::{closure#0}, core[7435606f1c80df2a]::result::Result<(), rustc_errors[5befc41d352b3f14]::ErrorReported>>::{closure#1} as core[7435606f1c80df2a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  60:     0x7f1f623adfc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hcbc6d2d80772be64
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/alloc/src/boxed.rs:1694:9
  61:     0x7f1f623adfc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9bffa2ca65a1d6e6
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/alloc/src/boxed.rs:1694:9
  62:     0x7f1f623adfc3 - std::sys::unix::thread::Thread::new::thread_start::ha678a8b0caec8f55
                               at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/sys/unix/thread.rs:106:17
  63:     0x7f1f620eb6db - start_thread
                               at /build/glibc-S9d2JN/glibc-2.27/nptl/pthread_create.c:463
  64:     0x7f1f61a0871f - __GI___clone
                               at /build/glibc-S9d2JN/glibc-2.27/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  65:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.1 (db9d1b20b 2022-01-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
For more information about this error, try `rustc --explain E0107`.
error: could not compile `report-bug` due to previous error
