`

error: crate `diesel` required to be available in rlib format, but was not found in this form

error: crate `tokio` required to be available in rlib format, but was not found in this form

error: crate `serde` required to be available in rlib format, but was not found in this form

error: crate `anyhow` required to be available in rlib format, but was not found in this form

error: crate `serde_with` required to be available in rlib format, but was not found in this form

error: crate `zip` required to be available in rlib format, but was not found in this form

error: crate `glob` required to be available in rlib format, but was not found in this form

error: crate `chrono` required to be available in rlib format, but was not found in this form

error: crate `fern` required to be available in rlib format, but was not found in this form

error: crate `regex` required to be available in rlib format, but was not found in this form

error: crate `bytesize` required to be available in rlib format, but was not found in this form

error: crate `reqwest` required to be available in rlib format, but was not found in this form

error: crate `tempfile` required to be available in rlib format, but was not found in this form

error: crate `envy` required to be available in rlib format, but was not found in this form

error: crate `dotenv` required to be available in rlib format, but was not found in this form

error: internal compiler error: src/librustc_mir/transform/generator.rs:713: Broken MIR: generator contains type &util::human_version::HumanVersionReq in MIR, but typeck only knows about for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13, 't14> {std::future::ResumeTy, &'r factorio::mods::Mods<P>, &'s mod_common::Mod, std::vec::Vec<std::string::String>, impl std::future::Future, (), std::result::Result<std::vec::Vec<mod_common::dependency::Dependency>, anyhow::Error>, std::vec::Vec<mod_common::dependency::Dependency>, std::vec::IntoIter<mod_common::dependency::Dependency>, mod_common::dependency::Dependency, &'t1 mod_common::dependency::Dependency, mod_common::dependency::Requirement, &'t2 str, std::result::Result<&'t3 mod_common::Mod, anyhow::Error>, std::option::Option<util::human_version::HumanVersionReq>, util::human_version::HumanVersionReq, impl std::future::Future, log::Level, bool, [&'t5 str; 4], &'t6 [&'t7 str], &'t8 [&'t9 str; 4], impl std::future::Future, std::string::String, &'t11 std::string::String, [&'t12 str; 3], &'t13 [&'t14 str; 3]}
   --> crates/modtorio/src/factorio/mods.rs:266:38
    |
266 |       ) -> anyhow::Result<Vec<String>> {
    |  ______________________________________^
267 | |         let mut missing = Vec::new();
268 | |
269 | |         for dep in target_mod.dependencies().await? {
...   |
317 | |         Ok(missing)
318 | |     }
    | |_____^

thread 'rustc' panicked at 'Box<Any>', /rustc/783139bd8fc3b94fac9a1bf81bba2c506e8221b6/src/libstd/macros.rs:13:23
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1076
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: rustc_driver::report_ice
  11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /home/matthias/.rustup/toolchains/master/lib/rustlib/src/rust/src/liballoc/boxed.rs:1071
  12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
             at /home/matthias/.rustup/toolchains/master/lib/rustlib/src/rust/src/libproc_macro/bridge/client.rs:318
  13: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:490
  14: std::panicking::begin_panic
  15: rustc_errors::HandlerInner::span_bug
  16: rustc_errors::Handler::span_bug
  17: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
  18: rustc_middle::ty::context::tls::with_opt::{{closure}}
  19: rustc_middle::ty::context::tls::with_opt
  20: rustc_middle::util::bug::opt_span_bug_fmt
  21: rustc_middle::util::bug::span_bug_fmt
  22: <rustc_mir::transform::generator::StateTransform as rustc_mir::transform::MirPass>::run_pass
  23: rustc_mir::transform::run_passes
  24: rustc_mir::transform::run_optimization_passes
  25: rustc_mir::transform::optimized_mir
  26: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute
  27: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  29: rustc_data_structures::stack::ensure_sufficient_stack
  30: rustc_query_system::query::plumbing::get_query_impl
  31: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
  32: rustc_middle::ty::layout::layout_raw
  33: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::layout_raw>::compute
  34: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  35: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  36: rustc_data_structures::stack::ensure_sufficient_stack
  37: rustc_query_system::query::plumbing::get_query_impl
  38: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  39: <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next
  40: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  41: <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next
  42: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  43: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
  44: rustc_middle::ty::layout::layout_raw
  45: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::layout_raw>::compute
  46: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  47: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  48: rustc_data_structures::stack::ensure_sufficient_stack
  49: rustc_query_system::query::plumbing::get_query_impl
  50: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  51: <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next
  52: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  53: <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next
  54: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  55: core::iter::adapters::process_results
  56: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
  57: rustc_middle::ty::layout::layout_raw
  58: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::layout_raw>::compute
  59: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  60: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  61: rustc_data_structures::stack::ensure_sufficient_stack
  62: rustc_query_system::query::plumbing::get_query_impl
  63: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  64: <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next
  65: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  66: <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next
  67: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  68: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
  69: rustc_middle::ty::layout::layout_raw
  70: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::layout_raw>::compute
  71: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  72: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  73: rustc_data_structures::stack::ensure_sufficient_stack
  74: rustc_query_system::query::plumbing::get_query_impl
  75: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::query::TyCtxtAt> as rustc_target::abi::LayoutOf>::layout_of
  76: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  77: rustc_mir::transform::run_passes
  78: rustc_mir::transform::run_optimization_passes
  79: rustc_mir::transform::optimized_mir
  80: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute
  81: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  82: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  83: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
  84: rustc_query_system::query::plumbing::get_query_impl
  85: rustc_mir::util::pretty::write_mir_pretty
  86: rustc_mir::transform::dump_mir::emit_mir
  87: rustc_interface::passes::start_codegen
  88: rustc_middle::ty::context::tls::enter_global
  89: rustc_interface::queries::Queries::ongoing_codegen
  90: rustc_interface::interface::run_compiler_in_existing_thread_pool
  91: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.45.0-nightly (783139bd8 2020-05-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] processing `factorio::mods::Mods::<P>::ensure_single_dependencies::{{closure}}#0`
rust-lang/rust-clippy#1 [layout_raw] computing layout of `[static generator@crates/modtorio/src/factorio/mods.rs:266:38: 318:6 self:&factorio::mods::Mods<P>, target_mod:&mod_common::Mod for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13, 't14> {std::future::ResumeTy, &'r factorio::mods::Mods<P>, &'s mod_common::Mod, std::vec::Vec<std::string::String>, impl std::future::Future, (), std::result::Result<std::vec::Vec<mod_common::dependency::Dependency>, anyhow::Error>, std::vec::Vec<mod_common::dependency::Dependency>, std::vec::IntoIter<mod_common::dependency::Dependency>, mod_common::dependency::Dependency, &'t1 mod_common::dependency::Dependency, mod_common::dependency::Requirement, &'t2 str, std::result::Result<&'t3 mod_common::Mod, anyhow::Error>, std::option::Option<util::human_version::HumanVersionReq>, util::human_version::HumanVersionReq, impl std::future::Future, log::Level, bool, [&'t5 str; 4], &'t6 [&'t7 str], &'t8 [&'t9 str; 4], impl std::future::Future, std::string::String, &'t11 std::string::String, [&'t12 str; 3], &'t13 [&'t14 str; 3]}]`
rust-lang/rust-clippy#2 [layout_raw] computing layout of `std::future::from_generator::GenFuture<[static generator@crates/modtorio/src/factorio/mods.rs:266:38: 318:6 self:&factorio::mods::Mods<P>, target_mod:&mod_common::Mod for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13, 't14> {std::future::ResumeTy, &'r factorio::mods::Mods<P>, &'s mod_common::Mod, std::vec::Vec<std::string::String>, impl std::future::Future, (), std::result::Result<std::vec::Vec<mod_common::dependency::Dependency>, anyhow::Error>, std::vec::Vec<mod_common::dependency::Dependency>, std::vec::IntoIter<mod_common::dependency::Dependency>, mod_common::dependency::Dependency, &'t1 mod_common::dependency::Dependency, mod_common::dependency::Requirement, &'t2 str, std::result::Result<&'t3 mod_common::Mod, anyhow::Error>, std::option::Option<util::human_version::HumanVersionReq>, util::human_version::HumanVersionReq, impl std::future::Future, log::Level, bool, [&'t5 str; 4], &'t6 [&'t7 str], &'t8 [&'t9 str; 4], impl std::future::Future, std::string::String, &'t11 std::string::String, [&'t12 str; 3], &'t13 [&'t14 str; 3]}]>`
rust-lang/rust-clippy#3 [layout_raw] computing layout of `[static generator@crates/modtorio/src/factorio/mods.rs:185:71: 207:6 self:&mut factorio::mods::Mods<P> for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13, 't14> {std::future::ResumeTy, &'r mut factorio::mods::Mods<P>, std::vec::Vec<std::string::String>, factorio::mods::Mods<P>, &'s std::collections::HashMap<std::string::String, std::sync::Arc<mod_common::Mod>>, std::collections::HashMap<std::string::String, std::sync::Arc<mod_common::Mod>>, std::collections::hash_map::Values<'t0, std::string::String, std::sync::Arc<mod_common::Mod>>, &'t1 std::sync::Arc<mod_common::Mod>, &'t2 mut std::vec::Vec<std::string::String>, &'t3 factorio::mods::Mods<P>, &'t4 mod_common::Mod, impl std::future::Future, (), bool, &'t7 std::vec::Vec<std::string::String>, std::slice::Iter<'t8, std::string::String>, &'t9 std::string::String, &'t10 str, &'t11 &'t12 std::string::String, std::option::Option<util::human_version::HumanVersion>, impl std::future::Future}]`
rust-lang/rust-clippy#4 [layout_raw] computing layout of `std::future::from_generator::GenFuture<[static generator@crates/modtorio/src/factorio/mods.rs:185:71: 207:6 self:&mut factorio::mods::Mods<P> for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13, 't14> {std::future::ResumeTy, &'r mut factorio::mods::Mods<P>, std::vec::Vec<std::string::String>, factorio::mods::Mods<P>, &'s std::collections::HashMap<std::string::String, std::sync::Arc<mod_common::Mod>>, std::collections::HashMap<std::string::String, std::sync::Arc<mod_common::Mod>>, std::collections::hash_map::Values<'t0, std::string::String, std::sync::Arc<mod_common::Mod>>, &'t1 std::sync::Arc<mod_common::Mod>, &'t2 mut std::vec::Vec<std::string::String>, &'t3 factorio::mods::Mods<P>, &'t4 mod_common::Mod, impl std::future::Future, (), bool, &'t7 std::vec::Vec<std::string::String>, std::slice::Iter<'t8, std::string::String>, &'t9 std::string::String, &'t10 str, &'t11 &'t12 std::string::String, std::option::Option<util::human_version::HumanVersion>, impl std::future::Future}]>`
rust-lang/rust-clippy#5 [optimized_mir] processing `factorio::mods::Mods::<P>::ensure_dependencies`
end of query stack
error: aborting due to 16 previous errors; 31 warnings emitted

error: could not compile `modtorio`.

To learn more, run the command again with --verbose.
