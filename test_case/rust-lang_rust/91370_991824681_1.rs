
   Compiling playground v0.0.1 (/playground)
error: incorrect function inside `extern` block
 --> src/main.rs:2:8
  |
1 |   extern {
  |   ------ `extern` blocks define existing foreign functions and functions inside of them cannot have a body
2 |       fn r() {
  |  ________^___-
  | |        |
  | |        cannot have a body
3 | |         impl Copy for u8 {}
4 | |     }
  | |_____- help: remove the invalid body: `;`
  |
  = help: you might have meant to write a function accessible through FFI, which can be done by writing `extern fn` outside of the `extern` block
  = note: for more information, visit https://doc.rust-lang.org/std/keyword.extern.html

error[E0601]: `main` function not found in crate `playground`
 --> src/main.rs:1:1
  |
1 | / extern {
2 | |     fn r() {
3 | |         impl Copy for u8 {}
4 | |     }
5 | | }
  | |_^ consider adding a `main` function to `src/main.rs`

error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:875:18: expected item, found unknown node (hir_id=HirId { owner: DefId(0:5 ~ playground[965a]::::r::{impl#0}), local_id: 0 })

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1170:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::bug
   3: <rustc_errors::Handler>::bug
   4: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::bug_fmt
   7: <rustc_middle::hir::map::Map>::expect_item
   8: rustc_typeck::collect::impl_trait_ref
   9: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::impl_trait_ref
  10: rustc_typeck::coherence::orphan::orphan_check_crate
  11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), &[rustc_span::def_id::LocalDefId]>>
  12: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::orphan_check_crate, rustc_query_impl::plumbing::QueryCtxt>
  13: <rustc_session::session::Session>::track_errors::<rustc_typeck::check_crate::{closure#3}, ()>
  14: rustc_typeck::check_crate
  15: rustc_interface::passes::analysis
  16: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  17: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  18: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
  19: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  20: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  21: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (0b42deacc 2021-12-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread 'rustc' panicked at 'index out of bounds: the len is 5 but the index is 5', compiler/rustc_middle/src/hir/map/mod.rs:210:52
stack backtrace:
   0:     0x7fb7e8cd435c - std::backtrace_rs::backtrace::libunwind::trace::hf7449935ead7573e
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fb7e8cd435c - std::backtrace_rs::backtrace::trace_unsynchronized::h221aa2d88d72372a
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fb7e8cd435c - std::sys_common::backtrace::_print_fmt::h1c77e8983e1df895
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7fb7e8cd435c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd4ec41a9a6b0d22c
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7fb7e8d3137c - core::fmt::write::h72801a82c94e6ff1
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/core/src/fmt/mod.rs:1149:17
   5:     0x7fb7e8cc4a65 - std::io::Write::write_fmt::haf74340a8cbeaa88
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/io/mod.rs:1660:15
   6:     0x7fb7e8cd7520 - std::sys_common::backtrace::_print::h2d15cd157796a64a
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7fb7e8cd7520 - std::sys_common::backtrace::print::h52d286d22e2398eb
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7fb7e8cd7520 - std::panicking::default_hook::{{closure}}::h6da08fba6306daf2
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:211:50
   9:     0x7fb7e8cd70cb - std::panicking::default_hook::h266f67a22e76b11a
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:228:9
  10:     0x7fb7e97fb291 - rustc_driver[feb513e6adc957d8]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fb7e8cd7d39 - std::panicking::rust_panic_with_hook::he55698a957f4fb6d
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:610:17
  12:     0x7fb7e8cd77f0 - std::panicking::begin_panic_handler::{{closure}}::h01f453c3ac181895
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:502:13
  13:     0x7fb7e8cd4804 - std::sys_common::backtrace::__rust_end_short_backtrace::h675d77c6e5a3926d
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys_common/backtrace.rs:139:18
  14:     0x7fb7e8cd7759 - rust_begin_unwind
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:498:5
  15:     0x7fb7e8c9bc21 - core::panicking::panic_fmt::h7b8580d81fcbbacd
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/core/src/panicking.rs:107:14
  16:     0x7fb7e8c9bbe2 - core::panicking::panic_bounds_check::h63650a5dfc9aa86f
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/core/src/panicking.rs:75:5
  17:     0x7fb7eb5bc802 - <rustc_middle[f720485c3bb04922]::hir::provide::{closure#7} as core[cc79c391059f8e46]::ops::function::FnOnce<(rustc_middle[f720485c3bb04922]::ty::context::TyCtxt, rustc_span[dda57b1885b40b9a]::def_id::DefId)>>::call_once
  18:     0x7fb7eb16065b - <rustc_query_impl[9c2ccd6b3a0df356]::Queries as rustc_middle[f720485c3bb04922]::ty::query::QueryEngine>::def_span
  19:     0x7fb7ebf367be - <rustc_middle[f720485c3bb04922]::ty::print::pretty::FmtPrinter<&mut alloc[1954047e53701c4d]::string::String> as rustc_middle[f720485c3bb04922]::ty::print::Printer>::print_def_path
  20:     0x7fb7ebf27375 - <rustc_middle[f720485c3bb04922]::ty::context::TyCtxt>::def_path_str
  21:     0x7fb7ea13d531 - <std[f24903a91e569aa2]::thread::local::LocalKey<core[cc79c391059f8e46]::cell::Cell<bool>>>::with::<rustc_middle[f720485c3bb04922]::ty::print::pretty::with_no_trimmed_paths<<rustc_query_impl[9c2ccd6b3a0df356]::queries::impl_trait_ref as rustc_query_system[27ba240b72fd7978]::query::config::QueryDescription<rustc_query_impl[9c2ccd6b3a0df356]::plumbing::QueryCtxt>>::describe::{closure#0}, alloc[1954047e53701c4d]::string::String>::{closure#0}, alloc[1954047e53701c4d]::string::String>
  22:     0x7fb7ea16f728 - <rustc_query_impl[9c2ccd6b3a0df356]::queries::impl_trait_ref as rustc_query_system[27ba240b72fd7978]::query::config::QueryDescription<rustc_query_impl[9c2ccd6b3a0df356]::plumbing::QueryCtxt>>::describe
  23:     0x7fb7ea1568c7 - <std[f24903a91e569aa2]::thread::local::LocalKey<core[cc79c391059f8e46]::cell::Cell<bool>>>::with::<rustc_middle[f720485c3bb04922]::ty::print::pretty::with_forced_impl_filename_line<rustc_query_impl[9c2ccd6b3a0df356]::make_query::impl_trait_ref::{closure#0}::{closure#0}, alloc[1954047e53701c4d]::string::String>::{closure#0}, alloc[1954047e53701c4d]::string::String>
  24:     0x7fb7ea14b719 - <std[f24903a91e569aa2]::thread::local::LocalKey<core[cc79c391059f8e46]::cell::Cell<bool>>>::with::<rustc_middle[f720485c3bb04922]::ty::print::pretty::with_no_visible_paths<rustc_query_impl[9c2ccd6b3a0df356]::make_query::impl_trait_ref::{closure#0}, alloc[1954047e53701c4d]::string::String>::{closure#0}, alloc[1954047e53701c4d]::string::String>
  25:     0x7fb7ea2ec6d5 - rustc_query_impl[9c2ccd6b3a0df356]::make_query::impl_trait_ref
  26:     0x7fb7ea0979cb - <rustc_query_system[27ba240b72fd7978]::query::plumbing::QueryState<rustc_middle[f720485c3bb04922]::dep_graph::dep_node::DepKind, rustc_span[dda57b1885b40b9a]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[9c2ccd6b3a0df356]::plumbing::QueryCtxt>
  27:     0x7fb7ea18db7f - <rustc_query_impl[9c2ccd6b3a0df356]::Queries>::try_collect_active_jobs
  28:     0x7fb7ea16d532 - <rustc_query_impl[9c2ccd6b3a0df356]::plumbing::QueryCtxt>::try_print_query_stack
  29:     0x7fb7e991978f - rustc_interface[4c4b5644b43577f1]::interface::try_print_query_stack
  30:     0x7fb7e97fb6b4 - rustc_driver[feb513e6adc957d8]::report_ice
  31:     0x7fb7e8cd7d39 - std::panicking::rust_panic_with_hook::he55698a957f4fb6d
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:610:17
  32:     0x7fb7ea929e6b - std[f24903a91e569aa2]::panicking::begin_panic::<rustc_errors[248f3f311c690252]::ExplicitBug>::{closure#0}
  33:     0x7fb7ea929e06 - std[f24903a91e569aa2]::sys_common::backtrace::__rust_end_short_backtrace::<std[f24903a91e569aa2]::panicking::begin_panic<rustc_errors[248f3f311c690252]::ExplicitBug>::{closure#0}, !>
  34:     0x7fb7ea92cf5f - std[f24903a91e569aa2]::panicking::begin_panic::<rustc_errors[248f3f311c690252]::ExplicitBug>
  35:     0x7fb7ea937ead - std[f24903a91e569aa2]::panic::panic_any::<rustc_errors[248f3f311c690252]::ExplicitBug>
  36:     0x7fb7ea93956d - <rustc_errors[248f3f311c690252]::HandlerInner>::bug
  37:     0x7fb7ea939010 - <rustc_errors[248f3f311c690252]::Handler>::bug
  38:     0x7fb7ea7bd5b6 - rustc_middle[f720485c3bb04922]::ty::context::tls::with_opt::<rustc_middle[f720485c3bb04922]::util::bug::opt_span_bug_fmt<rustc_span[dda57b1885b40b9a]::span_encoding::Span>::{closure#0}, ()>
  39:     0x7fb7ea7bdae0 - rustc_middle[f720485c3bb04922]::util::bug::opt_span_bug_fmt::<rustc_span[dda57b1885b40b9a]::span_encoding::Span>
  40:     0x7fb7ea7bda56 - rustc_middle[f720485c3bb04922]::util::bug::bug_fmt
  41:     0x7fb7eb61f7a7 - <rustc_middle[f720485c3bb04922]::hir::map::Map>::expect_item
  42:     0x7fb7eb92047a - rustc_typeck[3188bff36eb59f03]::collect::impl_trait_ref
  43:     0x7fb7eb15d252 - <rustc_query_impl[9c2ccd6b3a0df356]::Queries as rustc_middle[f720485c3bb04922]::ty::query::QueryEngine>::impl_trait_ref
  44:     0x7fb7eb955da5 - rustc_typeck[3188bff36eb59f03]::coherence::orphan::orphan_check_crate
  45:     0x7fb7ebb61319 - rustc_query_system[27ba240b72fd7978]::query::plumbing::try_execute_query::<rustc_query_impl[9c2ccd6b3a0df356]::plumbing::QueryCtxt, rustc_query_system[27ba240b72fd7978]::query::caches::DefaultCache<(), &[rustc_span[dda57b1885b40b9a]::def_id::LocalDefId]>>
  46:     0x7fb7ebba1ff1 - rustc_query_system[27ba240b72fd7978]::query::plumbing::get_query::<rustc_query_impl[9c2ccd6b3a0df356]::queries::orphan_check_crate, rustc_query_impl[9c2ccd6b3a0df356]::plumbing::QueryCtxt>
  47:     0x7fb7eb922304 - <rustc_session[2680299e10ec9d86]::session::Session>::track_errors::<rustc_typeck[3188bff36eb59f03]::check_crate::{closure#3}, ()>
  48:     0x7fb7eb909a7a - rustc_typeck[3188bff36eb59f03]::check_crate
  49:     0x7fb7eb6b8cf7 - rustc_interface[4c4b5644b43577f1]::passes::analysis
  50:     0x7fb7ebb5c135 - rustc_query_system[27ba240b72fd7978]::query::plumbing::try_execute_query::<rustc_query_impl[9c2ccd6b3a0df356]::plumbing::QueryCtxt, rustc_query_system[27ba240b72fd7978]::query::caches::DefaultCache<(), core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>>
  51:     0x7fb7ebbb2de5 - rustc_query_system[27ba240b72fd7978]::query::plumbing::get_query::<rustc_query_impl[9c2ccd6b3a0df356]::queries::analysis, rustc_query_impl[9c2ccd6b3a0df356]::plumbing::QueryCtxt>
  52:     0x7fb7eb6ae5b9 - <rustc_interface[4c4b5644b43577f1]::passes::QueryContext>::enter::<rustc_driver[feb513e6adc957d8]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>
  53:     0x7fb7eb68f7ea - <rustc_interface[4c4b5644b43577f1]::interface::Compiler>::enter::<rustc_driver[feb513e6adc957d8]::run_compiler::{closure#1}::{closure#2}, core[cc79c391059f8e46]::result::Result<core[cc79c391059f8e46]::option::Option<rustc_interface[4c4b5644b43577f1]::queries::Linker>, rustc_errors[248f3f311c690252]::ErrorReported>>
  54:     0x7fb7eb68c14d - rustc_span[dda57b1885b40b9a]::with_source_map::<core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>, rustc_interface[4c4b5644b43577f1]::interface::create_compiler_and_run<core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>, rustc_driver[feb513e6adc957d8]::run_compiler::{closure#1}>::{closure#1}>
  55:     0x7fb7eb6907ef - <scoped_tls[3fea4c3dcac147b1]::ScopedKey<rustc_span[dda57b1885b40b9a]::SessionGlobals>>::set::<rustc_interface[4c4b5644b43577f1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[4c4b5644b43577f1]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>, rustc_driver[feb513e6adc957d8]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>::{closure#0}::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>
  56:     0x7fb7eb68e995 - std[f24903a91e569aa2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4c4b5644b43577f1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[4c4b5644b43577f1]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>, rustc_driver[feb513e6adc957d8]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>
  57:     0x7fb7eb6aeffa - <<std[f24903a91e569aa2]::thread::Builder>::spawn_unchecked<rustc_interface[4c4b5644b43577f1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[4c4b5644b43577f1]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>, rustc_driver[feb513e6adc957d8]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>::{closure#1} as core[cc79c391059f8e46]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7fb7e8ce2e23 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4beb69a85f7fb16c
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/alloc/src/boxed.rs:1811:9
  59:     0x7fb7e8ce2e23 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hef865a799f44aaf2
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/alloc/src/boxed.rs:1811:9
  60:     0x7fb7e8ce2e23 - std::sys::unix::thread::Thread::new::thread_start::hee0b2d4e2414fa96
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys/unix/thread.rs:108:17
  61:     0x7fb7e8c1b609 - start_thread
  62:     0x7fb7e8b2f293 - clone
  63:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (0b42deacc 2021-12-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
error: could not compile `playground` due to 2 previous errors
