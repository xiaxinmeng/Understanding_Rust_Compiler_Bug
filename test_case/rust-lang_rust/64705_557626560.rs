
rustc --version --verbose
rustc 1.39.0 (4560ea788 2019-11-04)
binary: rustc
commit-hash: 4560ea788cb760f0a34127156c78e2552949f734
commit-date: 2019-11-04
host: x86_64-apple-darwin
release: 1.39.0
LLVM version: 9.0

error: internal compiler error: src/librustc/hir/def.rs:345: attempted .def_id() on invalid res: Err

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:812:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::HandlerInner::bug
   8: rustc_errors::Handler::bug
   9: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  10: rustc::ty::context::tls::with_opt::{{closure}}
  11: rustc::ty::context::tls::with_context_opt
  12: rustc::ty::context::tls::with_opt
  13: rustc::util::bug::opt_span_bug_fmt
  14: rustc::util::bug::bug_fmt
  15: rustc::hir::def::Res<Id>::def_id::{{closure}}
  16: rustdoc::clean::register_res
  17: <syntax::source_map::Spanned<rustc::hir::VisibilityKind> as rustdoc::clean::Clean<core::option::Option<rustdoc::clean::Visibility>>>::clean
  18: <rustc::hir::ImplItem as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  19: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  20: <rustdoc::doctree::Impl as rustdoc::clean::Clean<alloc::vec::Vec<rustdoc::clean::Item>>>::clean
  21: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend
  22: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  23: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  24: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  25: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  26: rustdoc::clean::krate
  27: rustc::ty::context::tls::enter_global
  28: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  29: rustc_interface::passes::create_global_ctxt::{{closure}}
  30: rustc_interface::passes::BoxedGlobalCtxt::enter
  31: rustc_interface::interface::run_compiler_in_existing_thread_pool
  32: rustdoc::core::run_core
  33: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  34: std::panicking::try::do_call
  35: __rust_maybe_catch_panic
  36: rustc_driver::catch_fatal_errors
  37: rustdoc::main_options
  38: std::thread::local::LocalKey<T>::with
  39: scoped_tls::ScopedKey<T>::set
  40: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
 Documenting rayon-core v1.6.1
error: aborting due to previous error

error: Could not document `lexical-core`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name lexical_core /Users/frjansso/.cargo/registry/src/github.com-1ecc6299db9ec823/lexical-core-0.6.2/src/lib.rs --cap-lints allow -o /Users/frjansso/dev/rust/advent_of_code2018/target/doc --cfg 'feature="arrayvec"' --cfg 'feature="correct"' --cfg 'feature="ryu"' --cfg 'feature="std"' --cfg 'feature="table"' --color always -L dependency=/Users/frjansso/dev/rust/advent_of_code2018/target/debug/deps --extern arrayvec=/Users/frjansso/dev/rust/advent_of_code2018/target/debug/deps/libarrayvec-3f025bb2d28518a6.rmeta --extern cfg_if=/Users/frjansso/dev/rust/advent_of_code2018/target/debug/deps/libcfg_if-426c934331571334.rmeta --extern ryu=/Users/frjansso/dev/rust/advent_of_code2018/target/debug/deps/libryu-8a280f8b9b571987.rmeta --extern static_assertions=/Users/frjansso/dev/rust/advent_of_code2018/target/debug/deps/libstatic_assertions-512ad2d38db95142.rmeta --cfg has_range_bounds --cfg has_slice_index --cfg has_full_range_inclusive --cfg has_const_index --cfg has_i128 --cfg has_ops_bound --cfg has_pointer_methods --cfg has_range_inclusive --cfg limb_width_64` (exit code: 1)
warning: build failed, waiting for other jobs to finish...
error: build failed
