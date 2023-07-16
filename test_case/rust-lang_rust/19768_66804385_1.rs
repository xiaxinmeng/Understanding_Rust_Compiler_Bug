
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'assertion failed: end <= self.len()', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libcore/slice.rs:432

stack backtrace:
   1:        0x10a41b5c0 - rt::backtrace::imp::write::hd3da47399b536f3dgVx
   2:        0x10a41e7d0 - failure::on_fail::hcbaa4351fd1cd347Uhy
   3:        0x10a670d95 - unwind::begin_unwind_inner::haf8c40ccae1da676FBc
   4:        0x10a670a77 - unwind::begin_unwind_fmt::h87e7c1472ee8c5812yc
   5:        0x10a670792 - rust_begin_unwind
   6:        0x10a6be4ac - panicking::panic_fmt::hc20ce06aae9c0de4Rtl
   7:        0x10a6b8f32 - panicking::panic::h572d9d2d545626c0hrl
   8:        0x107c8899d - metadata::loader::Context<'a>::extract_one::hdcc6f00df22da57bPww
   9:        0x107c8194e - metadata::loader::Context<'a>::find_library_crate::ha32a500828f7e95bbnw
  10:        0x107c7c93f - metadata::creader::PluginMetadataReader<'a>::read_plugin_metadata::h8f31ae7fa13eec2ffdv
  11:        0x107cb2c1d - plugin::load::PluginLoader<'a>.Visitor<'v>::visit_view_item::ha74f181071d8dd35E6z
  12:        0x107cb1fbb - plugin::load::load_plugins::h0b07b6b282d9e473u5z
  13:        0x106e488ff - driver::phase_2_configure_and_expand::hf7f3c8e52f612c09Qha
  14:        0x106e3c92f - driver::compile_input::h6cedb9382e2b9854pba
  15:        0x106ed42bd - run_compiler::h542f569c873ccc9aEYb
  16:        0x106ed2d1e - run::closure.21411
  17:        0x106ee415e - task::TaskBuilder::try_future::closure.22865
  18:        0x10a3f4843 - task::TaskBuilder::spawn_internal::closure.30641
  19:        0x10a66e99d - task::Task::spawn::closure.5568
  20:        0x10a6d616c - rust_try_inner
  21:        0x10a6d6156 - rust_try
  22:        0x10a66ea77 - unwind::try::h20dac7ad984fece2Wqc
  23:        0x10a66e84c - task::Task::run::h28163a638413e040fIb
  24:        0x10a66e54f - task::Task::spawn::closure.5544
  25:        0x10a66fe57 - thread::thread_start::h20523d95201137f0wZb
  26:     0x7fff933b62fc - _pthread_body
  27:     0x7fff933b6279 - _pthread_body
