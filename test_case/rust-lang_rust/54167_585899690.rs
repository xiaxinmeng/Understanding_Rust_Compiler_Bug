`
Updating only changed submodules
Submodules updated in 0.03 seconds
    Finished dev [unoptimized] target(s) in 0.12s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.19s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.16s
Installing libLLVM.so to stage 0 (x86_64-unknown-linux-gnu)
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Assuming that sanitizers rebuild is not necessary. To force a rebuild, remove the file `/home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/native/sanitizers/sanitizers-finished-building`
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling std v0.0.0 (/home/matthias/vcs/github/rust/src/libstd)
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: "read_option: expected 0 for None or 1 for Some"', src/librustc_metadata/rmeta/decoder.rs:225:9
stack backtrace:
   0:     0x7f57a3b1a118 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h73e4f0a35ffe6046
   1:     0x7f57a3b5e3ac - core::fmt::write::h8559a176d71a1ba9
   2:     0x7f57a3b19715 - std::io::Write::write_fmt::h4b423f3f674f6fb2
   3:     0x7f57a3affb45 - std::panicking::default_hook::{{closure}}::h3501b889eb26920d
   4:     0x7f57a3aff826 - std::panicking::default_hook::h7c34178090913852
   5:     0x7f57a51ac9f9 - rustc_driver::report_ice::hb57aa685137a6e81
   6:     0x7f57a3b00335 - std::panicking::rust_panic_with_hook::h23c5e790c3f509ba
   7:     0x7f57a3affe3b - rust_begin_unwind
   8:     0x7f57a3b66b31 - core::panicking::panic_fmt::h5f21779ba417ac9c
   9:     0x7f57a3b66bc3 - core::result::unwrap_failed::h815738a38039b680
  10:     0x7f57a7d11a01 - rustc_metadata::rmeta::decoder::<impl rustc_metadata::rmeta::Lazy<T>>::decode::hd56580b5ea9b17ef
  11:     0x7f57a7e17816 - rustc_metadata::rmeta::decoder::CrateMetadata::get_optimized_mir::he6ab81ab5fd912f3
  12:     0x7f57a7d319b6 - rustc_metadata::rmeta::decoder::cstore_impl::provide_extern::optimized_mir::he789c3463fbf877a
  13:     0x7f57a7650a89 - rustc::ty::query::__query_compute::optimized_mir::h0a49e31b5deb2685
  14:     0x7f57a756ea5b - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute::h012ae31d64922dfe
  15:     0x7f57a7829271 - rustc::dep_graph::graph::DepGraph::with_task_impl::h8eaa08467e9fa246
  16:     0x7f57a75a0d27 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h59da62a81262bb38
  17:     0x7f57a788ce49 - rustc_mir::interpret::eval_context::InterpCx<M>::load_mir::h6da638b8b1b858c4
  18:     0x7f57a7a13446 - rustc_mir::const_eval::eval_queries::const_eval_raw_provider::h0aadede088df5cfc
  19:     0x7f57a7650d55 - rustc::ty::query::__query_compute::const_eval_raw::h140577af68d5c5df
  20:     0x7f57a756eadf - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute::h51232fa4d0e97d28
  21:     0x7f57a782c366 - rustc::dep_graph::graph::DepGraph::with_task_impl::hb0f6e6272733fe39
  22:     0x7f57a75e8d68 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hfbc100c0d7ab68c2
  23:     0x7f57a7a122d3 - rustc_mir::const_eval::eval_queries::const_eval_validated_provider::h30f3cfa522771c77
  24:     0x7f57a7651f25 - rustc::ty::query::__query_compute::const_eval_validated::h93393798929b014a
  25:     0x7f57a756edaf - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute::h49b4ee36b609124e
  26:     0x7f57a782e966 - rustc::dep_graph::graph::DepGraph::with_task_impl::hd7642e8a79de40bc
  27:     0x7f57a75b03f6 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h8c557ff3523377c9
  28:     0x7f57a7a120bb - rustc_mir::const_eval::eval_queries::const_eval_validated_provider::h30f3cfa522771c77
  29:     0x7f57a86f1f4f - rustc::ty::query::__query_compute::const_eval_validated::hd5a825e01248714f
  30:     0x7f57a891b9df - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute::h49b4ee36b609124e
  31:     0x7f57a8304002 - rustc::dep_graph::graph::DepGraph::with_task_impl::hda130eb4393abe11
  32:     0x7f57a87cb125 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h317454190c113cae
  33:     0x7f57a87a24ac - rustc::mir::interpret::queries::<impl rustc::ty::context::TyCtxt>::const_eval_resolve::h563b3d6189bca81b
  34:     0x7f57a836b166 - rustc::ty::sty::Const::try_eval_bits::he5a8daafcacf10a8
  35:     0x7f57a8357f64 - rustc::ty::inhabitedness::<impl rustc::ty::TyS>::uninhabited_from::h116629611b4ae4fe
  36:     0x7f57a83573cd - rustc::ty::inhabitedness::<impl rustc::ty::FieldDef>::uninhabited_from::h4c148e9b1f98315f
  37:     0x7f57a8356d03 - rustc::ty::inhabitedness::<impl rustc::ty::VariantDef>::uninhabited_from::h10c3a3bb5992cfbb
  38:     0x7f57a8357ff5 - rustc::ty::inhabitedness::<impl rustc::ty::TyS>::uninhabited_from::h116629611b4ae4fe
  39:     0x7f57a87ae113 - rustc::ty::inhabitedness::<impl rustc::ty::context::TyCtxt>::is_ty_uninhabited_from::h4df27b705758926c
  40:     0x7f57a7513a0a - rustc_mir_build::hair::pattern::_match::is_useful::h4bb7849be0b03e82
  41:     0x7f57a744c403 - rustc_mir_build::hair::pattern::check_match::check_not_useful::h379548b40839069b
  42:     0x7f57a750fba2 - rustc_mir_build::hair::pattern::_match::MatchCheckCtxt::create_and_enter::hbd42d319a3cd9a9e
  43:     0x7f57a744aed8 - <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_local::h4ddcc102221fee5e
  44:     0x7f57a742c784 - rustc_hir::intravisit::walk_expr::h869d99dab0ad00cb
  45:     0x7f57a744acd8 - <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr::h126010ad3e2ed9f3
  46:     0x7f57a744acd8 - <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr::h126010ad3e2ed9f3
  47:     0x7f57a744aaa7 - rustc_mir_build::hair::pattern::check_match::check_match::hdff1292b3efcaac9
  48:     0x7f57a5353d4c - rustc::ty::query::__query_compute::check_match::h4e54e74b399b3b53
  49:     0x7f57a540cb1b - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_match>::compute::h5b6c76d79abe1ed0
  50:     0x7f57a5405f5e - rustc::dep_graph::graph::DepGraph::with_task_impl::he2f59497af4e1b22
  51:     0x7f57a5411daf - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h043fb0b3907eaaba
  52:     0x7f57a540c88d - rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::h79ad300effbd07e5
  53:     0x7f57a53f5c90 - std::panicking::try::do_call::hd6d4e83b5b80ca2d
  54:     0x7f57a3b29757 - __rust_maybe_catch_panic
  55:     0x7f57a53a4f67 - rustc_session::utils::<impl rustc_session::session::Session>::time::h70cfd643260358d7
  56:     0x7f57a54579fd - rustc_interface::passes::analysis::h8acc80b9913b96f8
  57:     0x7f57a51d7526 - rustc::ty::query::__query_compute::analysis::h76f4894cb5afa49f
  58:     0x7f57a523e4a3 - rustc::dep_graph::graph::DepGraph::with_task_impl::hbbf179f68d76f996
  59:     0x7f57a51b5a3a - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h2f1dbdcc5dd3fa23
  60:     0x7f57a523c20d - rustc::ty::context::tls::enter_global::h81e05442bbb907c5
  61:     0x7f57a521bbc5 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hbeee3a4fdcc04d5f
  62:     0x7f57a51d388e - scoped_tls::ScopedKey<T>::set::had37a036c22d7305
  63:     0x7f57a51b8b62 - syntax::attr::with_globals::hf5f93eaa884c4a17
  64:     0x7f57a51d4576 - std::sys_common::backtrace::__rust_begin_short_backtrace::hfc95484f7330eaff
  65:     0x7f57a3b29757 - __rust_maybe_catch_panic
  66:     0x7f57a5212358 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h80863f14a9968b25
  67:     0x7f57a3af4e3f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::he2e224d150246d6d
  68:     0x7f57a3b1ae82 - std::sys_common::thread::start_thread::h51aa2804f675b9e5
  69:     0x7f57a3b037e6 - std::sys::unix::thread::Thread::new::thread_start::hcb48ea36c631b01c
  70:     0x7f57a37814cf - start_thread
  71:     0x7f57a39412d3 - clone
  72:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z save-analysis -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=3 -C debuginfo=0 -C target-cpu=native -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type dylib --crate-type rlib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] processing `libc::sockaddr_storage::__ss_pad2::{{constant}}#0`
#1 [const_eval_raw] const-evaluating `libc::sockaddr_storage::__ss_pad2::{{constant}}#0`
#2 [const_eval_validated] const-evaluating + checking `libc::sockaddr_storage::__ss_pad2::{{constant}}#0`
#3 [const_eval_validated] const-evaluating + checking `libc::sockaddr_storage::__ss_pad2::{{constant}}#0`
#4 [check_match] processing `sys_common::net::sockname`
#5 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `std`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "4" "--release" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/home/matthias/vcs/github/rust/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /home/matthias/vcs/github/rust/build/bootstrap/debug/bootstrap build
Build completed unsuccessfully in 0:00:03
