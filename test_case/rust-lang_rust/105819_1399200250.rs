
error: internal compiler error: compiler/rustc_middle/src/ty/closure.rs:172:25: Unexpected type Alias(Opaque, AliasTy { substs: [], def_id: DefId(0:13 ~ c[e654]::upvar::T::{opaque#0}) }) for `Field` projection
  --> c.rs:13:27
   |
13 |         let Foo((a, b)) = foo;
   |                           ^^^

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/compiler/rustc_errors/src/lib.rs:987:33
stack backtrace:
   0:     0x7fce191653ca - std::backtrace_rs::backtrace::libunwind::trace::ha92ef9f6ccd5e56b
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fce191653ca - std::backtrace_rs::backtrace::trace_unsynchronized::ha44c299690166f2b
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fce191653ca - std::sys_common::backtrace::_print_fmt::h19cd3f39c344a33c
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fce191653ca - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h293cba156d45b475
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fce191c867e - core::fmt::write::h2a89c5a2cd2a0936
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/core/src/fmt/mod.rs:1213:17
   5:     0x7fce19155a15 - std::io::Write::write_fmt::h61b873d4305ed5fb
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/io/mod.rs:1682:15
   6:     0x7fce19165195 - std::sys_common::backtrace::_print::he80b3ad6361d876c
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fce19165195 - std::sys_common::backtrace::print::ha340fba9744b3a51
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fce19167f5f - std::panicking::default_hook::{{closure}}::hbb5472f169f46106
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/panicking.rs:267:22
   9:     0x7fce19167c9b - std::panicking::default_hook::h910b88c51274ec02
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/panicking.rs:286:9
  10:     0x7fce1c45ec54 - rustc_driver[639bbb82542a12c4]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fce1916879a - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hf5cbaf936ef8179d
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/alloc/src/boxed.rs:2002:9
  12:     0x7fce1916879a - std::panicking::rust_panic_with_hook::h637d33e8738b38ce
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/panicking.rs:692:13
  13:     0x7fce1c9c8521 - std[160d0d017ec6e0b8]::panicking::begin_panic::<rustc_errors[ab114d7d6097e91e]::ExplicitBug>::{closure#0}
  14:     0x7fce1c9c4fe6 - std[160d0d017ec6e0b8]::sys_common::backtrace::__rust_end_short_backtrace::<std[160d0d017ec6e0b8]::panicking::begin_panic<rustc_errors[ab114d7d6097e91e]::ExplicitBug>::{closure#0}, !>
  15:     0x7fce1ca27686 - std[160d0d017ec6e0b8]::panicking::begin_panic::<rustc_errors[ab114d7d6097e91e]::ExplicitBug>
  16:     0x7fce1c9e0366 - std[160d0d017ec6e0b8]::panic::panic_any::<rustc_errors[ab114d7d6097e91e]::ExplicitBug>
  17:     0x7fce1c9dfd22 - <rustc_errors[ab114d7d6097e91e]::HandlerInner>::span_bug::<rustc_span[6badc9707177032f]::span_encoding::Span, &alloc[3c9e883abe43a86f]::string::String>
  18:     0x7fce1c9ded77 - <rustc_errors[ab114d7d6097e91e]::Handler>::span_bug::<rustc_span[6badc9707177032f]::span_encoding::Span, &alloc[3c9e883abe43a86f]::string::String>
  19:     0x7fce1c9eed0b - rustc_middle[81963a276c0c8277]::util::bug::opt_span_bug_fmt::<rustc_span[6badc9707177032f]::span_encoding::Span>::{closure#0}
  20:     0x7fce1c9ec09a - rustc_middle[81963a276c0c8277]::ty::context::tls::with_opt::<rustc_middle[81963a276c0c8277]::util::bug::opt_span_bug_fmt<rustc_span[6badc9707177032f]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0x7fce1c9ec06a - rustc_middle[81963a276c0c8277]::ty::context::tls::with_context_opt::<rustc_middle[81963a276c0c8277]::ty::context::tls::with_opt<rustc_middle[81963a276c0c8277]::util::bug::opt_span_bug_fmt<rustc_span[6badc9707177032f]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0x7fce1c9eec46 - rustc_middle[81963a276c0c8277]::util::bug::opt_span_bug_fmt::<rustc_span[6badc9707177032f]::span_encoding::Span>
  23:     0x7fce1c9eec04 - rustc_middle[81963a276c0c8277]::util::bug::span_bug_fmt::<rustc_span[6badc9707177032f]::span_encoding::Span>
  24:     0x7fce1b97edd4 - <&mut rustc_middle[81963a276c0c8277]::ty::closure::symbols_for_closure_captures::{closure#0} as core[dd0ad632f08e6ea1]::ops::function::FnOnce<(&rustc_middle[81963a276c0c8277]::ty::closure::CapturedPlace,)>>::call_once
  25:     0x7fce1b97e3ef - <alloc[3c9e883abe43a86f]::vec::Vec<rustc_span[6badc9707177032f]::symbol::Symbol> as alloc[3c9e883abe43a86f]::vec::spec_from_iter::SpecFromIter<rustc_span[6badc9707177032f]::symbol::Symbol, core[dd0ad632f08e6ea1]::iter::adapters::map::Map<core[dd0ad632f08e6ea1]::iter::adapters::flatten::Flatten<core[dd0ad632f08e6ea1]::option::IntoIter<core[dd0ad632f08e6ea1]::iter::adapters::flatten::FlatMap<indexmap[469fb5cc230ef844]::map::Values<rustc_hir[af482e64e9f01e9c]::hir_id::HirId, alloc[3c9e883abe43a86f]::vec::Vec<rustc_middle[81963a276c0c8277]::ty::closure::CapturedPlace>>, core[dd0ad632f08e6ea1]::slice::iter::Iter<rustc_middle[81963a276c0c8277]::ty::closure::CapturedPlace>, <rustc_middle[81963a276c0c8277]::ty::typeck_results::TypeckResults>::closure_min_captures_flattened::{closure#0}::{closure#0}>>>, rustc_middle[81963a276c0c8277]::ty::closure::symbols_for_closure_captures::{closure#0}>>>::from_iter
  26:     0x7fce1b97e1ec - rustc_middle[81963a276c0c8277]::ty::closure::symbols_for_closure_captures
  27:     0x7fce1b9f87c7 - rustc_query_system[485dd4ae9ee084f0]::query::plumbing::try_execute_query::<rustc_query_impl[2185a4df21703c4]::queries::symbols_for_closure_captures, rustc_query_impl[2185a4df21703c4]::plumbing::QueryCtxt>
  28:     0x7fce1be4e6ff - <rustc_query_impl[2185a4df21703c4]::Queries as rustc_middle[81963a276c0c8277]::ty::query::QueryEngine>::symbols_for_closure_captures
  29:     0x7fce1b5acc4f - rustc_mir_build[ff9628d6c6d389c5]::build::mir_built
  30:     0x7fce1b24f415 - rustc_query_system[485dd4ae9ee084f0]::query::plumbing::try_execute_query::<rustc_query_impl[2185a4df21703c4]::queries::mir_built, rustc_query_impl[2185a4df21703c4]::plumbing::QueryCtxt>
  31:     0x7fce1b58208d - rustc_mir_transform[137284fa802fab14]::check_unsafety::unsafety_check_result
  32:     0x7fce1b39aa19 - rustc_query_system[485dd4ae9ee084f0]::query::plumbing::try_execute_query::<rustc_query_impl[2185a4df21703c4]::queries::unsafety_check_result, rustc_query_impl[2185a4df21703c4]::plumbing::QueryCtxt>
  33:     0x7fce1be4fe44 - <rustc_query_impl[2185a4df21703c4]::Queries as rustc_middle[81963a276c0c8277]::ty::query::QueryEngine>::unsafety_check_result
  34:     0x7fce1b584cce - rustc_mir_transform[137284fa802fab14]::check_unsafety::unsafety_check_result
  35:     0x7fce1b39aa19 - rustc_query_system[485dd4ae9ee084f0]::query::plumbing::try_execute_query::<rustc_query_impl[2185a4df21703c4]::queries::unsafety_check_result, rustc_query_impl[2185a4df21703c4]::plumbing::QueryCtxt>
  36:     0x7fce1b39735d - rustc_mir_transform[137284fa802fab14]::mir_const
  37:     0x7fce1b395d61 - rustc_query_system[485dd4ae9ee084f0]::query::plumbing::try_execute_query::<rustc_query_impl[2185a4df21703c4]::queries::mir_const, rustc_query_impl[2185a4df21703c4]::plumbing::QueryCtxt>
  38:     0x7fce1b38e1d9 - rustc_mir_transform[137284fa802fab14]::mir_promoted
  39:     0x7fce1b38c6a4 - rustc_query_system[485dd4ae9ee084f0]::query::plumbing::try_execute_query::<rustc_query_impl[2185a4df21703c4]::queries::mir_promoted, rustc_query_impl[2185a4df21703c4]::plumbing::QueryCtxt>
  40:     0x7fce1b38af48 - rustc_borrowck[e2111fe20b1902b8]::mir_borrowck
  41:     0x7fce1ad7fd9d - rustc_query_system[485dd4ae9ee084f0]::query::plumbing::try_execute_query::<rustc_query_impl[2185a4df21703c4]::queries::mir_borrowck, rustc_query_impl[2185a4df21703c4]::plumbing::QueryCtxt>
  42:     0x7fce1be512cd - <rustc_query_impl[2185a4df21703c4]::Queries as rustc_middle[81963a276c0c8277]::ty::query::QueryEngine>::mir_borrowck
  43:     0x7fce1c57f36a - <rustc_hir_analysis[dc2fdeeecccd2750]::collect::type_of::find_opaque_ty_constraints_for_tait::ConstraintLocator>::check
  44:     0x7fce1c57ec57 - rustc_hir_analysis[dc2fdeeecccd2750]::collect::type_of::find_opaque_ty_constraints_for_tait
  45:     0x7fce1bc86af9 - rustc_hir_analysis[dc2fdeeecccd2750]::collect::type_of::type_of
  46:     0x7fce1ad46fd8 - rustc_query_system[485dd4ae9ee084f0]::query::plumbing::get_query::<rustc_query_impl[2185a4df21703c4]::queries::type_of, rustc_query_impl[2185a4df21703c4]::plumbing::QueryCtxt, rustc_middle[81963a276c0c8277]::dep_graph::dep_node::DepKind>
  47:     0x7fce1b0101cb - rustc_hir_analysis[dc2fdeeecccd2750]::check::check::check_mod_item_types
  48:     0x7fce1ba0a151 - rustc_query_system[485dd4ae9ee084f0]::query::plumbing::try_execute_query::<rustc_query_impl[2185a4df21703c4]::queries::check_mod_item_types, rustc_query_impl[2185a4df21703c4]::plumbing::QueryCtxt>
  49:     0x7fce1be5082d - <rustc_query_impl[2185a4df21703c4]::Queries as rustc_middle[81963a276c0c8277]::ty::query::QueryEngine>::check_mod_item_types
  50:     0x7fce1aa8b097 - <rustc_session[59546dad5498a62e]::session::Session>::time::<(), rustc_hir_analysis[dc2fdeeecccd2750]::check_crate::{closure#6}>
  51:     0x7fce1aa8a8a2 - rustc_hir_analysis[dc2fdeeecccd2750]::check_crate
  52:     0x7fce1aa8a51b - rustc_interface[712fc5ea6eb72e1f]::passes::analysis
  53:     0x7fce1bbc267e - rustc_query_system[485dd4ae9ee084f0]::query::plumbing::try_execute_query::<rustc_query_impl[2185a4df21703c4]::queries::analysis, rustc_query_impl[2185a4df21703c4]::plumbing::QueryCtxt>
  54:     0x7fce1be4d6ba - <rustc_query_impl[2185a4df21703c4]::Queries as rustc_middle[81963a276c0c8277]::ty::query::QueryEngine>::analysis
  55:     0x7fce1b6e4420 - <rustc_interface[712fc5ea6eb72e1f]::passes::QueryContext>::enter::<rustc_driver[639bbb82542a12c4]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>>
  56:     0x7fce1b6e3221 - <rustc_interface[712fc5ea6eb72e1f]::interface::Compiler>::enter::<rustc_driver[639bbb82542a12c4]::run_compiler::{closure#1}::{closure#2}, core[dd0ad632f08e6ea1]::result::Result<core[dd0ad632f08e6ea1]::option::Option<rustc_interface[712fc5ea6eb72e1f]::queries::Linker>, rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>>
  57:     0x7fce1b6e12f4 - rustc_span[6badc9707177032f]::with_source_map::<core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>, rustc_interface[712fc5ea6eb72e1f]::interface::run_compiler<core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>, rustc_driver[639bbb82542a12c4]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  58:     0x7fce1b6e0e04 - <scoped_tls[3adb5d9e8b10118b]::ScopedKey<rustc_span[6badc9707177032f]::SessionGlobals>>::set::<rustc_interface[712fc5ea6eb72e1f]::interface::run_compiler<core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>, rustc_driver[639bbb82542a12c4]::run_compiler::{closure#1}>::{closure#0}, core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>>
  59:     0x7fce1b6e0502 - std[160d0d017ec6e0b8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[712fc5ea6eb72e1f]::util::run_in_thread_pool_with_globals<rustc_interface[712fc5ea6eb72e1f]::interface::run_compiler<core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>, rustc_driver[639bbb82542a12c4]::run_compiler::{closure#1}>::{closure#0}, core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>>
  60:     0x7fce1b6e02aa - <<std[160d0d017ec6e0b8]::thread::Builder>::spawn_unchecked_<rustc_interface[712fc5ea6eb72e1f]::util::run_in_thread_pool_with_globals<rustc_interface[712fc5ea6eb72e1f]::interface::run_compiler<core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>, rustc_driver[639bbb82542a12c4]::run_compiler::{closure#1}>::{closure#0}, core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[dd0ad632f08e6ea1]::result::Result<(), rustc_errors[ab114d7d6097e91e]::ErrorGuaranteed>>::{closure#1} as core[dd0ad632f08e6ea1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7fce19172bd3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4dd19c0bc3e6367b
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/alloc/src/boxed.rs:1988:9
  62:     0x7fce19172bd3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h379cba3f119da2fe
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/alloc/src/boxed.rs:1988:9
  63:     0x7fce19172bd3 - std::sys::unix::thread::Thread::new::thread_start::hd739ff786112afe1
                               at /rustc/5ce39f42bd2c8bca9c570f0560ebe1fce4eddb14/library/std/src/sys/unix/thread.rs:108:17
  64:     0x7fce18f048fd - <unknown>
  65:     0x7fce18f86a60 - <unknown>
  66:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (5ce39f42b 2023-01-20) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [symbols_for_closure_captures] finding symbols for captures of closure `upvar::{closure#0}` in `upvar::{closure#0}`
#1 [mir_built] building MIR for `upvar::{closure#0}`
#2 [unsafety_check_result] unsafety-checking `upvar::{closure#0}`
#3 [unsafety_check_result] unsafety-checking `upvar`
#4 [mir_const] preparing `upvar` for borrow checking
#5 [mir_promoted] processing MIR for `upvar`
#6 [mir_borrowck] borrow-checking `upvar`
#7 [type_of] computing type of `upvar::T::{opaque#0}`
#8 [check_mod_item_types] checking item types in top-level module
#9 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
