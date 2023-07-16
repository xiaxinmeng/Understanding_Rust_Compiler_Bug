
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.rlib
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so
INFO:rustc::metadata::loader: rlib reading metadata from: /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.rlib
INFO:rustc::metadata::loader: reading libstd-4e7c5e5c.rlib => 0ms
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-4e7c5e5c.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-4e7c5e5c.rlib
INFO:rustc::metadata::loader: reading liballoc-4e7c5e5c.rlib => 0ms
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libcore-4e7c5e5c.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-4e7c5e5c.rlib
INFO:rustc::metadata::loader: reading libcore-4e7c5e5c.rlib => 0ms
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-4e7c5e5c.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-4e7c5e5c.rlib
INFO:rustc::metadata::loader: reading liblibc-4e7c5e5c.rlib => 0ms
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libunicode-4e7c5e5c.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode-4e7c5e5c.rlib
INFO:rustc::metadata::loader: reading libunicode-4e7c5e5c.rlib => 0ms
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-4e7c5e5c.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-4e7c5e5c.rlib
INFO:rustc::metadata::loader: reading libcollections-4e7c5e5c.rlib => 0ms
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/librand-4e7c5e5c.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-4e7c5e5c.rlib
INFO:rustc::metadata::loader: reading librand-4e7c5e5c.rlib => 0ms
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/librustrt-4e7c5e5c.rlib
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/librustrt-4e7c5e5c.so
INFO:rustc::metadata::loader: rlib reading metadata from: /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustrt-4e7c5e5c.rlib
INFO:rustc::metadata::loader: reading librustrt-4e7c5e5c.rlib => 0ms
INFO:rustc::metadata::loader: rlib reading metadata from: /home/eduard/dev/toy/rust-ice-19768/target/deps/libacacia-44130cf3227ab7ae.rlib
INFO:rustc::metadata::loader: reading libacacia-44130cf3227ab7ae.rlib => 0ms
INFO:rustc::metadata::loader: lib candidate: /home/eduard/dev/toy/rust-ice-19768/target/deps/libnalgebra-d0bd0af049b73250.rlib
INFO:rustc::metadata::loader: lib candidate: /usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libnalgebra-4c93d261-0.1.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/libnalgebra-4c93d261-0.1.rlib
INFO:rustc::metadata::loader: reading libnalgebra-4c93d261-0.1.rlib => 0ms
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'assertion failed: end <= self.len()', /build/rust-git/src/rust/src/libcore/slice.rs:432

stack backtrace:
   1:     0x7f2914c98210 - rt::backtrace::imp::write::h49d8ac939d8781e7GOx
   2:     0x7f2914c9b310 - <unknown>
   3:     0x7f29148f7740 - unwind::begin_unwind_inner::h0288b255d3ea1869CJc
   4:     0x7f29148f7240 - unwind::begin_unwind_fmt::hf57382361e769392ZGc
   5:     0x7f29148f7200 - rust_begin_unwind
   6:     0x7f2914937e20 - panicking::panic_fmt::hbf754a34cfe87a0eRtl
   7:     0x7f2914935b20 - panicking::panic::hb1246f74b69e7954hrl
   8:     0x7f29132c1a30 - <unknown>
   9:     0x7f29132ba690 - <unknown>
  10:     0x7f29132b5d30 - metadata::loader::Context<'a>::load_library_crate::hddfb7c19c92a7307Rew
  11:     0x7f29132b0d60 - <unknown>
  12:     0x7f29132b3130 - <unknown>
  13:     0x7f29132b0d60 - <unknown>
  14:     0x7f29132abeb0 - metadata::creader::Env<'a>.visit..Visitor<'v>::visit_view_item::h070c7cbec1c3f414pzu
  15:     0x7f29132ab340 - metadata::creader::read_crates::h20d08d8c8148ca69jyu
  16:     0x7f29151062f0 - <unknown>
  17:     0x7f29150c7250 - driver::phase_3_run_analysis_passes::he4c9c4f305f26aeeCta
  18:     0x7f29150b65e0 - driver::compile_input::h0203f9fb584e1f48pba
  19:     0x7f2915151910 - <unknown>
  20:     0x7f2915151800 - <unknown>
  21:     0x7f2915163050 - <unknown>
  22:     0x7f2914c72160 - <unknown>
  23:     0x7f29148f53e0 - <unknown>
  24:     0x7f291494ab90 - <unknown>
  25:     0x7f291494ab80 - rust_try
  26:     0x7f29148f54c0 - unwind::try::h77d5b9b394dc26e4Tyc
  27:     0x7f29148f5280 - task::Task::run::hc596a395d876c158fKb
  28:     0x7f29148f4e70 - <unknown>
  29:     0x7f29148f68c0 - <unknown>
  30:     0x7f290fd1d250 - start_thread
  31:     0x7f29145cc589 - clone
  32:                0x0 - <unknown>
