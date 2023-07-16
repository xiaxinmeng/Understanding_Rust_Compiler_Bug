
mkdir -p build/i386-unknown-redox/debug
RUST_BACKTRACE=1 rustc --target=i386-unknown-redox.json -L build/i386-unknown-redox/debug -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code -C ar=i386-elf-ar -C linker=i386-elf-gcc -o build/i386-unknown-redox/debug/libcore.rlib rust/src/libcore/lib.rs
RUST_BACKTRACE=1 rustc --target=i386-unknown-redox.json -L build/i386-unknown-redox/debug -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code -C ar=i386-elf-ar -C linker=i386-elf-gcc -o build/i386-unknown-redox/debug/libio.rlib crates/io/lib.rs
RUST_BACKTRACE=1 rustc --target=i386-unknown-redox.json -L build/i386-unknown-redox/debug -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code -C ar=i386-elf-ar -C linker=i386-elf-gcc -o build/i386-unknown-redox/debug/liballoc_system.rlib liballoc_system/lib.rs
RUST_BACKTRACE=1 rustc --target=i386-unknown-redox.json -L build/i386-unknown-redox/debug -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code -C ar=i386-elf-ar -C linker=i386-elf-gcc -o build/i386-unknown-redox/debug/liballoc.rlib rust/src/liballoc/lib.rs
RUST_BACKTRACE=1 rustc --target=i386-unknown-redox.json -L build/i386-unknown-redox/debug -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code -C ar=i386-elf-ar -C linker=i386-elf-gcc -o build/i386-unknown-redox/debug/librustc_unicode.rlib rust/src/librustc_unicode/lib.rs
RUST_BACKTRACE=1 rustc --target=i386-unknown-redox.json -L build/i386-unknown-redox/debug -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code -C ar=i386-elf-ar -C linker=i386-elf-gcc -o build/i386-unknown-redox/debug/libcollections.rlib rust/src/libcollections/lib.rs
RUST_BACKTRACE=1 rustc --target=i386-unknown-redox.json -L build/i386-unknown-redox/debug -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code -C ar=i386-elf-ar -C linker=i386-elf-gcc -o build/i386-unknown-redox/debug/librand.rlib rust/src/librand/lib.rs
RUST_BACKTRACE=1 rustc --target=i386-unknown-redox.json -L build/i386-unknown-redox/debug -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code -C ar=i386-elf-ar -C linker=i386-elf-gcc -o build/i386-unknown-redox/debug/libsystem.rlib crates/system/lib.rs
RUST_BACKTRACE=1 rustc --target=i386-unknown-redox.json -L build/i386-unknown-redox/debug -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code -C ar=i386-elf-ar -C linker=i386-elf-gcc -o build/i386-unknown-redox/debug/libstd.rlib libstd/src/lib.rs -L native=libc/lib/
error: internal compiler error: ../src/librustc_metadata/encoder.rs:218: encode_symbol: id not found 49044
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/errors/mod.rs:575
stack backtrace:
   1:        0x10b8e4838 - std::sys::backtrace::tracing::imp::write::h714760a4c8c0cdd8
   2:        0x10b8f0c75 - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hff309ab1d83ffd90
   3:        0x10b8f07af - std::panicking::default_hook::h08ad3bb09872855b
   4:        0x10b8b5396 - std::sys_common::unwind::begin_unwind_inner::h406d5f1a330b854b
   5:        0x10aeadc3a - std::sys_common::unwind::begin_unwind::hb10e2b9ef92fd4a6
   6:        0x10aeada69 - syntax::errors::Handler::bug::h112319de7e910c9e
   7:        0x10a26babc - rustc::session::opt_span_bug_fmt::_$u7b$$u7b$closure$u7d$$u7d$::h2c926d3e6a2eb430
   8:        0x10a26b921 - rustc::session::opt_span_bug_fmt::he2c7fbeab8bf3d1d
   9:        0x10a282119 - rustc::session::bug_fmt::h9f67e9cd7aa5a073
  10:        0x10754e9c0 - rustc_metadata::encoder::encode_symbol::h165ac8a162612975
  11:        0x10755a1d3 - rustc_metadata::encoder::encode_info_for_item::h60f93cd3dd34a5cf
  12:        0x10756a42d - rustc_metadata::encoder::encode_metadata_inner::ha8580aa6ef04664a
  13:        0x107565fe9 - rustc_metadata::encoder::encode_metadata::h4120aac9f1041ba6
  14:        0x1075c1a1f - rustc_metadata::csearch::_<impl rustc..middle..cstore..CrateStore<'tcx> for cstore..CStore>::encode_metadata::he92619b644b6d919
  15:        0x107947262 - rustc_trans::base::trans_crate::_$u7b$$u7b$closure$u7d$$u7d$::h6042c55692ac48ec
  16:        0x107939f0d - rustc_trans::base::trans_crate::h45d3e3b1384148fb
  17:        0x106fcf3aa - rustc_driver::driver::phase_4_translate_to_llvm::h8d5ab3a4d94eae6a
  18:        0x106fcdb5c - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::hd32acfaba89541b2
  19:        0x106fca4ed - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h8148bf3bd7eb17b2
  20:        0x106fc40a5 - rustc::ty::context::TyCtxt::create_and_enter::h963727672a0d2ef1
  21:        0x106fc0aac - rustc_driver::driver::phase_3_run_analysis_passes::h035bf2dff707c080
  22:        0x106f946e1 - rustc_driver::driver::compile_input::h650fe5b01cb8d74d
  23:        0x106f7aeff - rustc_driver::run_compiler::h68d23e0e9b7b247d
  24:        0x106f78292 - std::sys_common::unwind::try::try_fn::h67fde221a73148bc
  25:        0x10b8e1fcb - __rust_try
  26:        0x10b8e1f53 - std::sys_common::unwind::inner_try::h4e97625a08807651
  27:        0x106f78b29 - _<F as std..boxed..FnBox<A>>::call_box::hc8936fa120642c49
  28:        0x10b8efb48 - std::sys::thread::Thread::new::thread_start::h74af400293164137
  29:     0x7fff8626f99c - _pthread_body
  30:     0x7fff8626f919 - _pthread_start

Makefile:434: recipe for target 'build/i386-unknown-redox/debug/libstd.rlib' failed
make: *** [build/i386-unknown-redox/debug/libstd.rlib] Error 101
