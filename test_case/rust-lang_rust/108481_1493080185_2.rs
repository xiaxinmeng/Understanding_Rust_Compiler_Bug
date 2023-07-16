
error: internal compiler error: encountered incremental compilation error with def_span(all_is_cubes[456c]::{impl#0}::{constant#0})
  |
  = help: This is a known issue with the compiler. Run `cargo clean -p all_is_cubes` or `cargo clean` to allow your project to compile
  = note: Please follow the instructions below to create a bug report with the provided information
  = note: See <https://github.com/rust-lang/rust/issues/84970> for more information

thread 'rustc' panicked at 'Found unstable fingerprints for def_span(all_is_cubes[456c]::{impl#0}::{constant#0}): src/lib.rs:3:53: 4:6 (#0)', /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/compiler/rustc_query_system/src/query/plumbing.rs:710:9
stack backtrace:
   0:     0x7f2501bbdf8a - std::backtrace_rs::backtrace::libunwind::trace::h2f1b14d2b9e4c6b4
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f2501bbdf8a - std::backtrace_rs::backtrace::trace_unsynchronized::haf404d9184796f09
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f2501bbdf8a - std::sys_common::backtrace::_print_fmt::h440840d8524e88f8
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f2501bbdf8a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0d5b0c2d97b8992f
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f2501c2205f - core::fmt::write::h63211a72e96d0d74
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/core/src/fmt/mod.rs:1254:17
   5:     0x7f2501bb0bc5 - std::io::Write::write_fmt::h83a6589baaeb8174
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/io/mod.rs:1698:15
   6:     0x7f2501bbdd55 - std::sys_common::backtrace::_print::h188e467357108441
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f2501bbdd55 - std::sys_common::backtrace::print::ha9da41b42961a865
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f2501bc09fe - std::panicking::default_hook::{{closure}}::h3fe2eceaaddaab65
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:269:22
   9:     0x7f2501bc07a5 - std::panicking::default_hook::h0c19eee30e1bb21a
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:288:9
  10:     0x7f2504ec34d5 - <rustc_driver_impl[be27ae895c9c1abf]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[633911572d8bda32]::ops::function::FnOnce<(&core[633911572d8bda32]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f2501bc11f4 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc01893d2c4f83c85
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/alloc/src/boxed.rs:1990:9
  12:     0x7f2501bc11f4 - std::panicking::rust_panic_with_hook::hce3464c8b95f72d6
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:694:13
  13:     0x7f2501bc0f69 - std::panicking::begin_panic_handler::{{closure}}::h8b73e8ac2e39f738
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:581:13
  14:     0x7f2501bbe3f6 - std::sys_common::backtrace::__rust_end_short_backtrace::h0437dc11b8a6708e
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7f2501bc0cc2 - rust_begin_unwind
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:577:5
  16:     0x7f2501c1e373 - core::panicking::panic_fmt::h27c352b46a87a69b
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/core/src/panicking.rs:67:14
  17:     0x7f2505759fcc - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::incremental_verify_ich_failed::<rustc_middle[2e01d9c02d9244ae]::ty::context::TyCtxt>
  18:     0x7f25037d635a - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::def_span, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#1}, core[633911572d8bda32]::option::Option<(rustc_span[9d027ba2263b9410]::span_encoding::Span, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core[633911572d8bda32]::option::Option<(rustc_span[9d027ba2263b9410]::span_encoding::Span, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>>
  19:     0x7f25037d48c7 - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::def_span, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  20:     0x7f2502e1f506 - rustc_ty_utils[43839840c81ed713]::ty::param_env
  21:     0x7f2502e22676 - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::param_env, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#2}, (rustc_middle[2e01d9c02d9244ae]::ty::ParamEnv, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle[2e01d9c02d9244ae]::ty::ParamEnv, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>
  22:     0x7f2502e1dfba - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::param_env, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  23:     0x7f2502e1d766 - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::param_env
  24:     0x7f250435e5b0 - rustc_hir_typeck[1db604f76e28b20e]::typeck_const_arg
  25:     0x7f25042f3b62 - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::typeck_const_arg, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#2}, (&rustc_middle[2e01d9c02d9244ae]::ty::typeck_results::TypeckResults, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (&rustc_middle[2e01d9c02d9244ae]::ty::typeck_results::TypeckResults, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>
  26:     0x7f25042f30df - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::typeck_const_arg, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  27:     0x7f25042f2ad4 - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::typeck_const_arg
  28:     0x7f2503f76384 - rustc_hir_typeck[1db604f76e28b20e]::typeck
  29:     0x7f2502fe6592 - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::typeck, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#2}, (&rustc_middle[2e01d9c02d9244ae]::ty::typeck_results::TypeckResults, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (&rustc_middle[2e01d9c02d9244ae]::ty::typeck_results::TypeckResults, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>
  30:     0x7f2502fdb290 - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::typeck, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  31:     0x7f25045fbd3b - rustc_data_structures[6ef3f1b9ef542b16]::sync::par_for_each_in::<&[rustc_span[9d027ba2263b9410]::def_id::LocalDefId], <rustc_middle[2e01d9c02d9244ae]::hir::map::Map>::par_body_owners<rustc_hir_typeck[1db604f76e28b20e]::typeck_item_bodies::{closure#0}>::{closure#0}>
  32:     0x7f25045fbb0f - rustc_hir_typeck[1db604f76e28b20e]::typeck_item_bodies
  33:     0x7f250467006d - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::typeck_item_bodies, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#2}, ((), rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>
  34:     0x7f250466f82b - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::typeck_item_bodies, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  35:     0x7f250466f34c - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::typeck_item_bodies
  36:     0x7f25042ef41f - <rustc_session[2115a9ee57a65798]::session::Session>::time::<(), rustc_hir_analysis[d8584a28d0f23821]::check_crate::{closure#7}>
  37:     0x7f25042ed0c7 - rustc_hir_analysis[d8584a28d0f23821]::check_crate
  38:     0x7f25042e7211 - rustc_interface[fa1cd19e003fbde7]::passes::analysis
  39:     0x7f250466d23a - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::analysis, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#2}, (core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>
  40:     0x7f250466cd8f - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::analysis, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  41:     0x7f250466c8f0 - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::analysis
  42:     0x7f25040ee54e - <rustc_interface[fa1cd19e003fbde7]::queries::QueryResult<&rustc_middle[2e01d9c02d9244ae]::ty::context::GlobalCtxt>>::enter::<core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_driver_impl[be27ae895c9c1abf]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  43:     0x7f25040ed02e - <rustc_interface[fa1cd19e003fbde7]::interface::Compiler>::enter::<rustc_driver_impl[be27ae895c9c1abf]::run_compiler::{closure#1}::{closure#2}, core[633911572d8bda32]::result::Result<core[633911572d8bda32]::option::Option<rustc_interface[fa1cd19e003fbde7]::queries::Linker>, rustc_span[9d027ba2263b9410]::ErrorGuaranteed>>
  44:     0x7f25040eb1e1 - rustc_span[9d027ba2263b9410]::set_source_map::<core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_interface[fa1cd19e003fbde7]::interface::run_compiler<core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_driver_impl[be27ae895c9c1abf]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  45:     0x7f25040ea78f - std[5eb8f1edb78db018]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fa1cd19e003fbde7]::util::run_in_thread_pool_with_globals<rustc_interface[fa1cd19e003fbde7]::interface::run_compiler<core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_driver_impl[be27ae895c9c1abf]::run_compiler::{closure#1}>::{closure#0}, core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>>
  46:     0x7f25040ea16e - <<std[5eb8f1edb78db018]::thread::Builder>::spawn_unchecked_<rustc_interface[fa1cd19e003fbde7]::util::run_in_thread_pool_with_globals<rustc_interface[fa1cd19e003fbde7]::interface::run_compiler<core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_driver_impl[be27ae895c9c1abf]::run_compiler::{closure#1}>::{closure#0}, core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>>::{closure#1} as core[633911572d8bda32]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7f2501bcb2d5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf8dc8be554bcb253
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/alloc/src/boxed.rs:1976:9
  48:     0x7f2501bcb2d5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7ce91a4bb123291d
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/alloc/src/boxed.rs:1976:9
  49:     0x7f2501bcb2d5 - std::sys::unix::thread::Thread::new::thread_start::h821bdfe2a1422348
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys/unix/thread.rs:108:17
  50:     0x7f250190db43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  51:     0x7f250199fa00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  52:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (5e1d3299a 2023-03-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C linker=clang -C incremental=[REDACTED] -Z incremental-verify-ich

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/compiler/rustc_query_system/src/query/job.rs:47:24
stack backtrace:
   0:     0x7f2501bbdf8a - std::backtrace_rs::backtrace::libunwind::trace::h2f1b14d2b9e4c6b4
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f2501bbdf8a - std::backtrace_rs::backtrace::trace_unsynchronized::haf404d9184796f09
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f2501bbdf8a - std::sys_common::backtrace::_print_fmt::h440840d8524e88f8
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f2501bbdf8a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0d5b0c2d97b8992f
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f2501c2205f - core::fmt::write::h63211a72e96d0d74
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/core/src/fmt/mod.rs:1254:17
   5:     0x7f2501bb0bc5 - std::io::Write::write_fmt::h83a6589baaeb8174
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/io/mod.rs:1698:15
   6:     0x7f2501bbdd55 - std::sys_common::backtrace::_print::h188e467357108441
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f2501bbdd55 - std::sys_common::backtrace::print::ha9da41b42961a865
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f2501bc09fe - std::panicking::default_hook::{{closure}}::h3fe2eceaaddaab65
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:269:22
   9:     0x7f2501bc07a5 - std::panicking::default_hook::h0c19eee30e1bb21a
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:288:9
  10:     0x7f2504ec34d5 - <rustc_driver_impl[be27ae895c9c1abf]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[633911572d8bda32]::ops::function::FnOnce<(&core[633911572d8bda32]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f2501bc11f4 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc01893d2c4f83c85
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/alloc/src/boxed.rs:1990:9
  12:     0x7f2501bc11f4 - std::panicking::rust_panic_with_hook::hce3464c8b95f72d6
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:694:13
  13:     0x7f2501bc0f22 - std::panicking::begin_panic_handler::{{closure}}::h8b73e8ac2e39f738
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:579:13
  14:     0x7f2501bbe3f6 - std::sys_common::backtrace::__rust_end_short_backtrace::h0437dc11b8a6708e
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7f2501bc0cc2 - rust_begin_unwind
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:577:5
  16:     0x7f2501c1e373 - core::panicking::panic_fmt::h27c352b46a87a69b
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/core/src/panicking.rs:67:14
  17:     0x7f2501c1e40d - core::panicking::panic::h263133906bfb9076
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/core/src/panicking.rs:117:5
  18:     0x7f25057ab7f3 - <rustc_query_system[bcefc4dc53f73d2]::query::job::QueryJobId>::find_cycle_in_stack::<rustc_middle[2e01d9c02d9244ae]::dep_graph::dep_node::DepKind>
  19:     0x7f25057274fd - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::cycle_error::<rustc_query_impl[a11cf9e729a76afa]::queries::source_span, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  20:     0x7f25037d4f09 - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::def_span, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  21:     0x7f25037d397a - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::def_span
  22:     0x7f25053f30f4 - <rustc_span[9d027ba2263b9410]::def_id::DefId as rustc_middle[2e01d9c02d9244ae]::query::keys::Key>::default_span
  23:     0x7f250583ec1e - rustc_query_impl[a11cf9e729a76afa]::plumbing::create_query_frame::<rustc_span[9d027ba2263b9410]::def_id::DefId>
  24:     0x7f250579cc59 - <rustc_query_impl[a11cf9e729a76afa]::query_structs::param_env::{closure#0}::{closure#0} as core[633911572d8bda32]::ops::function::FnOnce<(rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt, rustc_span[9d027ba2263b9410]::def_id::DefId)>>::call_once
  25:     0x7f250572365c - <rustc_query_system[bcefc4dc53f73d2]::query::plumbing::QueryState<rustc_span[9d027ba2263b9410]::def_id::DefId, rustc_middle[2e01d9c02d9244ae]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  26:     0x7f250581a232 - <rustc_query_impl[a11cf9e729a76afa]::Queries>::try_collect_active_jobs
  27:     0x7f25057274b4 - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::cycle_error::<rustc_query_impl[a11cf9e729a76afa]::queries::source_span, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  28:     0x7f25037d4f09 - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::def_span, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  29:     0x7f25037d397a - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::def_span
  30:     0x7f25053f30f4 - <rustc_span[9d027ba2263b9410]::def_id::DefId as rustc_middle[2e01d9c02d9244ae]::query::keys::Key>::default_span
  31:     0x7f250584084f - rustc_query_impl[a11cf9e729a76afa]::plumbing::create_query_frame::<(rustc_span[9d027ba2263b9410]::def_id::LocalDefId, rustc_span[9d027ba2263b9410]::def_id::DefId)>
  32:     0x7f2505799db4 - <rustc_query_impl[a11cf9e729a76afa]::query_structs::typeck_const_arg::{closure#0}::{closure#0} as core[633911572d8bda32]::ops::function::FnOnce<(rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt, (rustc_span[9d027ba2263b9410]::def_id::LocalDefId, rustc_span[9d027ba2263b9410]::def_id::DefId))>>::call_once
  33:     0x7f2505724f10 - <rustc_query_system[bcefc4dc53f73d2]::query::plumbing::QueryState<(rustc_span[9d027ba2263b9410]::def_id::LocalDefId, rustc_span[9d027ba2263b9410]::def_id::DefId), rustc_middle[2e01d9c02d9244ae]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  34:     0x7f250581a232 - <rustc_query_impl[a11cf9e729a76afa]::Queries>::try_collect_active_jobs
  35:     0x7f25057274b4 - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::cycle_error::<rustc_query_impl[a11cf9e729a76afa]::queries::source_span, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  36:     0x7f25037d4f09 - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::def_span, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  37:     0x7f25037d397a - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::def_span
  38:     0x7f25053f30f4 - <rustc_span[9d027ba2263b9410]::def_id::DefId as rustc_middle[2e01d9c02d9244ae]::query::keys::Key>::default_span
  39:     0x7f250583e8af - rustc_query_impl[a11cf9e729a76afa]::plumbing::create_query_frame::<rustc_hir[d510e89e73515c9b]::hir_id::OwnerId>
  40:     0x7f250579c833 - <rustc_query_impl[a11cf9e729a76afa]::query_structs::typeck::{closure#0}::{closure#0} as core[633911572d8bda32]::ops::function::FnOnce<(rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt, rustc_span[9d027ba2263b9410]::def_id::LocalDefId)>>::call_once
  41:     0x7f25057234f1 - <rustc_query_system[bcefc4dc53f73d2]::query::plumbing::QueryState<rustc_span[9d027ba2263b9410]::def_id::LocalDefId, rustc_middle[2e01d9c02d9244ae]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  42:     0x7f250581a232 - <rustc_query_impl[a11cf9e729a76afa]::Queries>::try_collect_active_jobs
  43:     0x7f25057ad057 - rustc_query_system[bcefc4dc53f73d2]::query::job::print_query_stack::<rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  44:     0x7f2505274c98 - rustc_interface[fa1cd19e003fbde7]::interface::try_print_query_stack
  45:     0x7f2504ec58fd - rustc_driver_impl[be27ae895c9c1abf]::report_ice
  46:     0x7f2504ec351c - <rustc_driver_impl[be27ae895c9c1abf]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[633911572d8bda32]::ops::function::FnOnce<(&core[633911572d8bda32]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  47:     0x7f2501bc11f4 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc01893d2c4f83c85
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/alloc/src/boxed.rs:1990:9
  48:     0x7f2501bc11f4 - std::panicking::rust_panic_with_hook::hce3464c8b95f72d6
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:694:13
  49:     0x7f2501bc0f69 - std::panicking::begin_panic_handler::{{closure}}::h8b73e8ac2e39f738
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:581:13
  50:     0x7f2501bbe3f6 - std::sys_common::backtrace::__rust_end_short_backtrace::h0437dc11b8a6708e
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys_common/backtrace.rs:150:18
  51:     0x7f2501bc0cc2 - rust_begin_unwind
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/panicking.rs:577:5
  52:     0x7f2501c1e373 - core::panicking::panic_fmt::h27c352b46a87a69b
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/core/src/panicking.rs:67:14
  53:     0x7f2505759fcc - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::incremental_verify_ich_failed::<rustc_middle[2e01d9c02d9244ae]::ty::context::TyCtxt>
  54:     0x7f25037d635a - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::def_span, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#1}, core[633911572d8bda32]::option::Option<(rustc_span[9d027ba2263b9410]::span_encoding::Span, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core[633911572d8bda32]::option::Option<(rustc_span[9d027ba2263b9410]::span_encoding::Span, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>>
  55:     0x7f25037d48c7 - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::def_span, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  56:     0x7f2502e1f506 - rustc_ty_utils[43839840c81ed713]::ty::param_env
  57:     0x7f2502e22676 - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::param_env, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#2}, (rustc_middle[2e01d9c02d9244ae]::ty::ParamEnv, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle[2e01d9c02d9244ae]::ty::ParamEnv, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>
  58:     0x7f2502e1dfba - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::param_env, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  59:     0x7f2502e1d766 - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::param_env
  60:     0x7f250435e5b0 - rustc_hir_typeck[1db604f76e28b20e]::typeck_const_arg
  61:     0x7f25042f3b62 - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::typeck_const_arg, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#2}, (&rustc_middle[2e01d9c02d9244ae]::ty::typeck_results::TypeckResults, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (&rustc_middle[2e01d9c02d9244ae]::ty::typeck_results::TypeckResults, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>
  62:     0x7f25042f30df - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::typeck_const_arg, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  63:     0x7f25042f2ad4 - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::typeck_const_arg
  64:     0x7f2503f76384 - rustc_hir_typeck[1db604f76e28b20e]::typeck
  65:     0x7f2502fe6592 - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::typeck, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#2}, (&rustc_middle[2e01d9c02d9244ae]::ty::typeck_results::TypeckResults, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (&rustc_middle[2e01d9c02d9244ae]::ty::typeck_results::TypeckResults, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>
  66:     0x7f2502fdb290 - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::typeck, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  67:     0x7f25045fbd3b - rustc_data_structures[6ef3f1b9ef542b16]::sync::par_for_each_in::<&[rustc_span[9d027ba2263b9410]::def_id::LocalDefId], <rustc_middle[2e01d9c02d9244ae]::hir::map::Map>::par_body_owners<rustc_hir_typeck[1db604f76e28b20e]::typeck_item_bodies::{closure#0}>::{closure#0}>
  68:     0x7f25045fbb0f - rustc_hir_typeck[1db604f76e28b20e]::typeck_item_bodies
  69:     0x7f250467006d - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::typeck_item_bodies, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#2}, ((), rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>
  70:     0x7f250466f82b - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::typeck_item_bodies, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  71:     0x7f250466f34c - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::typeck_item_bodies
  72:     0x7f25042ef41f - <rustc_session[2115a9ee57a65798]::session::Session>::time::<(), rustc_hir_analysis[d8584a28d0f23821]::check_crate::{closure#7}>
  73:     0x7f25042ed0c7 - rustc_hir_analysis[d8584a28d0f23821]::check_crate
  74:     0x7f25042e7211 - rustc_interface[fa1cd19e003fbde7]::passes::analysis
  75:     0x7f250466d23a - <std[5eb8f1edb78db018]::thread::local::LocalKey<core[633911572d8bda32]::cell::Cell<*const ()>>>::with::<rustc_middle[2e01d9c02d9244ae]::ty::context::tls::enter_context<rustc_query_system[bcefc4dc53f73d2]::query::plumbing::execute_job_incr<rustc_query_impl[a11cf9e729a76afa]::queries::analysis, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>::{closure#2}, (core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_query_system[bcefc4dc53f73d2]::dep_graph::graph::DepNodeIndex)>
  76:     0x7f250466cd8f - rustc_query_system[bcefc4dc53f73d2]::query::plumbing::try_execute_query::<rustc_query_impl[a11cf9e729a76afa]::queries::analysis, rustc_query_impl[a11cf9e729a76afa]::plumbing::QueryCtxt>
  77:     0x7f250466c8f0 - <rustc_query_impl[a11cf9e729a76afa]::Queries as rustc_middle[2e01d9c02d9244ae]::ty::query::QueryEngine>::analysis
  78:     0x7f25040ee54e - <rustc_interface[fa1cd19e003fbde7]::queries::QueryResult<&rustc_middle[2e01d9c02d9244ae]::ty::context::GlobalCtxt>>::enter::<core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_driver_impl[be27ae895c9c1abf]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  79:     0x7f25040ed02e - <rustc_interface[fa1cd19e003fbde7]::interface::Compiler>::enter::<rustc_driver_impl[be27ae895c9c1abf]::run_compiler::{closure#1}::{closure#2}, core[633911572d8bda32]::result::Result<core[633911572d8bda32]::option::Option<rustc_interface[fa1cd19e003fbde7]::queries::Linker>, rustc_span[9d027ba2263b9410]::ErrorGuaranteed>>
  80:     0x7f25040eb1e1 - rustc_span[9d027ba2263b9410]::set_source_map::<core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_interface[fa1cd19e003fbde7]::interface::run_compiler<core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_driver_impl[be27ae895c9c1abf]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  81:     0x7f25040ea78f - std[5eb8f1edb78db018]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fa1cd19e003fbde7]::util::run_in_thread_pool_with_globals<rustc_interface[fa1cd19e003fbde7]::interface::run_compiler<core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_driver_impl[be27ae895c9c1abf]::run_compiler::{closure#1}>::{closure#0}, core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>>
  82:     0x7f25040ea16e - <<std[5eb8f1edb78db018]::thread::Builder>::spawn_unchecked_<rustc_interface[fa1cd19e003fbde7]::util::run_in_thread_pool_with_globals<rustc_interface[fa1cd19e003fbde7]::interface::run_compiler<core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>, rustc_driver_impl[be27ae895c9c1abf]::run_compiler::{closure#1}>::{closure#0}, core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[633911572d8bda32]::result::Result<(), rustc_span[9d027ba2263b9410]::ErrorGuaranteed>>::{closure#1} as core[633911572d8bda32]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  83:     0x7f2501bcb2d5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf8dc8be554bcb253
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/alloc/src/boxed.rs:1976:9
  84:     0x7f2501bcb2d5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7ce91a4bb123291d
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/alloc/src/boxed.rs:1976:9
  85:     0x7f2501bcb2d5 - std::sys::unix::thread::Thread::new::thread_start::h821bdfe2a1422348
                               at /rustc/5e1d3299a290026b85787bc9c7e72bcc53ac283f/library/std/src/sys/unix/thread.rs:108:17
  86:     0x7f250190db43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  87:     0x7f250199fa00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  88:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (5e1d3299a 2023-03-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C linker=clang -C incremental=[REDACTED] -Z incremental-verify-ich

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [def_span] looking up span for `<impl at src/lib.rs:3:10: 3:15>::{constant#0}`
end of query stack
thread panicked while panicking. aborting.
error: could not compile `all-is-cubes` (lib) due to previous error

Caused by:
  process didn't exit successfully: `rustc --crate-name all_is_cubes --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=209 --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=ce9a0cbf1d21f99c -C extra-filename=-ce9a0cbf1d21f99c --out-dir /home/nilsh/other/all-is-cubes/target/debug/deps -C linker=clang -C incremental=/home/nilsh/other/all-is-cubes/target/debug/incremental -L dependency=/home/nilsh/other/all-is-cubes/target/debug/deps -Zincremental-verify-ich` (signal: 6, SIGABRT: process abort signal)
