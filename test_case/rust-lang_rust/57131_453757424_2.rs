
$ RUST_BACKTRACE=full cargo tarpaulin --out Xml
[INFO tarpaulin] Running Tarpaulin
[INFO tarpaulin] Building project
error: internal compiler error: src/librustc/traits/codegen/mod.rs:58: Encountered error `Unimplemented` selecting `Binder(<auto::application::Application as glib::translate::ToGlibPtr<*mut <auto::action_group::ActionGroup as glib::wrapper::Wrapper>::GlibType>>)` during codegen

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:588:9
stack backtrace:
   0:     0x7fbbcf9b0633 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h52a58eda1b111766
                               at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1:     0x7fbbcf9a8c98 - std::sys_common::backtrace::_print::hb1bd9eefebb87041
                               at src/libstd/sys_common/backtrace.rs:70
   2:     0x7fbbcf9ac7c2 - std::panicking::default_hook::{{closure}}::hf4d199fe1ad92983
                               at src/libstd/sys_common/backtrace.rs:58
                               at src/libstd/panicking.rs:200
   3:     0x7fbbcf9ac534 - std::panicking::default_hook::h201273aac19b9edb
                               at src/libstd/panicking.rs:215
   4:     0x7fbbcb59ceff - rustc::util::common::panic_hook::hef7378882c23d1e6
   5:     0x7fbbcf9acfa9 - std::panicking::rust_panic_with_hook::h84fbac26321577dc
                               at src/libstd/panicking.rs:482
   6:     0x7fbbca2c1d9c - std::panicking::begin_panic::hd7cf2196732f81d1
   7:     0x7fbbca2bcdce - rustc_errors::Handler::bug::hea476058307b1df2
   8:     0x7fbbcb1b36ee - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h07c4a516b9a192ff
   9:     0x7fbbcb1b1739 - rustc::ty::context::tls::with_opt::{{closure}}::hdd791b8ae4fb671b
  10:     0x7fbbcb1b1654 - rustc::ty::context::tls::with_context_opt::hf50044ff730d0dbd
  11:     0x7fbbcb1b16e6 - rustc::ty::context::tls::with_opt::h897797804c26e886
  12:     0x7fbbcb1b35f4 - rustc::util::bug::opt_span_bug_fmt::h5ba7581d379ee75f
  13:     0x7fbbcb1b3566 - rustc::util::bug::bug_fmt::h40215c828135d98d
  14:     0x7fbbcb3d5208 - rustc::ty::context::GlobalCtxt::enter_local::he2870f64f62ac9a2
  15:     0x7fbbcb58c7ae - rustc::traits::codegen::codegen_fulfill_obligation::h8f37980871818a9b
  16:     0x7fbbcaf75fa0 - rustc::ty::query::__query_compute::codegen_fulfill_obligation::h04454d0cfb4fd0f0
  17:     0x7fbbcb3e0005 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::codegen_fulfill_obligation<'tcx>>::compute::h182add9f1761effa
  18:     0x7fbbcb01fcac - rustc::dep_graph::graph::DepGraph::with_task_impl::h04e1d18c7e905a51
  19:     0x7fbbcb299536 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with::h9a9907aaa12068bd
  20:     0x7fbbcb6125e6 - rustc::ty::instance::Instance::resolve::h260052a2845a5568
  21:     0x7fbbcc05d19e - <rustc_mir::monomorphize::collector::RootCollector<'b, 'a, 'v> as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item::hfcc7721cc2b834b7
  22:     0x7fbbcbf8cbe8 - rustc::hir::Crate::visit_all_item_likes::h8aad38ba751cf9c7
  23:     0x7fbbcc05889a - rustc_mir::monomorphize::collector::collect_roots::hfdda14c5fa806b66
  24:     0x7fbbcc0e9108 - rustc::util::common::time::h43ea9c1a858c8825
  25:     0x7fbbcc058507 - rustc_mir::monomorphize::collector::collect_crate_mono_items::h8111e5840c845e57
  26:     0x7fbbcc0e979a - rustc::util::common::time::he914d9921cb2d543
  27:     0x7fbbcbeb8903 - rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items::he0f9e56be3701921
  28:     0x7fbbc4d8e806 - rustc::ty::query::__query_compute::collect_and_partition_mono_items::h78243d0533832858
  29:     0x7fbbc4d3d3e8 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::collect_and_partition_mono_items<'tcx>>::compute::hf764cd41a880bc6a
  30:     0x7fbbc4daf7ab - rustc::dep_graph::graph::DepGraph::with_task_impl::hd402888ac3137d6a
  31:     0x7fbbc4d40395 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with::h31bcc80b578326ec
  32:     0x7fbbc4d20dbf - rustc_codegen_ssa::back::symbol_export::exported_symbols_provider_local::h36eb3e4ad74ab578
  33:     0x7fbbcd2eb1a6 - rustc::ty::query::__query_compute::exported_symbols::hf01bd49946e83e92
  34:     0x7fbbcd264c48 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::exported_symbols<'tcx>>::compute::h2ae5e16312580d7a
  35:     0x7fbbcd1c5e2f - rustc::dep_graph::graph::DepGraph::with_task_impl::hc2b24188c74ea6ca
  36:     0x7fbbcd2734f2 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with::h386aae49c9bc9d2a
  37:     0x7fbbcd1578ad - rustc_metadata::encoder::encode_metadata::h751e899534e343f8
  38:     0x7fbbcd23161e - rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata::hd6b73d8f1a2e90b6
  39:     0x7fbbcb3cbc6a - rustc::ty::context::TyCtxt::encode_metadata::h7586e46a3a0797c5
  40:     0x7fbbc514f785 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::write_metadata::h987ee69c8b385b9b
  41:     0x7fbbc524a394 - rustc::util::common::time::h881c0f3373566091
  42:     0x7fbbc51d92d0 - rustc_codegen_ssa::base::codegen_crate::h4b92ef3d3b177cf0
  43:     0x7fbbc5150bb0 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate::he995305b17d9deef
  44:     0x7fbbcfd315a0 - rustc_driver::driver::phase_4_codegen::h33782f9845e8b356
  45:     0x7fbbcfd5e940 - rustc_driver::driver::compile_input::{{closure}}::hda7adc675e3e6e7c
  46:     0x7fbbcfd5ad34 - <std::thread::local::LocalKey<T>>::with::he11ef6ca79b804a8
  47:     0x7fbbcfd1ad16 - rustc_driver::driver::compile_input::h3582cc7d4937429c
  48:     0x7fbbcfca5b20 - rustc_driver::run_compiler_with_pool::h6eab0a4050fe4827
  49:     0x7fbbcfcb0a55 - <scoped_tls::ScopedKey<T>>::set::h7313b1e4ae398688
  50:     0x7fbbcfca496a - rustc_driver::run_compiler::hf937c997197f5af2
  51:     0x7fbbcfcb0c0a - <scoped_tls::ScopedKey<T>>::set::hc282c4c24d822502
  52:     0x7fbbcfd83d62 - std::sys_common::backtrace::__rust_begin_short_backtrace::h76c60934d80213f7
  53:     0x7fbbcf9be559 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:92
  54:     0x7fbbcfd8f280 - <F as alloc::boxed::FnBox<A>>::call_box::hd5e479892b494de2
  55:     0x7fbbcf9bd31d - std::sys::unix::thread::Thread::new::thread_start::hc9f20ddfe96efa84
                               at /rustc/b43986184b8f4e0d633e8ae1704f0e19aec30cb2/src/liballoc/boxed.rs:744
                               at src/libstd/sys_common/thread.rs:14
                               at src/libstd/sys/unix/thread.rs:81
  56:     0x7fbbcf7305a6 - start_thread
  57:     0x7fbbcf05b22e - __GI___clone
  58:                0x0 - <unknown>
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `glib::translate::ToGlibPtr` fulfills its obligations
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
#2 [exported_symbols] exported_symbols
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-nightly (b43986184 2019-01-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C relocation-model=dynamic-no-pic -C link-dead-code -C opt-level=0 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

[ERROR tarpaulin] Failed to compile tests! Error: Could not compile `gio`.
