plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
---- [ui] tests/ui/attributes/log-backtrace.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: RUSTC_LOG="info" RUSTC_LOG_BACKTRACE="rustc_metadata::creader" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/attributes/log-backtrace.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/log-backtrace/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/log-backtrace/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Unable to get span scope; this is a bug', /cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-tree-0.2.0/src/lib.rs:311:18
stack backtrace:
   0:     0x7f4a76e9caf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hdd4aa28f337f523e
   1:     0x7f4a76f09cc8 - core::fmt::write::h85ffdb57a56da83e
   2:     0x7f4a76e911d1 - std::io::Write::write_fmt::h3a2ae64c65928adf
   3:     0x7f4a76e9c901 - std::sys_common::backtrace::print::h0ec953311f5061d8
   4:     0x7f4a76e9fad4 - std::panicking::default_hook::{{closure}}::h60a0ced62ab604c7
   5:     0x7f4a76e9f7ba - std::panicking::default_hook::h3ad9e1dde3e1e424
   6:     0x7f4a779a4c35 - rustc_driver_impl[6e6e6528accfd63]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f4a76ea01f1 - std::panicking::rust_panic_with_hook::hba2254683d4f2350
   8:     0x7f4a76e9ff69 - std::panicking::begin_panic_handler::{{closure}}::h1bb73a52ba6d86bf
   9:     0x7f4a76e9cfc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h457fa3a8fbf96bd7
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  10:     0x7f4a76e9fc47 - rust_begin_unwind
  11:     0x7f4a76e562c3 - core::panicking::panic_fmt::h09f165c1cec36677
  12:     0x7f4a76f05c71 - core::panicking::panic_display::hb8a27bc80344dd4a
  13:     0x7f4a76f05c1b - core::panicking::panic_str::h0d683924b644cf7e
  14:     0x7f4a76e56286 - core::option::expect_failed::h7416cbc6859a82cf
  15:     0x7f4a77a46424 - <tracing_tree[174f855df6821241]::HierarchicalLayer<std[a5c91af2d15e892]::io::stdio::stderr> as tracing_subscriber[a5d0039778a0dacc]::layer::Layer<tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>>>::on_event
  16:     0x7f4a77a3c59f - <tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::FilterState>::did_enable::<<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_tree[174f855df6821241]::HierarchicalLayer<std[a5c91af2d15e892]::io::stdio::stderr>, tracing_subscriber[a5d0039778a0dacc]::filter::env::EnvFilter, tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>> as tracing_subscriber[a5d0039778a0dacc]::layer::Layer<tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>>>::on_event::{closure#0}>
  17:     0x7f4a77a52cd7 - <std[a5c91af2d15e892]::thread::local::LocalKey<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::FilterState>>::with::<<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_tree[174f855df6821241]::HierarchicalLayer<std[a5c91af2d15e892]::io::stdio::stderr>, tracing_subscriber[a5d0039778a0dacc]::filter::env::EnvFilter, tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>>>::did_enable<<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_tree[174f855df6821241]::HierarchicalLayer<std[a5c91af2d15e892]::io::stdio::stderr>, tracing_subscriber[a5d0039778a0dacc]::filter::env::EnvFilter, tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>> as tracing_subscriber[a5d0039778a0dacc]::layer::Layer<tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>>>::on_event::{closure#0}>::{closure#0}, ()>
  18:     0x7f4a77a3d4ec - <tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_tree[174f855df6821241]::HierarchicalLayer<std[a5c91af2d15e892]::io::stdio::stderr>, tracing_subscriber[a5d0039778a0dacc]::filter::env::EnvFilter, tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>> as tracing_subscriber[a5d0039778a0dacc]::layer::Layer<tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>>>::on_event
  19:     0x7f4a77a3e796 - <tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::fmt::fmt_layer::Layer<tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_tree[174f855df6821241]::HierarchicalLayer<std[a5c91af2d15e892]::io::stdio::stderr>, tracing_subscriber[a5d0039778a0dacc]::filter::env::EnvFilter, tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>>, tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>>, tracing_subscriber[a5d0039778a0dacc]::fmt::format::DefaultFields, rustc_log[1113402437150d85]::BacktraceFormatter, std[a5c91af2d15e892]::io::stdio::stderr>, tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_tree[174f855df6821241]::HierarchicalLayer<std[a5c91af2d15e892]::io::stdio::stderr>, tracing_subscriber[a5d0039778a0dacc]::filter::env::EnvFilter, tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>>, tracing_subscriber[a5d0039778a0dacc]::layer::layered::Layered<tracing_subscriber[a5d0039778a0dacc]::filter::layer_filters::Filtered<tracing_error[a6eb3201f9170c2e]::layer::ErrorLayer<tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::filter::filter_fn::FilterFn<rustc_log[1113402437150d85]::init_env_logger::{closure#0}>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>, tracing_subscriber[a5d0039778a0dacc]::registry::sharded::Registry>>> as tracing_core[56bc781f3098d5b1]::subscriber::Subscriber>::event
  20:     0x7f4a7acd065e - tracing_core[56bc781f3098d5b1]::dispatcher::get_default::<(), <tracing_core[56bc781f3098d5b1]::event::Event>::dispatch::{closure#0}>
  21:     0x7f4a7accdcb1 - <tracing_core[56bc781f3098d5b1]::event::Event>::dispatch
  22:     0x7f4a788fe495 - <rustc_resolve[53e067340479d2a0]::Resolver>::resolve_crate_root
  23:     0x7f4a789733bf - <core[feb76cca35898a1f]::iter::adapters::map::Map<core[feb76cca35898a1f]::ops::range::Range<usize>, rustc_span[1d330a307b171ff]::hygiene::update_dollar_crate_names<<rustc_resolve[53e067340479d2a0]::Resolver as rustc_expand[25d81198ceafc4c6]::base::ResolverExpand>::resolve_dollar_crates::{closure#0}>::{closure#1}> as core[feb76cca35898a1f]::iter::traits::iterator::Iterator>::fold::<(), core[feb76cca35898a1f]::iter::traits::iterator::Iterator::for_each::call<rustc_span[1d330a307b171ff]::symbol::Symbol, <alloc[e09c93b9546bc735]::vec::Vec<rustc_span[1d330a307b171ff]::symbol::Symbol>>::extend_trusted<core[feb76cca35898a1f]::iter::adapters::map::Map<core[feb76cca35898a1f]::ops::range::Range<usize>, rustc_span[1d330a307b171ff]::hygiene::update_dollar_crate_names<<rustc_resolve[53e067340479d2a0]::Resolver as rustc_expand[25d81198ceafc4c6]::base::ResolverExpand>::resolve_dollar_crates::{closure#0}>::{closure#1}>>::{closure#0}>::{closure#0}>
  24:     0x7f4a7892e1cf - <alloc[e09c93b9546bc735]::vec::Vec<rustc_span[1d330a307b171ff]::symbol::Symbol> as alloc[e09c93b9546bc735]::vec::spec_from_iter::SpecFromIter<rustc_span[1d330a307b171ff]::symbol::Symbol, core[feb76cca35898a1f]::iter::adapters::map::Map<core[feb76cca35898a1f]::ops::range::Range<usize>, rustc_span[1d330a307b171ff]::hygiene::update_dollar_crate_names<<rustc_resolve[53e067340479d2a0]::Resolver as rustc_expand[25d81198ceafc4c6]::base::ResolverExpand>::resolve_dollar_crates::{closure#0}>::{closure#1}>>>::from_iter
  25:     0x7f4a788620e3 - rustc_span[1d330a307b171ff]::hygiene::update_dollar_crate_names::<<rustc_resolve[53e067340479d2a0]::Resolver as rustc_expand[25d81198ceafc4c6]::base::ResolverExpand>::resolve_dollar_crates::{closure#0}>
  26:     0x7f4a79da44de - <rustc_expand[25d81198ceafc4c6]::expand::MacroExpander>::collect_invocations
  27:     0x7f4a79d9fe8d - <rustc_expand[25d81198ceafc4c6]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7f4a79d9faff - <rustc_expand[25d81198ceafc4c6]::expand::MacroExpander>::expand_crate
  29:     0x7f4a77aa598f - <rustc_session[73226824cc72d182]::session::Session>::time::<rustc_ast[e7d7a5047d46c716]::ast::Crate, rustc_interface[6219fe1e1c8d4b8]::passes::configure_and_expand::{closure#1}>
  30:     0x7f4a77a7cf3c - rustc_interface[6219fe1e1c8d4b8]::passes::resolver_for_lowering
  31:     0x7f4a7979ff39 - rustc_query_system[ec0f893703c89270]::query::plumbing::try_execute_query::<rustc_query_impl[f22ae8d29d82c5ee]::queries::resolver_for_lowering, rustc_query_impl[f22ae8d29d82c5ee]::plumbing::QueryCtxt>
  32:     0x7f4a79494963 - <rustc_query_impl[f22ae8d29d82c5ee]::Queries as rustc_middle[cd8924ec0e73eabb]::ty::query::QueryEngine>::resolver_for_lowering
  33:     0x7f4a779a69c0 - <rustc_middle[cd8924ec0e73eabb]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[6e6e6528accfd63]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[bf7766984c15ead8]::steal::Steal<(rustc_middle[cd8924ec0e73eabb]::ty::ResolverAstLowering, alloc[e09c93b9546bc735]::rc::Rc<rustc_ast[e7d7a5047d46c716]::ast::Crate>)>>
  34:     0x7f4a779f38b6 - <rustc_interface[6219fe1e1c8d4b8]::interface::Compiler>::enter::<rustc_driver_impl[6e6e6528accfd63]::run_compiler::{closure#1}::{closure#2}, core[feb76cca35898a1f]::result::Result<core[feb76cca35898a1f]::option::Option<rustc_interface[6219fe1e1c8d4b8]::queries::Linker>, rustc_span[1d330a307b171ff]::ErrorGuaranteed>>
  35:     0x7f4a779ad3d8 - rustc_span[1d330a307b171ff]::with_source_map::<core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>, rustc_interface[6219fe1e1c8d4b8]::interface::run_compiler<core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>, rustc_driver_impl[6e6e6528accfd63]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  36:     0x7f4a779e4307 - std[a5c91af2d15e892]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6219fe1e1c8d4b8]::util::run_in_thread_pool_with_globals<rustc_interface[6219fe1e1c8d4b8]::interface::run_compiler<core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>, rustc_driver_impl[6e6e6528accfd63]::run_compiler::{closure#1}>::{closure#0}, core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>>
  37:     0x7f4a779f4406 - std[a5c91af2d15e892]::panicking::try::<core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>, core[feb76cca35898a1f]::panic::unwind_safe::AssertUnwindSafe<<std[a5c91af2d15e892]::thread::Builder>::spawn_unchecked_<rustc_interface[6219fe1e1c8d4b8]::util::run_in_thread_pool_with_globals<rustc_interface[6219fe1e1c8d4b8]::interface::run_compiler<core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>, rustc_driver_impl[6e6e6528accfd63]::run_compiler::{closure#1}>::{closure#0}, core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  38:     0x7f4a779b1945 - <<std[a5c91af2d15e892]::thread::Builder>::spawn_unchecked_<rustc_interface[6219fe1e1c8d4b8]::util::run_in_thread_pool_with_globals<rustc_interface[6219fe1e1c8d4b8]::interface::run_compiler<core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>, rustc_driver_impl[6e6e6528accfd63]::run_compiler::{closure#1}>::{closure#0}, core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[feb76cca35898a1f]::result::Result<(), rustc_span[1d330a307b171ff]::ErrorGuaranteed>>::{closure#1} as core[feb76cca35898a1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f4a76eac4de - std::sys::unix::thread::Thread::new::thread_start::h7a8fb31791c13ba9
  40:     0x7f4a76c46b43 - <unknown>
  41:     0x7f4a76cd8a00 - <unknown>
  42:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (474b3619f 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
end of query stack
note: run with `RUSTC_ICE_LOG=trace` environment variable to display a SpanTrace


---- [ui] tests/ui/chalkify/bugs/async.rs stdout ----


error: Error: expected failure status (Some(101)) but received status Some(1).
status: exit status: 1
command: RUSTC_ICE_LOG="trace" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/chalkify/bugs/async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/auxiliary" "--edition=2021" "-Z" "trait-solver=chalk"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when finding item bounds for `foo::{opaque#0}`
  --> fake-test-src-base/chalkify/bugs/async.rs:24:25
   |
LL | async fn foo(x: u32) -> u32 {
   |
   |
   = note: ...which immediately requires finding item bounds for `foo::{opaque#0}` again
note: cycle used when collecting item types in top-level module
  --> fake-test-src-base/chalkify/bugs/async.rs:22:1
LL | / fn main() -> () {}
LL | |
LL | |
LL | | async fn foo(x: u32) -> u32 {
LL | |     x
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
