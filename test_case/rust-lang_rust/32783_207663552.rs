
RUST_BACKTRACE=1 rustc --target=i386-unknown-redox.json -L build/i386-unknown-redox/debug -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code -C ar=i386-elf-ar -C linker=i386-elf-gcc -o build/i386-unknown-redox/debug/libstd.rlib libstd/src/lib.rs -L native=libc/lib/
error: internal compiler error: ../src/librustc_metadata/encoder.rs:218: encode_symbol: id not found 49044
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/errors/mod.rs:575
stack backtrace:
   1:        0x11252d838 - std::sys::backtrace::tracing::imp::write::h714760a4c8c0cdd8
   2:        0x112539c75 - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hff309ab1d83ffd90
   3:        0x1125397af - std::panicking::default_hook::h08ad3bb09872855b
   4:        0x1124fe396 - std::sys_common::unwind::begin_unwind_inner::h406d5f1a330b854b
   5:        0x111b06c3a - std::sys_common::unwind::begin_unwind::hb10e2b9ef92fd4a6
   6:        0x111b06a69 - syntax::errors::Handler::bug::h112319de7e910c9e
   7:        0x110ed6abc - rustc::session::opt_span_bug_fmt::_$u7b$$u7b$closure$u7d$$u7d$::h2c926d3e6a2eb430
   8:        0x110ed6921 - rustc::session::opt_span_bug_fmt::he2c7fbeab8bf3d1d
   9:        0x110eed119 - rustc::session::bug_fmt::h9f67e9cd7aa5a073
  10:        0x10e1c59c0 - rustc_metadata::encoder::encode_symbol::h165ac8a162612975
  11:        0x10e1d11d3 - rustc_metadata::encoder::encode_info_for_item::h60f93cd3dd34a5cf
  12:        0x10e1e142d - rustc_metadata::encoder::encode_metadata_inner::ha8580aa6ef04664a
  13:        0x10e1dcfe9 - rustc_metadata::encoder::encode_metadata::h4120aac9f1041ba6
  14:        0x10e238a1f - rustc_metadata::csearch::_<impl rustc..middle..cstore..CrateStore<'tcx> for cstore..CStore>::encode_metadata::he92619b644b6d919
  15:        0x10e5bd262 - rustc_trans::base::trans_crate::_$u7b$$u7b$closure$u7d$$u7d$::h6042c55692ac48ec
  16:        0x10e5aff0d - rustc_trans::base::trans_crate::h45d3e3b1384148fb
  17:        0x10dc4d3aa - rustc_driver::driver::phase_4_translate_to_llvm::h8d5ab3a4d94eae6a
  18:        0x10dc4bb5c - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::hd32acfaba89541b2
  19:        0x10dc484ed - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h8148bf3bd7eb17b2
  20:        0x10dc420a5 - rustc::ty::context::TyCtxt::create_and_enter::h963727672a0d2ef1
  21:        0x10dc3eaac - rustc_driver::driver::phase_3_run_analysis_passes::h035bf2dff707c080
  22:        0x10dc126e1 - rustc_driver::driver::compile_input::h650fe5b01cb8d74d
  23:        0x10dbf8eff - rustc_driver::run_compiler::h68d23e0e9b7b247d
  24:        0x10dbf6292 - std::sys_common::unwind::try::try_fn::h67fde221a73148bc
  25:        0x11252afcb - __rust_try
  26:        0x11252af53 - std::sys_common::unwind::inner_try::h4e97625a08807651
  27:        0x10dbf6b29 - _<F as std..boxed..FnBox<A>>::call_box::hc8936fa120642c49
  28:        0x112538b48 - std::sys::thread::Thread::new::thread_start::h74af400293164137
  29:     0x7fff8f80199c - _pthread_body
  30:     0x7fff8f801919 - _pthread_start
