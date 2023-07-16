
[...]
   Compiling core v0.0.0 (/home/xanewok/repos/rust/src/libcore)
error: internal compiler error: src/librustc/ty/context.rs:214: node type <T>::IntoIter (hir_id=HirId { owner: DefIndex(4040), local_id: 61 }) with HirId::owner DefId(0:4040 ~ core[db27]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0]::flatten[0]) cannot be placed in TypeckTables with local_id_root DefId(0:4036 ~ core[db27]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
stack backtrace:
   0:     0x7fcfb9b9fa82 - std::sys_common::backtrace::print::h3ccd05a8e990e9b0
   1:     0x7fcfb9bafc82 - std::panicking::default_hook::{{closure}}::h2c2369fef3dd8738
   2:     0x7fcfb9baf9f4 - std::panicking::default_hook::h5593167b5f202307
   3:     0x7fcfbc8f07d1 - rustc::util::common::panic_hook::h7cdff5d1edfec23a
   4:     0x7fcfb9bb066e - std::panicking::rust_panic_with_hook::he0d392b5db3b1393
   5:     0x7fcfbce3b83d - std::panicking::begin_panic::hed9199d93294adf2
   6:     0x7fcfbce64fff - rustc_errors::Handler::bug::hcdd16ef9af0b70a5
   7:     0x7fcfbc417ac8 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h172f1a229254071a
   8:     0x7fcfbc412f13 - rustc::ty::context::tls::with_opt::{{closure}}::h20b6cc14b4d4ee22
   9:     0x7fcfbc412e83 - rustc::ty::context::tls::with_context_opt::h4b93cd0baf1cf64c
  10:     0x7fcfbc412ec7 - rustc::ty::context::tls::with_opt::h59612e3fe61d9a6e
  11:     0x7fcfbc4179b8 - rustc::util::bug::opt_span_bug_fmt::h0bcb131f0ef383f3
  12:     0x7fcfbc417922 - rustc::util::bug::bug_fmt::h9efce7ee2f38ac34
  13:     0x7fcfbc753786 - rustc::ty::context::validate_hir_id_for_typeck_tables::{{closure}}::h40b9c9909894d5c5
  14:     0x7fcfbc76c389 - rustc::ty::context::tls::with::{{closure}}::hdfea32d74f0e82f7
  15:     0x7fcfbc76c360 - rustc::ty::context::tls::with_context::{{closure}}::h375e4279d6f7e516
  16:     0x7fcfbc76ae28 - rustc::ty::context::tls::with_context_opt::h2c26a79a9d51d01f
  17:     0x7fcfbc76ae36 - rustc::ty::context::tls::with_context::h472f2429318db06e
  18:     0x7fcfbc76c376 - rustc::ty::context::tls::with::h3db787a9be2b7222
  19:     0x7fcfbc7545fe - rustc::ty::context::TypeckTables::qpath_res::h64f4550ee1debf16
  20:     0x7fcfbb96c6bf - rustc_save_analysis::SaveContext::get_path_res::h75df113a150104f7
  21:     0x7fcfbb9d7c2d - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_ty::h2f81ec7e83d8985e
  22:     0x7fcfbb9bface - syntax::visit::walk_generic_args::h5060edf426757671
  23:     0x7fcfbb9d7dc1 - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_ty::h2f81ec7e83d8985e
  24:     0x7fcfbb9c16ac - syntax::visit::walk_ty::h037aa9d3b0ff40c2
  25:     0x7fcfbb9d46db - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item::h7784b50029a7c2b4
  26:     0x7fcfbb9cb83e - rustc_save_analysis::dump_visitor::DumpVisitor::process_method::h229a40ba297088ca
  27:     0x7fcfbb9d4408 - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item::h7784b50029a7c2b4
  28:     0x7fcfbb9d1c6a - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item::h7784b50029a7c2b4
  29:     0x7fcfbb9d1c6a - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item::h7784b50029a7c2b4
  30:     0x7fcfbb9d1c6a - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item::h7784b50029a7c2b4
  31:     0x7fcfbb9d0a2a - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_mod::h054a0b766f054540
  32:     0x7fcfba237b16 - syntax::visit::walk_crate::hfd3f86321d89f575
  33:     0x7fcfba18ba3b - rustc::dep_graph::graph::DepGraph::with_ignore::hc4aceef6921b5d51
  34:     0x7fcfba24e938 - rustc_save_analysis::process_crate::h209b10f593ffc88a
  35:     0x7fcfba195a23 - rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}::h22fe8b90cef236c4
  36:     0x7fcfba18af91 - rustc::util::common::time::hc42a40104d377b36
  37:     0x7fcfba1d1d1d - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::had5d7435a68c899b
  38:     0x7fcfba3cdb8a - rustc_interface::passes::create_global_ctxt::{{closure}}::hacffd98af6b4442e
  39:     0x7fcfba1d38d2 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h53d03c09ad272798
  40:     0x7fcfba1e96b1 - std::thread::local::LocalKey<T>::with::h9154d3730506d959
  41:     0x7fcfba205a55 - scoped_tls::ScopedKey<T>::set::h369df4d9d9aa6258
  42:     0x7fcfba237952 - syntax::with_globals::h8b475ba4fa6aace5
  43:     0x7fcfba1b306d - std::sys_common::backtrace::__rust_begin_short_backtrace::hc7aee0ada2eefdd2
  44:     0x7fcfba1b5b5e - std::panicking::try::do_call::h001c18685067ae4f
  45:     0x7fcfb9bb1cd4 - __rust_maybe_catch_panic
  46:     0x7fcfba1b5a9a - std::panicking::try::he4318e326d9619df
  47:     0x7fcfba1bd4ee - core::ops::function::FnOnce::call_once{{vtable.shim}}::h8e5aa07c722a4072
  48:     0x7fcfb9ba841f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hbd9c620013ca32c8
  49:     0x7fcfb9bb0f60 - std::sys::unix::thread::Thread::new::thread_start::h081abdb0058386a4
  50:     0x7fcfb904a6db - start_thread
  51:     0x7fcfb984788f - __clone
  52:                0x0 - <unknown>
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `core`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/home/xanewok/repos/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "12" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/xanewok/repos/rust/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /home/xanewok/repos/rust/build/bootstrap/debug/bootstrap build
