rust
error: internal compiler error: /rustc/8ea09f172814926c4c234f649b199f6aa9205307/compiler/rustc_infer/src/infer/outlives/env.rs:145:26: add_outlives_bounds: unexpected regions

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/8ea09f172814926c4c234f649b199f6aa9205307/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7fc55f58750a - std::backtrace_rs::backtrace::libunwind::trace::h6cb6c29587441f8e
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fc55f58750a - std::backtrace_rs::backtrace::trace_unsynchronized::h8c5477e383770049
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc55f58750a - std::sys_common::backtrace::_print_fmt::h83f599ed5943481c
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fc55f58750a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8324227ae12c4eff
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fc55f5eab3e - core::fmt::write::h5c56cab042d28558
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/core/src/fmt/mod.rs:1232:17
   5:     0x7fc55f57a375 - std::io::Write::write_fmt::hf4f87c91cf61b6f0
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/io/mod.rs:1684:15
   6:     0x7fc55f5872d5 - std::sys_common::backtrace::_print::h0bdb139796fd67c5
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fc55f5872d5 - std::sys_common::backtrace::print::h4097914cd2148b67
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fc55f58a04f - std::panicking::default_hook::{{closure}}::h1d3059499a0be6b5
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/panicking.rs:271:22
   9:     0x7fc55f589d8b - std::panicking::default_hook::h928cd28524aa6750
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/panicking.rs:290:9
  10:     0x7fc55e2ae365 - rustc_driver_impl[185cc224cea72581]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fc55f58a88d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h43db6c5edbab66d6
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/alloc/src/boxed.rs:2002:9
  12:     0x7fc55f58a88d - std::panicking::rust_panic_with_hook::h7c9082922d0f7bfc
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/panicking.rs:696:13
  13:     0x7fc55e8134e1 - std[262da2da9c85f821]::panicking::begin_panic::<rustc_errors[3aceb780ccb62908]::ExplicitBug>::{closure#0}
  14:     0x7fc55e80e946 - std[262da2da9c85f821]::sys_common::backtrace::__rust_end_short_backtrace::<std[262da2da9c85f821]::panicking::begin_panic<rustc_errors[3aceb780ccb62908]::ExplicitBug>::{closure#0}, !>
  15:     0x7fc55e80bbf6 - std[262da2da9c85f821]::panicking::begin_panic::<rustc_errors[3aceb780ccb62908]::ExplicitBug>
  16:     0x7fc55e860976 - std[262da2da9c85f821]::panic::panic_any::<rustc_errors[3aceb780ccb62908]::ExplicitBug>
  17:     0x7fc55e85d196 - <rustc_errors[3aceb780ccb62908]::HandlerInner>::bug::<&alloc[6dfa18b13b64e7c4]::string::String>
  18:     0x7fc55e85ce60 - <rustc_errors[3aceb780ccb62908]::Handler>::bug::<&alloc[6dfa18b13b64e7c4]::string::String>
  19:     0x7fc55e84931b - rustc_middle[858f6d7dd951ad9a]::util::bug::opt_span_bug_fmt::<rustc_span[caf13abfe4deac85]::span_encoding::Span>::{closure#0}
  20:     0x7fc55e847c6a - rustc_middle[858f6d7dd951ad9a]::ty::context::tls::with_opt::<rustc_middle[858f6d7dd951ad9a]::util::bug::opt_span_bug_fmt<rustc_span[caf13abfe4deac85]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0x7fc55e847c36 - rustc_middle[858f6d7dd951ad9a]::ty::context::tls::with_context_opt::<rustc_middle[858f6d7dd951ad9a]::ty::context::tls::with_opt<rustc_middle[858f6d7dd951ad9a]::util::bug::opt_span_bug_fmt<rustc_span[caf13abfe4deac85]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0x7fc55e849266 - rustc_middle[858f6d7dd951ad9a]::util::bug::opt_span_bug_fmt::<rustc_span[caf13abfe4deac85]::span_encoding::Span>
  23:     0x7fc55c8f2133 - rustc_middle[858f6d7dd951ad9a]::util::bug::bug_fmt
  24:     0x7fc55cac0d45 - <rustc_infer[c3ef9866355676a6]::infer::outlives::env::OutlivesEnvironment>::with_bounds::<core[a11a512dc6886a58]::iter::adapters::flatten::Flatten<core[a11a512dc6886a58]::iter::adapters::map::Map<indexmap[e821fc8f43264d5d]::set::IntoIter<rustc_middle[858f6d7dd951ad9a]::ty::Ty>, <rustc_infer[c3ef9866355676a6]::infer::InferCtxt as rustc_trait_selection[26b52ce1de15822d]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0}>>>
  25:     0x7fc55ce37ce0 - rustc_hir_analysis[d442839d624bc529]::check::compare_impl_item::compare_method_predicate_entailment
  26:     0x7fc55ce31701 - rustc_hir_analysis[d442839d624bc529]::check::compare_impl_item::compare_impl_method
  27:     0x7fc55ce29f3c - rustc_hir_analysis[d442839d624bc529]::check::check::check_impl_items_against_trait
  28:     0x7fc55ce212d8 - rustc_hir_analysis[d442839d624bc529]::check::check::check_mod_item_types
  29:     0x7fc55d8e9a9e - rustc_query_system[cc70169c16cb3288]::query::plumbing::try_execute_query::<rustc_query_impl[7095ed3878df9db6]::queries::check_mod_item_types, rustc_query_impl[7095ed3878df9db6]::plumbing::QueryCtxt>
  30:     0x7fc55d8e9623 - <rustc_query_impl[7095ed3878df9db6]::Queries as rustc_middle[858f6d7dd951ad9a]::ty::query::QueryEngine>::check_mod_item_types
  31:     0x7fc55d95b2dc - <rustc_middle[858f6d7dd951ad9a]::hir::map::Map>::for_each_module::<rustc_hir_analysis[d442839d624bc529]::check_crate::{closure#6}::{closure#0}>
  32:     0x7fc55c5cea78 - rustc_hir_analysis[d442839d624bc529]::check_crate
  33:     0x7fc55c5c6695 - rustc_interface[f786dc321c281ebd]::passes::analysis
  34:     0x7fc55dac25bc - rustc_query_system[cc70169c16cb3288]::query::plumbing::try_execute_query::<rustc_query_impl[7095ed3878df9db6]::queries::analysis, rustc_query_impl[7095ed3878df9db6]::plumbing::QueryCtxt>
  35:     0x7fc55dac22b0 - <rustc_query_impl[7095ed3878df9db6]::Queries as rustc_middle[858f6d7dd951ad9a]::ty::query::QueryEngine>::analysis
  36:     0x7fc55d8ef089 - <rustc_middle[858f6d7dd951ad9a]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[185cc224cea72581]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>
  37:     0x7fc55d485688 - rustc_span[caf13abfe4deac85]::with_source_map::<core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>, rustc_interface[f786dc321c281ebd]::interface::run_compiler<core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>, rustc_driver_impl[185cc224cea72581]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7fc55d47d1a0 - <scoped_tls[5aa1769cc2076f03]::ScopedKey<rustc_span[caf13abfe4deac85]::SessionGlobals>>::set::<rustc_interface[f786dc321c281ebd]::interface::run_compiler<core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>, rustc_driver_impl[185cc224cea72581]::run_compiler::{closure#1}>::{closure#0}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>
  39:     0x7fc55d47c882 - std[262da2da9c85f821]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f786dc321c281ebd]::util::run_in_thread_pool_with_globals<rustc_interface[f786dc321c281ebd]::interface::run_compiler<core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>, rustc_driver_impl[185cc224cea72581]::run_compiler::{closure#1}>::{closure#0}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>
  40:     0x7fc55d47c62a - <<std[262da2da9c85f821]::thread::Builder>::spawn_unchecked_<rustc_interface[f786dc321c281ebd]::util::run_in_thread_pool_with_globals<rustc_interface[f786dc321c281ebd]::interface::run_compiler<core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>, rustc_driver_impl[185cc224cea72581]::run_compiler::{closure#1}>::{closure#0}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>::{closure#1} as core[a11a512dc6886a58]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7fc55f594783 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4e1f4cc398ef09af
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/alloc/src/boxed.rs:1988:9
  42:     0x7fc55f594783 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1cd028674f7cf6f1
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/alloc/src/boxed.rs:1988:9
  43:     0x7fc55f594783 - std::sys::unix::thread::Thread::new::thread_start::h4af0c18be6dcac5b
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/sys/unix/thread.rs:108:17
  44:     0x7fc55aa23ea5 - start_thread
  45:     0x7fc55a74cb0d - clone
  46:                0x0 - <unknown>

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (8ea09f172 2023-03-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z binary-dep-depinfo -Z tls-model=initial-exec

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_mod_item_types] checking item types in module `spki`
#1 [analysis] running analysis passes on this crate

error: could not compile `spki`
