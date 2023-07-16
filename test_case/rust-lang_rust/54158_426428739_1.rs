
   Compiling core v0.0.0 (/tmp/portage/dev-lang/rust-9999/work/rust-git-src/src/libcore)
     Running `/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format jso
n --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=7430e8604190d953 -C extra-filename=-7430e8604190d953 --out-dir /tmp/portage/dev-lang/rust-9
999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/tmp/por
tage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/tmp/portage/dev-lang/
rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -L /usr/lib64/rust-1.29.0/ -L /usr/lib64/rust-1.29.0/rustlib//lib/`
rustc command: "LD_LIBRARY_PATH"="/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1/lib:/tmp/portage/dev-lang/rust-9999/
work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1-std/release/deps:/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stag
e0/lib" "/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--crate-name" "core" "libcore/lib.rs" "--color" "
always" "--crate-type" "lib" "--emit=dep-info,link" "-C" "opt-level=2" "-C" "metadata=7430e8604190d953-rustc" "-C" "extra-filename=-7430e8604190d953" "--out-d
ir" "/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unk
nown-linux-gnu" "-L" "dependency=/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/
deps" "-L" "dependency=/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1-std/release/deps" "-L" "/usr/lib64/rust-1.29.0/
" "-L" "/usr/lib64/rust-1.29.0/rustlib//lib/" "--cfg" "stage1" "--sysroot" "/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/s
tage1" "-Cprefer-dynamic" "-Clinker=x86_64-pc-linux-gnu-gcc" "-C" "debug-assertions=y" "-Zsave-analysis" "-Z" "force-unstable-if-unmarked" "-Dwarnings" "-Dbar
e_trait_objects"
sysroot: "/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1"
libdir: "/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1/lib"
error: internal compiler error: librustc/ty/context.rs:273: node unknown node (id=651901) with HirId::owner DefId(0/0:0 ~ core[565c]) cannot be placed in Type
ckTables with local_id_root DefId(0/0:1817 ~ core[565c]::panicking[0]::panic_fmt[0])

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:599:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc::ty::context::validate_hir_id_for_typeck_tables::{{closure}}
  15: rustc::ty::context::tls::with::{{closure}}
  16: rustc::ty::context::tls::with_context::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_context
  19: rustc::ty::context::tls::with
  20: rustc::ty::context::TypeckTables::node_id_to_type_opt
  21: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O>>::process_formals
  22: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_foreign_item
  23: syntax::visit::walk_item
  24: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  25: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  26: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_mod
  27: syntax::visit::walk_crate
  28: <rustc_save_analysis::DumpHandler<'a> as rustc_save_analysis::SaveHandler>::save
  29: rustc::ty::context::tls::with_context
  30: rustc_save_analysis::process_crate
  31: rustc_driver::enable_save_analysis::{{closure}}::{{closure}}
  32: rustc::util::common::time
  33: rustc_driver::enable_save_analysis::{{closure}}
  34: rustc::ty::context::tls::with_context
  35: rustc_driver::driver::compile_input::{{closure}}
  36: rustc::ty::context::tls::enter_context
  37: <std::thread::local::LocalKey<T>>::with
  38: rustc::ty::context::TyCtxt::create_and_enter
  39: rustc_driver::driver::compile_input
  40: rustc_driver::run_compiler_with_pool
  41: rustc_driver::driver::spawn_thread_pool
  42: rustc_driver::run_compiler
  43: syntax::with_globals
  44: __rust_maybe_catch_panic
  45: std::panicking::try
  46: rustc_driver::run
  47: rustc_driver::main
  48: std::rt::lang_start::{{closure}}
  49: std::panicking::try::do_call
  50: __rust_maybe_catch_panic
  51: std::panicking::try
  52: std::rt::lang_start_internal
  53: main
  54: __libc_start_main
  55: _start
query stack during panic:
end of query stack
error: aborting due to previous error

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.31.0-dev (1c5e9c68e 2018-10-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C linker=x86_64-pc-linux-gnu-gcc -C debug-assertions=y --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `core`.

Caused by:
  process didn't exit successfully: `/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=7430e8604190d953 -C extra-filename=-7430e8604190d953 --out-dir /tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -L /usr/lib64/rust-1.29.0/ -L /usr/lib64/rust-1.29.0/rustlib//lib/` (exit code: 101)
command did not execute successfully: "/tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "1" "-v" "--release" "--locked" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/tmp/portage/dev-lang/rust-9999/work/rust-git-src/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:477
   5: std::panicking::begin_panic
             at libstd/panicking.rs:411
   6: bootstrap::compile::run_cargo
             at bootstrap/compile.rs:1112
   7: <bootstrap::compile::Std as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:115
   8: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:1215
   9: <bootstrap::compile::Test as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:356
  10: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:1215
  11: <bootstrap::compile::Rustc as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:478
  12: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:1215
  13: <bootstrap::compile::Assemble as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:970
  14: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:1215
  15: bootstrap::builder::Builder::compiler
             at bootstrap/builder.rs:580
  16: <bootstrap::compile::Std as bootstrap::builder::Step>::make_run
             at bootstrap/compile.rs:55
  17: bootstrap::builder::StepDescription::maybe_run
             at bootstrap/builder.rs:191
  18: bootstrap::builder::StepDescription::run
             at bootstrap/builder.rs:215
  19: bootstrap::builder::Builder::run_step_descriptions
             at bootstrap/builder.rs:572
  20: bootstrap::builder::Builder::execute_cli
             at bootstrap/builder.rs:562
  21: bootstrap::Build::build
             at bootstrap/lib.rs:485
  22: bootstrap::main
             at bootstrap/bin/main.rs:29
  23: std::rt::lang_start::{{closure}}
             at libstd/rt.rs:74
  24: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  25: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  26: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  27: std::rt::lang_start
             at libstd/rt.rs:74
  28: main
  29: __libc_start_main
  30: _start
Traceback (most recent call last):
  File "./x.py", line 20, in <module>
    bootstrap.main()
  File "/tmp/portage/dev-lang/rust-9999/work/rust-git-src/src/bootstrap/bootstrap.py", line 855, in main
    bootstrap(help_triggered)
  File "/tmp/portage/dev-lang/rust-9999/work/rust-git-src/src/bootstrap/bootstrap.py", line 841, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/tmp/portage/dev-lang/rust-9999/work/rust-git-src/src/bootstrap/bootstrap.py", line 151, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /tmp/portage/dev-lang/rust-9999/work/rust-git-src/build/bootstrap/debug/bootstrap build --verbose --config=/tmp/portage/dev-lang/rust-9999/work/rust-git-src/config.toml -j1 --exclude src/tools/miri
