
   Compiling playground v0.0.1 (/playground)
thread 'rustc' panicked at 'assertion failed: !ty.needs_infer() && !ty.has_placeholders() && !ty.has_free_regions()', compiler/rustc_hir_typeck/src/writeback.rs:135:9
stack backtrace:
   0:     0x7f885b3b6f8a - std::backtrace_rs::backtrace::libunwind::trace::hc36a5d781c5b5ccc
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f885b3b6f8a - std::backtrace_rs::backtrace::trace_unsynchronized::hdf896053c14221b1
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f885b3b6f8a - std::sys_common::backtrace::_print_fmt::h39cabfaabdc86ac1
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f885b3b6f8a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h03d255761b970500
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f885b41b05f - core::fmt::write::h9be633e4e0b71092
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/core/src/fmt/mod.rs:1254:17
   5:     0x7f885b3a9bc5 - std::io::Write::write_fmt::hf094eaeff4c6712a
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/io/mod.rs:1698:15
   6:     0x7f885b3b6d55 - std::sys_common::backtrace::_print::hb3526973e1a9ef48
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f885b3b6d55 - std::sys_common::backtrace::print::h8b0ebf35d627197b
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f885b3b99fe - std::panicking::default_hook::{{closure}}::h800b864dad9a2ddb
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/panicking.rs:269:22
   9:     0x7f885b3b97a5 - std::panicking::default_hook::h47199288a8f9de13
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/panicking.rs:288:9
  10:     0x7f885e6bccf5 - <rustc_driver_impl[6aaaaf3365eb2b0a]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[c98013cfbe010fb2]::ops::function::FnOnce<(&core[c98013cfbe010fb2]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f885b3ba1f4 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::ha04c9fa67a129829
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/alloc/src/boxed.rs:1990:9
  12:     0x7f885b3ba1f4 - std::panicking::rust_panic_with_hook::hb6edee1e00f0405f
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/panicking.rs:694:13
  13:     0x7f885b3b9f22 - std::panicking::begin_panic_handler::{{closure}}::h8ad86fc31155b4e5
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/panicking.rs:579:13
  14:     0x7f885b3b73f6 - std::sys_common::backtrace::__rust_end_short_backtrace::h1382820b1dda7cef
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7f885b3b9cc2 - rust_begin_unwind
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/panicking.rs:577:5
  16:     0x7f885b417373 - core::panicking::panic_fmt::hf50318b79d34c964
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/core/src/panicking.rs:67:14
  17:     0x7f885b41740d - core::panicking::panic::h0e046283eaf71120
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/core/src/panicking.rs:117:5
  18:     0x7f885c96298e - <rustc_hir_typeck[779c0a30e9c08528]::writeback::WritebackCx as rustc_hir[c0e3d940cad26931]::intravisit::Visitor>::visit_expr
  19:     0x7f885c95f244 - <rustc_hir_typeck[779c0a30e9c08528]::writeback::WritebackCx as rustc_hir[c0e3d940cad26931]::intravisit::Visitor>::visit_expr
  20:     0x7f885c95fb1e - <rustc_hir_typeck[779c0a30e9c08528]::writeback::WritebackCx as rustc_hir[c0e3d940cad26931]::intravisit::Visitor>::visit_expr
  21:     0x7f885d780e25 - <rustc_hir_typeck[779c0a30e9c08528]::fn_ctxt::FnCtxt>::resolve_type_vars_in_body
  22:     0x7f885d777bd2 - rustc_hir_typeck[779c0a30e9c08528]::typeck
  23:     0x7f885c7d60b3 - rustc_query_system[14d29c9f668b88ba]::query::plumbing::try_execute_query::<rustc_query_impl[46437a9286a69426]::queries::typeck, rustc_query_impl[46437a9286a69426]::plumbing::QueryCtxt>
  24:     0x7f885ddf77db - rustc_data_structures[e01e904362c563e3]::sync::par_for_each_in::<&[rustc_span[ea3aeb57fd08449]::def_id::LocalDefId], <rustc_middle[3bee5fa4b5ed62c7]::hir::map::Map>::par_body_owners<rustc_hir_typeck[779c0a30e9c08528]::typeck_item_bodies::{closure#0}>::{closure#0}>
  25:     0x7f885ddf75af - rustc_hir_typeck[779c0a30e9c08528]::typeck_item_bodies
  26:     0x7f885de6a637 - rustc_query_system[14d29c9f668b88ba]::query::plumbing::try_execute_query::<rustc_query_impl[46437a9286a69426]::queries::typeck_item_bodies, rustc_query_impl[46437a9286a69426]::plumbing::QueryCtxt>
  27:     0x7f885de6a34c - <rustc_query_impl[46437a9286a69426]::Queries as rustc_middle[3bee5fa4b5ed62c7]::ty::query::QueryEngine>::typeck_item_bodies
  28:     0x7f885dae5bef - <rustc_session[de7bc1a706471cae]::session::Session>::time::<(), rustc_hir_analysis[849915a09ceaa25b]::check_crate::{closure#7}>
  29:     0x7f885dae3897 - rustc_hir_analysis[849915a09ceaa25b]::check_crate
  30:     0x7f885dadd9e1 - rustc_interface[5bb7432590c6b07d]::passes::analysis
  31:     0x7f885de67d82 - rustc_query_system[14d29c9f668b88ba]::query::plumbing::try_execute_query::<rustc_query_impl[46437a9286a69426]::queries::analysis, rustc_query_impl[46437a9286a69426]::plumbing::QueryCtxt>
  32:     0x7f885de67a80 - <rustc_query_impl[46437a9286a69426]::Queries as rustc_middle[3bee5fa4b5ed62c7]::ty::query::QueryEngine>::analysis
  33:     0x7f885d8f4f9e - <rustc_interface[5bb7432590c6b07d]::queries::QueryResult<&rustc_middle[3bee5fa4b5ed62c7]::ty::context::GlobalCtxt>>::enter::<core[c98013cfbe010fb2]::result::Result<(), rustc_span[ea3aeb57fd08449]::ErrorGuaranteed>, rustc_driver_impl[6aaaaf3365eb2b0a]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  34:     0x7f885d8f3a7e - <rustc_interface[5bb7432590c6b07d]::interface::Compiler>::enter::<rustc_driver_impl[6aaaaf3365eb2b0a]::run_compiler::{closure#1}::{closure#2}, core[c98013cfbe010fb2]::result::Result<core[c98013cfbe010fb2]::option::Option<rustc_interface[5bb7432590c6b07d]::queries::Linker>, rustc_span[ea3aeb57fd08449]::ErrorGuaranteed>>
  35:     0x7f885d8f1c31 - rustc_span[ea3aeb57fd08449]::set_source_map::<core[c98013cfbe010fb2]::result::Result<(), rustc_span[ea3aeb57fd08449]::ErrorGuaranteed>, rustc_interface[5bb7432590c6b07d]::interface::run_compiler<core[c98013cfbe010fb2]::result::Result<(), rustc_span[ea3aeb57fd08449]::ErrorGuaranteed>, rustc_driver_impl[6aaaaf3365eb2b0a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  36:     0x7f885d8f11df - std[86a3120b066ce118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5bb7432590c6b07d]::util::run_in_thread_pool_with_globals<rustc_interface[5bb7432590c6b07d]::interface::run_compiler<core[c98013cfbe010fb2]::result::Result<(), rustc_span[ea3aeb57fd08449]::ErrorGuaranteed>, rustc_driver_impl[6aaaaf3365eb2b0a]::run_compiler::{closure#1}>::{closure#0}, core[c98013cfbe010fb2]::result::Result<(), rustc_span[ea3aeb57fd08449]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c98013cfbe010fb2]::result::Result<(), rustc_span[ea3aeb57fd08449]::ErrorGuaranteed>>
  37:     0x7f885d8f0bbe - <<std[86a3120b066ce118]::thread::Builder>::spawn_unchecked_<rustc_interface[5bb7432590c6b07d]::util::run_in_thread_pool_with_globals<rustc_interface[5bb7432590c6b07d]::interface::run_compiler<core[c98013cfbe010fb2]::result::Result<(), rustc_span[ea3aeb57fd08449]::ErrorGuaranteed>, rustc_driver_impl[6aaaaf3365eb2b0a]::run_compiler::{closure#1}>::{closure#0}, core[c98013cfbe010fb2]::result::Result<(), rustc_span[ea3aeb57fd08449]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c98013cfbe010fb2]::result::Result<(), rustc_span[ea3aeb57fd08449]::ErrorGuaranteed>>::{closure#1} as core[c98013cfbe010fb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f885b3c42d5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h245efbc079a0cdd8
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/alloc/src/boxed.rs:1976:9
  39:     0x7f885b3c42d5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc1a24c73fee76425
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/alloc/src/boxed.rs:1976:9
  40:     0x7f885b3c42d5 - std::sys::unix::thread::Thread::new::thread_start::hfc741dc8224d9dca
                               at /rustc/3a8a131e9509c478ece1c58fe0ea2d49463d2300/library/std/src/sys/unix/thread.rs:108:17
  41:     0x7f885b291609 - start_thread
  42:     0x7f885b1b4133 - clone
  43:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (3a8a131e9 2023-04-02) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `bar`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `playground` (lib)
