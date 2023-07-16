
error: internal compiler error: ../src/librustc_metadata/encoder.rs:220: encode_symbol: id not found 49044
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with RUST_BACKTRACE=1 for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/errors/mod.rs:575
stack backtrace:
   1:        0x10f734978 - std::sys::backtrace::tracing::imp::write::h714760a4c8c0cdd8
   2:        0x10f740dc5 - std::panicking::defaulthook::$u7b$$u7b$closure$u7d$$u7d$::hff309ab1d83ffd90
   3:        0x10f7408fd - std::panicking::default_hook::h08ad3bb09872855b
   4:        0x10f704196 - std::sys_common::unwind::begin_unwind_inner::h406d5f1a330b854b
   5:        0x10ece5cea - std::sys_common::unwind::begin_unwind::hb10e2b9ef92fd4a6
   6:        0x10ece5b19 - syntax::errors::Handler::bug::h112319de7e910c9e
   7:        0x10e072c9c - rustc::session::opt_span_bugfmt::$u7b$$u7b$closure$u7d$$u7d$::h2c926d3e6a2eb430
   8:        0x10e072b01 - rustc::session::opt_span_bug_fmt::he2c7fbeab8bf3d1d
   9:        0x10e0816a9 - rustc::session::bug_fmt::h9f67e9cd7aa5a073
  10:        0x10b68a4f0 - rustc_metadata::encoder::encode_symbol::h165ac8a162612975
  11:        0x10b69710b - rustc_metadata::encoder::encode_info_for_item::hfab1a46f9bf30c3c
  12:        0x10b69dc71 - rustc::front::map::Map::with_path_next::h6ba163a93b4eab46
  13:        0x10b69dca4 - rustc::front::map::Map::with_path_next::h6ba163a93b4eab46
  14:        0x10b69dca4 - rustc::front::map::Map::with_pathnext::h6ba163a93b4eab46
  15:        0x10b6a0d4e - <encoder..EncodeVisitor<'a, 'b, 'c, 'tcx> as rustc_front..intravisit..Visitor<'tcx>>::visit_item::h1cde6db68ffd78b8
  16:        0x10b6a7f5d - rustc_metadata::encoder::encode_metadata_inner::hcfb24759120a6c88
  17:        0x10b6a3b19 - rustc_metadata::encoder::encode_metadata::ha3e422357afb6bf6
  18:        0x10b70007f - rustcmetadata::csearch::<impl rustc..middle..cstore..CrateStore<'tcx> for cstore..CStore>::encode_metadata::h88a2f338ac9ea48a
  19:        0x10b885ee2 - rustc_trans::base::transcrate::$u7b$$u7b$closure$u7d$$u7d$::h6042c55692ac48ec
  20:        0x10b877dfd - rustc_trans::base::trans_crate::h45d3e3b1384148fb
  21:        0x10ae1cce0 - rustc_driver::driver::phase_4_translate_to_llvm::h8d5ab3a4d94eae6a
  22:        0x10ae1b434 - rustc_driver::driver::compileinput::$u7b$$u7b$closure$u7d$$u7d$::hd32acfaba89541b2
  23:        0x10ae1802d - rustc_driver::driver::phase_3_run_analysispasses::$u7b$$u7b$closure$u7d$$u7d$::h1692b932d4d7b15b
  24:        0x10ae121a5 - rustc::ty::context::TyCtxt::create_and_enter::h926ea4fe45f15d00
  25:        0x10ae0eba5 - rustc_driver::driver::phase_3_run_analysis_passes::h3c035e5cf4758407
  26:        0x10ade2af1 - rustc_driver::driver::compile_input::h650fe5b01cb8d74d
  27:        0x10adc904f - rustc_driver::run_compiler::h68d23e0e9b7b247d
  28:        0x10adc63e2 - std::sys_common::unwind::try::try_fn::h67fde221a73148bc
  29:        0x10f731fdb - __rust_try
  30:        0x10f731f63 - std::sys_common::unwind::innertry::h4e97625a08807651
  31:        0x10adc6c79 - <F as std..boxed..FnBox<A>>::call_box::hc8936fa120642c49
  32:        0x10f73fc88 - std::sys::thread::Thread::new::thread_start::h74af400293164137
  33:     0x7fff9302199c - _pthread_body
  34:     0x7fff93021919 - _pthread_start

make: * [build/i386-unknown-redox/debug/libstd.rlib] Error 101
