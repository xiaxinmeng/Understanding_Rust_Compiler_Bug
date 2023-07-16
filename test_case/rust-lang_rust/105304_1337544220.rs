`
thread 'rustc' panicked at 'Box<dyn Any>', /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:973:33
stack backtrace:
   0:     0x7feed858a5c4 - std::backtrace_rs::backtrace::libunwind::trace::hc8277bb3a00e0049
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7feed858a5c4 - std::backtrace_rs::backtrace::trace_unsynchronized::h7171069a8a6029ec
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7feed858a5c4 - std::sys_common::backtrace::_print_fmt::h9b8d864b9cacdf65
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7feed858a5c4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h30059ecb241f0374
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7feed2deabd8 - core::fmt::write::h7d3fc703cf396835
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/fmt/mod.rs:1208:17
   5:     0x7feed85630df - std::io::Write::write_fmt::h0070d228c09bbaad
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/io/mod.rs:1682:15
   6:     0x7feed858a3c5 - std::sys_common::backtrace::_print::hcfecaa3f0a2ed334
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7feed858a3c5 - std::sys_common::backtrace::print::hb72c8184e0b6a41e
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7feed8572587 - std::panicking::default_hook::{{closure}}::h483836af052c1481
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:267:22
   9:     0x7feed8572247 - std::panicking::default_hook::h1e612ca3289dc41d
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:286:9
  10:     0x7feed58b5a32 - <alloc[48e4295a588061ab]::boxed::Box<dyn for<'a, 'b> core[a14b18922c5685d]::ops::function::Fn<(&'a core[a14b18922c5685d]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[a14b18922c5685d]::marker::Sync + core[a14b18922c5685d]::marker::Send> as core[a14b18922c5685d]::ops::function::Fn<(&core[a14b18922c5685d]::panic::panic_info::PanicInfo,)>>::call
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:2032:9
  11:     0x7feed58b5a32 - rustc_driver[9a27ed754b60ba1d]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:1202:13
  12:     0x7feed8572fac - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::ha7249c7081218c8a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:2032:9
  13:     0x7feed8572fac - std::panicking::rust_panic_with_hook::h80de57e62ee80af8
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:692:13
  14:     0x7feed60875a3 - std[a43d24b5a93ee688]::panicking::begin_panic::<rustc_errors[3b65cac499643c0c]::ExplicitBug>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:608:9
  15:     0x7feed60800a6 - std[a43d24b5a93ee688]::sys_common::backtrace::__rust_end_short_backtrace::<std[a43d24b5a93ee688]::panicking::begin_panic<rustc_errors[3b65cac499643c0c]::ExplicitBug>::{closure#0}, !>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:137:18
  16:     0x7feed61962d6 - std[a43d24b5a93ee688]::panicking::begin_panic::<rustc_errors[3b65cac499643c0c]::ExplicitBug>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:607:12
  17:     0x7feed61962c6 - std[a43d24b5a93ee688]::panic::panic_any::<rustc_errors[3b65cac499643c0c]::ExplicitBug>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:61:5
  18:     0x7feed618f844 - <rustc_errors[3b65cac499643c0c]::HandlerInner>::span_bug::<rustc_span[ef98d0b2f0389c7a]::span_encoding::Span, &alloc[48e4295a588061ab]::string::String>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:1514:9
  19:     0x7feed618f387 - <rustc_errors[3b65cac499643c0c]::Handler>::span_bug::<rustc_span[ef98d0b2f0389c7a]::span_encoding::Span, &alloc[48e4295a588061ab]::string::String>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:973:9
  20:     0x7feed6119d5b - rustc_middle[6c487b6d7a03859d]::util::bug::opt_span_bug_fmt::<rustc_span[ef98d0b2f0389c7a]::span_encoding::Span>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/util/bug.rs:34:40
  21:     0x7feed6119d5b - rustc_middle[6c487b6d7a03859d]::ty::context::tls::with_opt::<rustc_middle[6c487b6d7a03859d]::util::bug::opt_span_bug_fmt<rustc_span[ef98d0b2f0389c7a]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:2065:40
  22:     0x7feed6119d5b - rustc_middle[6c487b6d7a03859d]::ty::context::tls::with_context_opt::<rustc_middle[6c487b6d7a03859d]::ty::context::tls::with_opt<rustc_middle[6c487b6d7a03859d]::util::bug::opt_span_bug_fmt<rustc_span[ef98d0b2f0389c7a]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:2017:22
  23:     0x7feed6119a77 - rustc_middle[6c487b6d7a03859d]::ty::context::tls::with_opt::<rustc_middle[6c487b6d7a03859d]::util::bug::opt_span_bug_fmt<rustc_span[ef98d0b2f0389c7a]::span_encoding::Span>::{closure#0}, ()>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:2065:9
  24:     0x7feed6119a77 - rustc_middle[6c487b6d7a03859d]::util::bug::opt_span_bug_fmt::<rustc_span[ef98d0b2f0389c7a]::span_encoding::Span>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/util/bug.rs:31:5
  25:     0x7feed6119a35 - rustc_middle[6c487b6d7a03859d]::util::bug::span_bug_fmt::<rustc_span[ef98d0b2f0389c7a]::span_encoding::Span>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/util/bug.rs:22:5
  26:     0x7feed5f5236b - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::suggest_missing_break_or_return_expr::{closure#2}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/suggestions.rs:926:21
  27:     0x7feed5f5236b - <core[a14b18922c5685d]::option::Option<rustc_middle[6c487b6d7a03859d]::ty::Ty>>::unwrap_or_else::<<rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::suggest_missing_break_or_return_expr::{closure#2}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/option.rs:828:21
  28:     0x7feed5f5236b - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::suggest_missing_break_or_return_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/suggestions.rs:925:40
  29:     0x7feed617af92 - <rustc_hir_typeck[c8ab3e7e38adbcad]::coercion::CoerceMany<&rustc_hir[3788c4a2d17ef2a2]::hir::Expr>>::report_return_mismatched_types
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/coercion.rs:1730:13
  30:     0x7feed6189d65 - <rustc_hir_typeck[c8ab3e7e38adbcad]::coercion::CoerceMany<&rustc_hir[3788c4a2d17ef2a2]::hir::Expr>>::coerce_inner
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/coercion.rs:1547:31
  31:     0x7feed5f44bb9 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1449:17
  32:     0x7feed5f44bb9 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::with_breakable_ctxt::<<rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}, ()>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs:1428:22
  33:     0x7feed5f44bb9 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_block_with_expected
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1429:26
  34:     0x7feed5f9c951 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_kind
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:329:41
  35:     0x7feed5f21178 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:237:18
  36:     0x7feed5f21178 - stacker[580307df02ffed7e]::maybe_grow::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  37:     0x7feed5f21178 - rustc_data_structures[d31a97b6c18021ec]::stack::ensure_sufficient_stack::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  38:     0x7feed5f21178 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:233:18
  39:     0x7feed5f9b3e5 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:187:9
  40:     0x7feed5f9bea2 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_kind
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:343:39
  41:     0x7feed5f21178 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:237:18
  42:     0x7feed5f21178 - stacker[580307df02ffed7e]::maybe_grow::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  43:     0x7feed5f21178 - rustc_data_structures[d31a97b6c18021ec]::stack::ensure_sufficient_stack::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  44:     0x7feed5f21178 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:233:18
  45:     0x7feed5f9b3e5 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:187:9
  46:     0x7feed5f44844 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1436:50
  47:     0x7feed5f44844 - <core[a14b18922c5685d]::option::Option<&&rustc_hir[3788c4a2d17ef2a2]::hir::Expr>>::map::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/option.rs:925:29
  48:     0x7feed5f44844 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1436:32
  49:     0x7feed5f44844 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::with_breakable_ctxt::<<rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}, ()>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs:1428:22
  50:     0x7feed5f44844 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_block_with_expected
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1429:26
  51:     0x7feed5f9c951 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_kind
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:329:41
  52:     0x7feed5f21178 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:237:18
  53:     0x7feed5f21178 - stacker[580307df02ffed7e]::maybe_grow::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  54:     0x7feed5f21178 - rustc_data_structures[d31a97b6c18021ec]::stack::ensure_sufficient_stack::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  55:     0x7feed5f21178 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:233:18
  56:     0x7feed5f9b3e5 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:187:9
  57:     0x7feed5f23056 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_hint
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:139:9
  58:     0x7feed5f23056 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_return_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:834:30
  59:     0x7feed60f4646 - rustc_hir_typeck[c8ab3e7e38adbcad]::check::check_fn
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/check.rs:121:9
  60:     0x7feed5f923b4 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_closure::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/closure.rs:83:31
  61:     0x7feed5f923b4 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_closure
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/closure.rs:72:19
  62:     0x7feed5f923b4 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_closure
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/closure.rs:61:9
  63:     0x7feed5f9c698 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_kind
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:328:43
  64:     0x7feed5f21178 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:237:18
  65:     0x7feed5f21178 - stacker[580307df02ffed7e]::maybe_grow::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  66:     0x7feed5f21178 - rustc_data_structures[d31a97b6c18021ec]::stack::ensure_sufficient_stack::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  67:     0x7feed5f21178 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:233:18
  68:     0x7feed5f9b3e5 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:187:9
  69:     0x7feed5f3b3c2 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_argument_types::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:256:30
  70:     0x7feed5f3b3c2 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_argument_types
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:374:34
  71:     0x7feed5f0a8bb - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::confirm_builtin_call
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/callee.rs:460:9
  72:     0x7feed5f0761c - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_call
  73:     0x7feed5f9c2e0 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_kind
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:330:45
  74:     0x7feed5f21178 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:237:18
  75:     0x7feed5f21178 - stacker[580307df02ffed7e]::maybe_grow::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  76:     0x7feed5f21178 - rustc_data_structures[d31a97b6c18021ec]::stack::ensure_sufficient_stack::<rustc_middle[6c487b6d7a03859d]::ty::Ty, <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  77:     0x7feed5f21178 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:233:18
  78:     0x7feed5f9b3e5 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_expectation
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:187:9
  79:     0x7feed5f23056 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_expr_with_hint
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:139:9
  80:     0x7feed5f23056 - <rustc_hir_typeck[c8ab3e7e38adbcad]::fn_ctxt::FnCtxt>::check_return_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/expr.rs:834:30
  81:     0x7feed60f4646 - rustc_hir_typeck[c8ab3e7e38adbcad]::check::check_fn
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/check.rs:121:9
  82:     0x7feed60aedce - rustc_hir_typeck[c8ab3e7e38adbcad]::typeck_with_fallback::<rustc_hir_typeck[c8ab3e7e38adbcad]::typeck::{closure#0}>::{closure#0}::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/lib.rs:251:13
  83:     0x7feed60aedce - <rustc_hir_typeck[c8ab3e7e38adbcad]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[c8ab3e7e38adbcad]::typeck_with_fallback<rustc_hir_typeck[c8ab3e7e38adbcad]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/inherited.rs:105:9
  84:     0x7feed60aedce - rustc_hir_typeck[c8ab3e7e38adbcad]::typeck_with_fallback::<rustc_hir_typeck[c8ab3e7e38adbcad]::typeck::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/lib.rs:234:26
  85:     0x7feed60aedce - rustc_hir_typeck[c8ab3e7e38adbcad]::typeck_with_fallback::<rustc_hir_typeck[c8ab3e7e38adbcad]::typeck::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/lib.rs:217:36
  86:     0x7feed60aedce - rustc_hir_typeck[c8ab3e7e38adbcad]::typeck
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_hir_typeck/src/lib.rs:198:9
  87:     0x7feed75331e7 - <rustc_query_system[d1c7f5502e959b5d]::query::config::QueryVTable<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>>::compute
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/config.rs:66:9
  88:     0x7feed75331e7 - rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job::<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:435:13
  89:     0x7feed75331e7 - stacker[580307df02ffed7e]::maybe_grow::<&rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults, rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  90:     0x7feed75331e7 - rustc_data_structures[d31a97b6c18021ec]::stack::ensure_sufficient_stack::<&rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults, rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  91:     0x7feed75331e7 - <rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt as rustc_query_system[d1c7f5502e959b5d]::query::QueryContext>::start_query::<&rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults, rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:128:17
  92:     0x7feed75331e7 - rustc_middle[6c487b6d7a03859d]::ty::context::tls::enter_context::<<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt as rustc_query_system[d1c7f5502e959b5d]::query::QueryContext>::start_query<&rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults, rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:2000:50
  93:     0x7feed75331e7 - rustc_middle[6c487b6d7a03859d]::ty::context::tls::set_tlv::<rustc_middle[6c487b6d7a03859d]::ty::context::tls::enter_context<<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt as rustc_query_system[d1c7f5502e959b5d]::query::QueryContext>::start_query<&rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults, rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1984:9
  94:     0x7feed75331e7 - rustc_middle[6c487b6d7a03859d]::ty::context::tls::enter_context::<<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt as rustc_query_system[d1c7f5502e959b5d]::query::QueryContext>::start_query<&rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults, rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:2000:9
  95:     0x7feed75331e7 - <rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt as rustc_query_system[d1c7f5502e959b5d]::query::QueryContext>::start_query::<&rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults, rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:127:13
  96:     0x7feed75331e7 - rustc_middle[6c487b6d7a03859d]::ty::context::tls::with_related_context::<<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt as rustc_query_system[d1c7f5502e959b5d]::query::QueryContext>::start_query<&rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults, rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:2044:13
  97:     0x7feed75331e7 - rustc_middle[6c487b6d7a03859d]::ty::context::tls::with_context::<rustc_middle[6c487b6d7a03859d]::ty::context::tls::with_related_context<<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt as rustc_query_system[d1c7f5502e959b5d]::query::QueryContext>::start_query<&rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults, rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:2028:40
  98:     0x7feed75331e7 - rustc_middle[6c487b6d7a03859d]::ty::context::tls::with_context_opt::<rustc_middle[6c487b6d7a03859d]::ty::context::tls::with_context<rustc_middle[6c487b6d7a03859d]::ty::context::tls::with_related_context<<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt as rustc_query_system[d1c7f5502e959b5d]::query::QueryContext>::start_query<&rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults, rustc_query_system[d1c7f5502e959b5d]::query::plumbing::execute_job<rustc_query_impl[d284c1843138b3a0]::plumbing::QueryCtxt, rustc_span[ef98d0b2f0389c7a]::def_id::LocalDefId, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[6c487b6d7a03859d]::ty::context::TypeckResults>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:2017:22
