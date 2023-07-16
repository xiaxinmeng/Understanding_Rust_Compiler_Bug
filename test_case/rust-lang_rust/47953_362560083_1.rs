
...
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (file:///home/paulg/Projets/rust-lang/rust-alexcrichton/src/libcore)
error: internal compiler error: librustc/ty/context.rs:263: node unknown node (id=1) with HirId::owner DefId(0/0:0 ~ core[d915]) cannot be placed in TypeckTables with local_id_root DefId(0/0:1640 ~ core[d915]::panicking[0]::panic_fmt[0])

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:535:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::bug
   7: <std::thread::local::LocalKey<T>>::with
   8: rustc::ty::context::tls::with_opt
   9: rustc::session::opt_span_bug_fmt
  10: rustc::session::bug_fmt
  11: <std::thread::local::LocalKey<T>>::with
  12: rustc::ty::context::tls::with
  13: rustc::ty::context::TypeckTables::node_id_to_type_opt
  14: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O>>::process_formals
  15: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_foreign_item
  16: syntax::visit::walk_item
  17: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  18: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  19: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_mod
  20: <rustc_save_analysis::DumpHandler<'a> as rustc_save_analysis::SaveHandler>::save
  21: rustc::dep_graph::graph::DepGraph::with_ignore
  22: rustc_driver::enable_save_analysis::{{closure}}::{{closure}}
  23: rustc_driver::enable_save_analysis::{{closure}}
  24: rustc::dep_graph::graph::DepGraph::with_ignore
  25: rustc_driver::driver::compile_input::{{closure}}
  26: <std::thread::local::LocalKey<T>>::with
  27: <std::thread::local::LocalKey<T>>::with
  28: rustc_driver::driver::compile_input
  29: rustc_driver::run_compiler

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-dev running on x86_64-unknown-linux-gnu

error: Could not compile `core`.

Caused by:
  process didn't exit successfully: `/home/paulg/Projets/rust-lang/rust-alexcrichton/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=40f5c600e4b14118 -C extra-filename=-40f5c600e4b14118 --out-dir /home/paulg/Projets/rust-lang/rust-alexcrichton/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/paulg/Projets/rust-lang/rust-alexcrichton/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/paulg/Projets/rust-lang/rust-alexcrichton/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
thread 'main' panicked at 'command did not execute successfully: "/home/paulg/Projets/rust-lang/rust-alexcrichton/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "8" "--release" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/home/paulg/Projets/rust-lang/rust-alexcrichton/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', bootstrap/compile.rs:1061:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at /checkout/src/libstd/sys_common/backtrace.rs:68
             at /checkout/src/libstd/sys_common/backtrace.rs:57
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:397
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:538
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: bootstrap::compile::run_cargo
             at bootstrap/compile.rs:1061
   8: <bootstrap::compile::Std as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:109
   9: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:732
  10: <bootstrap::compile::Test as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:343
  11: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:732
  12: <bootstrap::compile::Rustc as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:459
  13: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:732
  14: <bootstrap::compile::Assemble as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:885
  15: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:732
  16: bootstrap::builder::Builder::compiler
             at bootstrap/builder.rs:339
  17: <bootstrap::compile::Std as bootstrap::builder::Step>::make_run
             at bootstrap/compile.rs:56
  18: bootstrap::builder::StepDescription::maybe_run
             at bootstrap/builder.rs:151
  19: bootstrap::builder::StepDescription::run
             at bootstrap/builder.rs:163
  20: bootstrap::builder::Builder::run
             at bootstrap/builder.rs:326
  21: bootstrap::Build::build
             at bootstrap/lib.rs:378
  22: bootstrap::main
             at bootstrap/bin/main.rs:29
  23: std::rt::lang_start::{{closure}}
             at /checkout/src/libstd/rt.rs:74
  24: std::panicking::try::do_call
             at /checkout/src/libstd/rt.rs:59
             at /checkout/src/libstd/panicking.rs:480
  25: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:101
  26: std::rt::lang_start_internal
             at /checkout/src/libstd/panicking.rs:459
             at /checkout/src/libstd/panic.rs:365
             at /checkout/src/libstd/rt.rs:58
  27: std::rt::lang_start
             at /checkout/src/libstd/rt.rs:74
  28: main
  29: __libc_start_main
  30: _start
failed to run: /home/paulg/Projets/rust-lang/rust-alexcrichton/build/bootstrap/debug/bootstrap build
Build completed unsuccessfully in 0:00:18
