
[~/Documents/rust-build-ios]$ make -j3 RUST_BACKTRACE=1                                               [ruby-2.1.2] 
cfg: version 1.1.0-dev (35ceea399 2015-06-19)
cfg: build triple x86_64-apple-darwin
cfg: host triples x86_64-apple-darwin
cfg: target triples x86_64-apple-darwin armv7-apple-ios armv7s-apple-ios i386-apple-ios aarch64-apple-ios x86_64-apple-ios
cfg: non-build target triples armv7-apple-ios armv7s-apple-ios i386-apple-ios aarch64-apple-ios x86_64-apple-ios
cfg: host for x86_64-apple-darwin is x86_64
cfg: host for armv7-apple-ios is armv7
cfg: host for armv7s-apple-ios is armv7s
cfg: host for i386-apple-ios is i386
cfg: host for aarch64-apple-ios is aarch64
cfg: host for x86_64-apple-ios is x86_64
cfg: os for x86_64-apple-darwin is apple-darwin
cfg: os for armv7-apple-ios is apple-ios
cfg: os for armv7s-apple-ios is apple-ios
cfg: os for i386-apple-ios is apple-ios
cfg: os for aarch64-apple-ios is apple-ios
cfg: os for x86_64-apple-ios is apple-ios
cfg: good valgrind for x86_64-apple-darwin is 
cfg: good valgrind for armv7-apple-ios is 
cfg: good valgrind for armv7s-apple-ios is 
cfg: good valgrind for i386-apple-ios is 
cfg: good valgrind for aarch64-apple-ios is 
cfg: good valgrind for x86_64-apple-ios is 
cfg: using CC=clang (CFG_CC)
cfg: using CXX=clang++ (CFG_CXX)
cfg: disabling valgrind run-pass tests
rustc: x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/librustc
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'os error', /Users/arias/Documents/rust/src/libsyntax/diagnostics/plugin.rs:171

stack backtrace:
   1:        0x103544ffc - sys::backtrace::write::he9710ed45a1f2fddc2r
   2:        0x10354cce5 - panicking::on_panic::h651f57d66847d188Lgw
   3:        0x103510cc3 - rt::unwind::begin_unwind_inner::h8ce36f48c4f8d07cuYv
   4:        0x103511a95 - rt::unwind::begin_unwind_fmt::h6e0a2760e6dce819AXv
   5:        0x102d3b8de - diagnostics::plugin::expand_build_diagnostic_array::h13d81b0feb89cd21QNa
   6:        0x100e16ac3 - ext::base::F.TTMacroExpander::expand::h1439180917286580280
   7:        0x102f3f794 - ext::expand::expand_item_mac::h5fe1b3afec5fdc89Bgc
   8:        0x102f35868 - ext::expand::expand_annotatable::h53cf52c7843d40b73Kc
   9:        0x102f32640 - ext::expand::expand_item::hfe47e3cc20ce4f92Bdc
  10:        0x102f3a60f - iter::FlatMap<I, U, F>.Iterator::next::h3360546454327385898
  11:        0x102f3a38b - vec::Vec<T>.FromIterator<T>::from_iter::h10984259839597934138
  12:        0x102f37698 - ext::expand::expand_item_underscore::h459df196ec0e0203cec
  13:        0x102f74858 - fold::noop_fold_item_simple::h1980239792232803492
  14:        0x102f3646e - ext::expand::expand_annotatable::h53cf52c7843d40b73Kc
  15:        0x102f32640 - ext::expand::expand_item::hfe47e3cc20ce4f92Bdc
  16:        0x102f7a777 - ext::expand::expand_crate::hd57edfc3f3c25e28zcd
  17:        0x100315a5a - driver::phase_2_configure_and_expand::closure.20198
  18:        0x1002cee5a - driver::phase_2_configure_and_expand::hfd45f222dc91151aYsa
  19:        0x1002c0c4c - driver::compile_input::h5e52b152a6e61d0dQba
  20:        0x100368e62 - run_compiler::h2bae6ad099a5e6be75b
  21:        0x1003669be - boxed::F.FnBox<A>::call_box::h14492299571019174799
  22:        0x100366187 - rt::unwind::try::try_fn::h1980993045082273195
  23:        0x1035d7368 - rust_try_inner
  24:        0x1035d7355 - rust_try
  25:        0x100366425 - boxed::F.FnBox<A>::call_box::h3280019795504741580
  26:        0x10354bab8 - sys::thread::Thread::new::thread_start::h220539c055074a3aYjv
  27:     0x7fff8b069267 - _pthread_body
  28:     0x7fff8b0691e4 - _pthread_start

make: *** [x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/stamp.rustc] Error 101
