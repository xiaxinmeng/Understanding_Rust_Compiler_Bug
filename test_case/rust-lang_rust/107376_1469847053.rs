
error: internal compiler error: /rustc/e84e5ff04a647ce28540300244a26ba120642eea/compiler/rustc_infer/src/infer/outlives/env.rs:145:26: add_outlives_bounds: unexpected regions

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/e84e5ff04a647ce28540300244a26ba120642eea/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7f5269f4151a - std::backtrace_rs::backtrace::libunwind::trace::hab17a96bd248950c
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f5269f4151a - std::backtrace_rs::backtrace::trace_unsynchronized::h2dd96cad48ba21dc
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f5269f4151a - std::sys_common::backtrace::_print_fmt::h38b10a2f9d2b2585
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f5269f4151a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h194193fb0b01617d
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f5269fa4c5e - core::fmt::write::hd9150a25fdbebf32
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/core/src/fmt/mod.rs:1232:17
   5:     0x7f5269f34275 - std::io::Write::write_fmt::h3837df0e41e13e0e
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/io/mod.rs:1684:15
   6:     0x7f5269f412e5 - std::sys_common::backtrace::_print::h8a5cae525f1be897
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f5269f412e5 - std::sys_common::backtrace::print::h49b97caefe2bba8a
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f5269f4405f - std::panicking::default_hook::{{closure}}::h2a3da047f4b2da03
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/panicking.rs:271:22
   9:     0x7f5269f43d9b - std::panicking::default_hook::hc971a327b2a9d145
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/panicking.rs:290:9
  10:     0x7f526d244045 - rustc_driver_impl[6fa38601416e453]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f5269f4489d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h8520e136aebea51a
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/alloc/src/boxed.rs:2002:9
  12:     0x7f5269f4489d - std::panicking::rust_panic_with_hook::h93b4e29d3ec3eadc
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/panicking.rs:696:13
    Checking webrender_api v0.61.0 (/tmp/.tmp2jNQeL/webrender_api)
  13:     0x7f526d7a27a1 - std[45927792e1219606]::panicking::begin_panic::<rustc_errors[d7b71bcef7ff042a]::ExplicitBug>::{closure#0}
  14:     0x7f526d79dc46 - std[45927792e1219606]::sys_common::backtrace::__rust_end_short_backtrace::<std[45927792e1219606]::panicking::begin_panic<rustc_errors[d7b71bcef7ff042a]::ExplicitBug>::{closure#0}, !>
  15:     0x7f526d744536 - std[45927792e1219606]::panicking::begin_panic::<rustc_errors[d7b71bcef7ff042a]::ExplicitBug>
  16:     0x7f526d7eeea6 - std[45927792e1219606]::panic::panic_any::<rustc_errors[d7b71bcef7ff042a]::ExplicitBug>
  17:     0x7f526d7eb726 - <rustc_errors[d7b71bcef7ff042a]::HandlerInner>::bug::<&alloc[f3ac33a76f945683]::string::String>
  18:     0x7f526d7eb3f0 - <rustc_errors[d7b71bcef7ff042a]::Handler>::bug::<&alloc[f3ac33a76f945683]::string::String>
  19:     0x7f526d7d77fb - rustc_middle[7f2c9dc76d3467a6]::util::bug::opt_span_bug_fmt::<rustc_span[a2f01ea7f179d208]::span_encoding::Span>::{closure#0}
  20:     0x7f526d7d625a - rustc_middle[7f2c9dc76d3467a6]::ty::context::tls::with_opt::<rustc_middle[7f2c9dc76d3467a6]::util::bug::opt_span_bug_fmt<rustc_span[a2f01ea7f179d208]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0x7f526d7d6226 - rustc_middle[7f2c9dc76d3467a6]::ty::context::tls::with_context_opt::<rustc_middle[7f2c9dc76d3467a6]::ty::context::tls::with_opt<rustc_middle[7f2c9dc76d3467a6]::util::bug::opt_span_bug_fmt<rustc_span[a2f01ea7f179d208]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0x7f526d7d7746 - rustc_middle[7f2c9dc76d3467a6]::util::bug::opt_span_bug_fmt::<rustc_span[a2f01ea7f179d208]::span_encoding::Span>
  23:     0x7f526b9389f3 - rustc_middle[7f2c9dc76d3467a6]::util::bug::bug_fmt
  24:     0x7f526c14d855 - <rustc_infer[219913fda715446f]::infer::outlives::env::OutlivesEnvironment>::with_bounds::<core[33bcc9bae161e6d]::iter::adapters::flatten::Flatten<core[33bcc9bae161e6d]::iter::adapters::map::Map<indexmap[52bde976a34d3203]::set::IntoIter<rustc_middle[7f2c9dc76d3467a6]::ty::Ty>, <rustc_infer[219913fda715446f]::infer::InferCtxt as rustc_trait_selection[a330fa3a60fd254]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0}>>>
  25:     0x7f526b3050e0 - rustc_hir_analysis[afd80bd0636fe849]::check::compare_impl_item::compare_method_predicate_entailment
  26:     0x7f526b2ff061 - rustc_hir_analysis[afd80bd0636fe849]::check::compare_impl_item::compare_impl_method
  27:     0x7f526b2f1a67 - rustc_hir_analysis[afd80bd0636fe849]::check::check::check_mod_item_types
  28:     0x7f526c88023e - rustc_query_system[c5524b2d21c39947]::query::plumbing::try_execute_query::<rustc_query_impl[79380581a3052c55]::queries::check_mod_item_types, rustc_query_impl[79380581a3052c55]::plumbing::QueryCtxt>
  29:     0x7f526c87fdc3 - <rustc_query_impl[79380581a3052c55]::Queries as rustc_middle[7f2c9dc76d3467a6]::ty::query::QueryEngine>::check_mod_item_types
  30:     0x7f526b5dacb4 - <rustc_session[63828082f6106403]::session::Session>::time::<(), rustc_hir_analysis[afd80bd0636fe849]::check_crate::{closure#6}>
  31:     0x7f526b5d7bf8 - rustc_hir_analysis[afd80bd0636fe849]::check_crate
  32:     0x7f526b5cf942 - rustc_interface[920696c1fd270c1b]::passes::analysis
  33:     0x7f526ca5cbec - rustc_query_system[c5524b2d21c39947]::query::plumbing::try_execute_query::<rustc_query_impl[79380581a3052c55]::queries::analysis, rustc_query_impl[79380581a3052c55]::plumbing::QueryCtxt>
  34:     0x7f526ca5c8e0 - <rustc_query_impl[79380581a3052c55]::Queries as rustc_middle[7f2c9dc76d3467a6]::ty::query::QueryEngine>::analysis
  35:     0x7f526c883a19 - <rustc_middle[7f2c9dc76d3467a6]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[6fa38601416e453]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[33bcc9bae161e6d]::result::Result<(), rustc_span[a2f01ea7f179d208]::ErrorGuaranteed>>
  36:     0x7f526c44baf8 - rustc_span[a2f01ea7f179d208]::with_source_map::<core[33bcc9bae161e6d]::result::Result<(), rustc_span[a2f01ea7f179d208]::ErrorGuaranteed>, rustc_interface[920696c1fd270c1b]::interface::run_compiler<core[33bcc9bae161e6d]::result::Result<(), rustc_span[a2f01ea7f179d208]::ErrorGuaranteed>, rustc_driver_impl[6fa38601416e453]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  37:     0x7f526c442c5c - std[45927792e1219606]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[920696c1fd270c1b]::util::run_in_thread_pool_with_globals<rustc_interface[920696c1fd270c1b]::interface::run_compiler<core[33bcc9bae161e6d]::result::Result<(), rustc_span[a2f01ea7f179d208]::ErrorGuaranteed>, rustc_driver_impl[6fa38601416e453]::run_compiler::{closure#1}>::{closure#0}, core[33bcc9bae161e6d]::result::Result<(), rustc_span[a2f01ea7f179d208]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33bcc9bae161e6d]::result::Result<(), rustc_span[a2f01ea7f179d208]::ErrorGuaranteed>>
  38:     0x7f526c44268a - <<std[45927792e1219606]::thread::Builder>::spawn_unchecked_<rustc_interface[920696c1fd270c1b]::util::run_in_thread_pool_with_globals<rustc_interface[920696c1fd270c1b]::interface::run_compiler<core[33bcc9bae161e6d]::result::Result<(), rustc_span[a2f01ea7f179d208]::ErrorGuaranteed>, rustc_driver_impl[6fa38601416e453]::run_compiler::{closure#1}>::{closure#0}, core[33bcc9bae161e6d]::result::Result<(), rustc_span[a2f01ea7f179d208]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[33bcc9bae161e6d]::result::Result<(), rustc_span[a2f01ea7f179d208]::ErrorGuaranteed>>::{closure#1} as core[33bcc9bae161e6d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f5269f4e793 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h81703dda3203ac5a
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/alloc/src/boxed.rs:1988:9
  40:     0x7f5269f4e793 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h30492a21713d5868
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/alloc/src/boxed.rs:1988:9
  41:     0x7f5269f4e793 - std::sys::unix::thread::Thread::new::thread_start::h834788d019271333
                               at /rustc/e84e5ff04a647ce28540300244a26ba120642eea/library/std/src/sys/unix/thread.rs:108:17
"/home/kobzol/.rustup/toolchains/e84e5ff04a647ce28540300244a26ba120642eea/bin/rustc" ["--crate-name", "webrender_api", "--edition=2018", "webrender_api/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata", "-C", "panic=abort", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=a06d87f097185725", "-C", "extra-filename=-a06d87f097185725", "--out-dir", "/tmp/.tmp2jNQeL/target/debug/deps", "-L", "dependency=/tmp/.tmp2jNQeL/target/debug/deps", "--extern", "app_units=/tmp/.tmp2jNQeL/target/debug/deps/libapp_units-e681452e9dd8cc1f.rmeta", "--extern", "bitflags=/tmp/.tmp2jNQeL/target/debug/deps/libbitflags-0c06b3de9eeaffca.rmeta", "--extern", "byteorder=/tmp/.tmp2jNQeL/target/debug/deps/libbyteorder-b12ea949ed394b3d.rmeta", "--extern", "crossbeam_channel=/tmp/.tmp2jNQeL/target/debug/deps/libcrossbeam_channel-5cb9c8316425f592.rmeta", "--extern", "derive_more=/tmp/.tmp2jNQeL/target/debug/deps/libderive_more-46e888cbbc3b0572.so", "--extern", "euclid=/tmp/.tmp2jNQeL/target/debug/deps/libeuclid-3dade91424e2ce9f.rmeta", "--extern", "malloc_size_of_derive=/tmp/.tmp2jNQeL/target/debug/deps/libmalloc_size_of_derive-21d38c919dfc4e5a.so", "--extern", "peek_poke=/tmp/.tmp2jNQeL/target/debug/deps/libpeek_poke-e342700e96f26c4f.rmeta", "--extern", "serde=/tmp/.tmp2jNQeL/target/debug/deps/libserde-f35afb1b090c7b1f.rmeta", "--extern", "serde_bytes=/tmp/.tmp2jNQeL/target/debug/deps/libserde_bytes-5ab9bd8ae188611f.rmeta", "--extern", "serde_derive=/tmp/.tmp2jNQeL/target/debug/deps/libserde_derive-f27ea7983d9d2a1c.so", "--extern", "time=/tmp/.tmp2jNQeL/target/debug/deps/libtime-2cd048cec9905c10.rmeta", "--extern", "malloc_size_of=/tmp/.tmp2jNQeL/target/debug/deps/libwr_malloc_size_of-c8cdd12f12e44948.rmeta", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
  42:     0x7f5269c75b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  43:     0x7f5269d07a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  44:                0x0 - <unknown>

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (e84e5ff04 2023-03-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C panic=abort -C embed-bitcode=no -C debuginfo=2 -Z incremental-verify-ich

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_mod_item_types] checking item types in module `de`
#1 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `bincode` (lib)
