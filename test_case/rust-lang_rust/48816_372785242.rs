
19:05:10.870 /home/alex/project/atom-ide-rust/node_modules/atom-languageclient/build/lib/logger.js:9 Rust (RLS) stderr thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:482:9
19:05:10.870 /home/alex/project/atom-ide-rust/node_modules/atom-languageclient/build/lib/logger.js:9 Rust (RLS) stderr note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
19:05:10.870 /home/alex/project/atom-ide-rust/node_modules/atom-languageclient/build/lib/logger.js:9 Rust (RLS) stderr stack backtrace:
19:05:10.870 /home/alex/project/atom-ide-rust/node_modules/atom-languageclient/build/lib/logger.js:9 Rust (RLS) stderr    0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
19:05:10.870 /home/alex/project/atom-ide-rust/node_modules/atom-languageclient/build/lib/logger.js:9 Rust (RLS) stderr              at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
19:05:10.870 /home/alex/project/atom-ide-rust/node_modules/atom-languageclient/build/lib/logger.js:9 Rust (RLS) stderr    1: std::sys_common::backtrace::_print
19:05:10.870 /home/alex/project/atom-ide-rust/node_modules/atom-languageclient/build/lib/logger.js:9 Rust (RLS) stderr              at libstd/sys_common/backtrace.rs:71
19:05:10.870 /home/alex/project/atom-ide-rust/node_modules/atom-languageclient/build/lib/logger.js:9 Rust (RLS) stderr    2: std::panicking::default_hook::{{closure}}
stderr              at libstd/sys_common/backtrace.rs:59
stderr              at libstd/panicking.rs:207
stderr    3: std::panicking::default_hook
stderr              at libstd/panicking.rs:223
stderr    4: core::ops::function::Fn::call
stderr    5: std::panicking::rust_panic_with_hook
stderr              at libstd/panicking.rs:403
stderr    6: std::panicking::begin_panic
stderr    7: rustc_errors::Handler::span_bug
stderr    8: rustc::session::opt_span_bug_fmt::{{closure}}
stderr    
stderr 9: rustc::ty::context::tls::with_opt::{{closure}}
stderr   10: <std::thread::local::LocalKey<T>>::try_with
stderr   11: <std::thread::local::LocalKey<T>>::with
stderr   12: rustc::ty::context::tls::with
stderr   13: rustc::ty::context::tls::with_opt
stderr   14: rustc::session::opt_span_bug_fmt
stderr   15: rustc::session::span_bug_fmt
stderr   16: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_expr
stderr   17: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_expr
stderr   18: syntax::visit::walk_expr
stderr   19: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_expr
stderr   20: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O>>::process_method
stderr   21: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O>>::process_impl_item
stderr   22: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>
stderr ::visit_item
stderr   23: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
stderr   24: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
stderr   25: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_mod
stderr   26: <rustc_save_analysis::CallbackHandler<'b> as rustc_save_analysis::SaveHandler>::save
stderr   27: <rls::build::rustc::RlsRustcCalls as rustc_driver::CompilerCalls<'a>>::build_controller::{{closure}}
stderr   28: rustc_driver::driver::compile_input::{{closure}}
stderr   29: rustc::ty::context::TyCtxt::create_and_enter
stderr   30: rustc_driver::driver::compile_input
stderr   31: rustc_driver::run_compiler
stderr note: the compiler unexpectedly panicked. this is a bug.
stderr note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
stderr note: rustc 1.26.0-nightly (2789b067d 2018-03-06) running on x86_64-unknown-linux-gnu
