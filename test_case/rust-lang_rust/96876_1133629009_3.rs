
error: internal compiler error: compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:868:17: `resolve_ty_and_res_fully_qualified_call` called on `LangItem`

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1160:9
stack backtrace:
   0:       0x3fa3a2d206 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha2794daee6ba9423
   1:       0x3fa3a790a0 - core::fmt::write::ha45966a0d0bf9feb
   2:       0x3fa3a20092 - std::io::Write::write_fmt::h08378143163f41c9
   3:       0x3fa3a304b2 - std::panicking::default_hook::{{closure}}::h221571fdb3d8c73d
   4:       0x3fa3a301ea - std::panicking::default_hook::h345b3fce6c89ab24
   5:       0x3fa415cae6 - rustc_driver[9005cb7fb2104183]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:       0x3fa3a30ba6 - std::panicking::rust_panic_with_hook::h8ffebecf7a9398ab
   7:       0x3fa5a31014 - std[c958d8f0cdffa67e]::panicking::begin_panic::<rustc_errors[85b203b8ba770dd8]::ExplicitBug>::{closure#0}
   8:       0x3fa5a304f2 - std[c958d8f0cdffa67e]::sys_common::backtrace::__rust_end_short_backtrace::<std[c958d8f0cdffa67e]::panicking::begin_panic<rustc_errors[85b203b8ba770dd8]::ExplicitBug>::{closure#0}, !>
   9:       0x3fa40f8cfe - std[c958d8f0cdffa67e]::panicking::begin_panic::<rustc_errors[85b203b8ba770dd8]::ExplicitBug>
  10:       0x3fa5a2f3fa - std[c958d8f0cdffa67e]::panic::panic_any::<rustc_errors[85b203b8ba770dd8]::ExplicitBug>
  11:       0x3fa5a24fc0 - <rustc_errors[85b203b8ba770dd8]::HandlerInner>::bug
  12:       0x3fa5a212b4 - <rustc_errors[85b203b8ba770dd8]::Handler>::bug
  13:       0x3fa58aa74a - rustc_middle[18ecd2b671ace77]::ty::context::tls::with_opt::<rustc_middle[18ecd2b671ace77]::util::bug::opt_span_bug_fmt<rustc_span[9b21865714f79047]::span_encoding::Span>::{closure#0}, ()>
  14:       0x3fa58d6b7c - rustc_middle[18ecd2b671ace77]::util::bug::opt_span_bug_fmt::<rustc_span[9b21865714f79047]::span_encoding::Span>
  15:       0x3fa40e882e - rustc_middle[18ecd2b671ace77]::util::bug::bug_fmt
  16:       0x3fa4608b9c - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::resolve_ty_and_res_fully_qualified_call
  17:       0x3fa45f96d2 - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_expr_path
  18:       0x3fa45f153c - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  19:       0x3fa460b376 - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_argument_types
  20:       0x3fa45a56b0 - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  21:       0x3fa45a2c9e - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_call
  22:       0x3fa45f153c - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  23:       0x3fa459c646 - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::demand_scrutinee_type
  24:       0x3fa46fbea8 - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_match
  25:       0x3fa45f153c - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:       0x3fa460db0c - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_decl_initializer
  27:       0x3fa460dbd6 - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_decl
  28:       0x3fa460dd22 - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_stmt
  29:       0x3fa460e520 - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  30:       0x3fa45f153c - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  31:       0x3fa45f9e86 - <rustc_typeck[d8d854df65b69f8b]::check::fn_ctxt::FnCtxt>::check_return_expr
  32:       0x3fa4701e58 - rustc_typeck[d8d854df65b69f8b]::check::check::check_fn
  33:       0x3fa4679256 - rustc_typeck[d8d854df65b69f8b]::check::typeck
  34:       0x3fa4ea48bc - rustc_query_system[444d3042d9776945]::query::plumbing::try_execute_query::<rustc_query_impl[fac207277026ba76]::plumbing::QueryCtxt, rustc_query_system[444d3042d9776945]::query::caches::DefaultCache<rustc_span[9b21865714f79047]::def_id::LocalDefId, &rustc_middle[18ecd2b671ace77]::ty::context::TypeckResults>>
  35:       0x3fa5035a66 - <rustc_query_impl[fac207277026ba76]::Queries as rustc_middle[18ecd2b671ace77]::ty::query::QueryEngine>::typeck
  36:       0x3fa467c990 - rustc_typeck[d8d854df65b69f8b]::check::typeck_item_bodies
  37:       0x3fa4f0d19e - rustc_query_system[444d3042d9776945]::query::plumbing::try_execute_query::<rustc_query_impl[fac207277026ba76]::plumbing::QueryCtxt, rustc_query_system[444d3042d9776945]::query::caches::DefaultCache<(), ()>>
  38:       0x3fa5035944 - <rustc_query_impl[fac207277026ba76]::Queries as rustc_middle[18ecd2b671ace77]::ty::query::QueryEngine>::typeck_item_bodies
  39:       0x3fa46fac40 - rustc_typeck[d8d854df65b69f8b]::check_crate
  40:       0x3fa4226470 - rustc_interface[995760d1abdadbfd]::passes::analysis
  41:       0x3fa4efbd7c - rustc_query_system[444d3042d9776945]::query::plumbing::try_execute_query::<rustc_query_impl[fac207277026ba76]::plumbing::QueryCtxt, rustc_query_system[444d3042d9776945]::query::caches::DefaultCache<(), core[3b62df0531f623c4]::result::Result<(), rustc_errors[85b203b8ba770dd8]::ErrorReported>>>
  42:       0x3fa50220c0 - <rustc_query_impl[fac207277026ba76]::Queries as rustc_middle[18ecd2b671ace77]::ty::query::QueryEngine>::analysis
  43:       0x3fa4142db4 - rustc_interface[995760d1abdadbfd]::interface::run_compiler::<core[3b62df0531f623c4]::result::Result<(), rustc_errors[85b203b8ba770dd8]::ErrorReported>, rustc_driver[9005cb7fb2104183]::run_compiler::{closure#1}>::{closure#0}
  44:       0x3fa4174ecc - std[c958d8f0cdffa67e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[995760d1abdadbfd]::util::run_in_thread_pool_with_globals<rustc_interface[995760d1abdadbfd]::interface::run_compiler<core[3b62df0531f623c4]::result::Result<(), rustc_errors[85b203b8ba770dd8]::ErrorReported>, rustc_driver[9005cb7fb2104183]::run_compiler::{closure#1}>::{closure#0}, core[3b62df0531f623c4]::result::Result<(), rustc_errors[85b203b8ba770dd8]::ErrorReported>>::{closure#0}, core[3b62df0531f623c4]::result::Result<(), rustc_errors[85b203b8ba770dd8]::ErrorReported>>
  45:       0x3fa413df4a - <<std[c958d8f0cdffa67e]::thread::Builder>::spawn_unchecked_<rustc_interface[995760d1abdadbfd]::util::run_in_thread_pool_with_globals<rustc_interface[995760d1abdadbfd]::interface::run_compiler<core[3b62df0531f623c4]::result::Result<(), rustc_errors[85b203b8ba770dd8]::ErrorReported>, rustc_driver[9005cb7fb2104183]::run_compiler::{closure#1}>::{closure#0}, core[3b62df0531f623c4]::result::Result<(), rustc_errors[85b203b8ba770dd8]::ErrorReported>>::{closure#0}, core[3b62df0531f623c4]::result::Result<(), rustc_errors[85b203b8ba770dd8]::ErrorReported>>::{closure#1} as core[3b62df0531f623c4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:       0x3fa3a39de6 - std::sys::unix::thread::Thread::new::thread_start::h018c2571d3ddafbc
  47:       0x3fa6469b72 - start
                               at /home/buildozer/aports/main/musl/src/v1.2.3/src/thread/pthread_create.c:203:2

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0 running on riscv64-alpine-linux-musl

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
