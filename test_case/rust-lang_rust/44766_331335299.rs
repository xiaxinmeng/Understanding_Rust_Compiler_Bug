
$ RUST_BACKTRACE=1 ./x.py build
Updating submodules
    Finished dev [unoptimized] target(s) in 0.0 secs
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling syntax v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/libsyntax)
   Compiling rustc_const_math v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_const_math)
   Compiling proc_macro v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/libproc_macro)
   Compiling rustc_back v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_back)
   Compiling syntax_ext v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/libsyntax_ext)
   Compiling rustc v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc)
   Compiling rustc_allocator v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_allocator)
   Compiling rustc_incremental v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_incremental)
   Compiling rustc_const_eval v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_const_eval)
   Compiling rustc_resolve v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_resolve)
   Compiling rustc_metadata v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_metadata)
   Compiling rustc_trans_utils v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_trans_utils)
   Compiling rustc_typeck v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_typeck)
   Compiling rustc_privacy v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_privacy)
   Compiling rustc_passes v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_passes)
   Compiling rustc_mir v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_mir)
   Compiling rustc_lint v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_lint)
   Compiling rustc_trans v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_trans)
   Compiling rustc_save_analysis v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_save_analysis)
   Compiling rustc_borrowck v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_borrowck)
   Compiling rustc_plugin v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_plugin)
   Compiling rustc_driver v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_driver)
   Compiling rustc-main v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/rustc)
    Finished release [optimized] target(s) in 718.57 secs
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/libcore)
   Compiling unwind v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/libunwind)
   Compiling libc v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/rustc/libc_shim)
   Compiling compiler_builtins v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/rustc/compiler_builtins_shim)
   Compiling rustc_tsan v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_tsan)
   Compiling rustc_lsan v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_lsan)
   Compiling rustc_asan v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_asan)
   Compiling rustc_msan v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/librustc_msan)
   Compiling alloc_jemalloc v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/liballoc_jemalloc)
   Compiling std v0.0.0 (file:///home/sunjay/Documents/projects/rust/src/libstd)
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.22.0-dev running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'path resolved multiple times (PathResolution { base_def: Trait(DefId { krate: CrateNum(0), node: DefIndex(0:3185) }), unresolved_segments: 0 } before, PathResolution { base_def: Trait(DefId { krate: CrateNum(0), node: DefIndex(0:3185) }), unresolved_segments: 0 } now)', src/librustc_resolve/lib.rs:3460:12
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: std::panicking::begin_panic_fmt
   7: rustc_resolve::Resolver::record_def
   8: rustc_resolve::Resolver::smart_resolve_path_fragment
   9: rustc_resolve::Resolver::smart_resolve_path
  10: <rustc_resolve::Resolver<'a> as syntax::visit::Visitor<'tcx>>::visit_generics
  11: syntax::visit::walk_impl_item
  12: rustc_resolve::Resolver::with_type_parameter_rib
  13: rustc_resolve::Resolver::with_current_self_type
  14: rustc_resolve::Resolver::with_self_rib
  15: rustc_resolve::Resolver::with_optional_trait_ref
  16: rustc_resolve::Resolver::with_self_rib
  17: rustc_resolve::Resolver::resolve_item
  18: syntax::visit::walk_item
  19: rustc_resolve::Resolver::resolve_item
  20: rustc_resolve::Resolver::resolve_crate
  21: rustc_driver::driver::phase_2_configure_and_expand
  22: rustc_driver::driver::compile_input
  23: rustc_driver::run_compiler

error: Could not compile `core`.

Caused by:
  process didn't exit successfully: `/home/sunjay/Documents/projects/rust/build/bootstrap/debug/rustc --crate-name core src/libcore/lib.rs --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=3181dd9e46400ebd -C extra-filename=-3181dd9e46400ebd --out-dir /home/sunjay/Documents/projects/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/sunjay/Documents/projects/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/sunjay/Documents/projects/rust/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
warning: build failed, waiting for other jobs to finish...
error: build failed
thread 'main' panicked at 'command did not execute successfully: "/home/sunjay/Documents/projects/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/home/sunjay/Documents/projects/rust/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', src/bootstrap/compile.rs:883:8
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:397
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:572
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: bootstrap::compile::run_cargo
   8: <bootstrap::compile::Std as bootstrap::builder::Step>::run
   9: bootstrap::builder::Builder::ensure
  10: <bootstrap::compile::Test as bootstrap::builder::Step>::run
  11: bootstrap::builder::Builder::ensure
  12: <bootstrap::compile::Rustc as bootstrap::builder::Step>::run
  13: bootstrap::builder::Builder::ensure
  14: <bootstrap::compile::Assemble as bootstrap::builder::Step>::run
  15: bootstrap::builder::Builder::ensure
  16: bootstrap::builder::Builder::compiler
  17: <bootstrap::compile::Std as bootstrap::builder::Step>::make_run
  18: bootstrap::builder::StepDescription::maybe_run
  19: bootstrap::builder::StepDescription::run
  20: bootstrap::builder::Builder::run
  21: bootstrap::Build::build
  22: bootstrap::main
  23: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:99
  24: std::rt::lang_start
             at /checkout/src/libstd/panicking.rs:459
             at /checkout/src/libstd/panic.rs:361
             at /checkout/src/libstd/rt.rs:61
  25: main
  26: __libc_start_main
  27: _start
failed to run: /home/sunjay/Documents/projects/rust/build/bootstrap/debug/bootstrap build
Build completed unsuccessfully in 0:12:04
