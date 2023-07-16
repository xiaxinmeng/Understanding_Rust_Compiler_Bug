
RUST_BACKTRACE=1 cargo build --verbose
       Fresh log v0.3.6
       Fresh libc v0.2.9
       Fresh cfg-if v0.1.0
       Fresh semver v0.1.20
       Fresh getopts v0.2.14
       Fresh bitflags v0.4.0
       Fresh rustc_version v0.1.7
       Fresh nix v0.5.1-pre (https://github.com/mkollaro/nix#22ae34a7)
   Compiling librdb v0.1.0 (file:///mnt/data/src/rdb)
     Running `rustc src/lib.rs --crate-name librdb --crate-type lib -g --cfg feature=\"default\" --out-dir /mnt/data/src/rdb/target/debug --emit=dep-info,link -L dependency=/mnt/data/src/rdb/target/debug -L dependency=/mnt/data/src/rdb/target/debug/deps --extern libc=/mnt/data/src/rdb/target/debug/deps/liblibc-d2268fc21ed63f2c.rlib --extern log=/mnt/data/src/rdb/target/debug/deps/liblog-30a8a27ec161f1be.rlib --extern nix=/mnt/data/src/rdb/target/debug/deps/libnix-4281f0d4c5a4bab6.rlib --extern getopts=/mnt/data/src/rdb/target/debug/deps/libgetopts-852fe11dd444cf81.rlib -L native=/mnt/data/src/rdb/thirdparty/libdwarf/libdwarf -l static=dwarf -l elf -l z`
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Utf8Error { valid_up_to: 0 }', ../src/libcore/result.rs:746
stack backtrace:
   1:     0x7f96765bc330 - sys::backtrace::tracing::imp::write::h3675b4f0ca767761Xcv
   2:     0x7f96765c568b - panicking::default_handler::_$u7b$$u7b$closure$u7d$$u7d$::closure.44519
   3:     0x7f96765c51e3 - panicking::default_handler::h18faf4fbd296d909lSz
   4:     0x7f96765899ac - sys_common::unwind::begin_unwind_inner::hfb5d07d6e405c6bbg1t
   5:     0x7f967658a448 - sys_common::unwind::begin_unwind_fmt::h8b491a76ae84af35m0t
   6:     0x7f96765b9d91 - rust_begin_unwind
   7:     0x7f967660c7af - panicking::panic_fmt::h98b8cbb286f5298alcM
   8:     0x7f966e36e31b - result::unwrap_failed::h14307315525592478485
   9:     0x7f966e36e28e - Doc<'doc>::as_str_slice::ha91bf39a19faeabdHVa
  10:     0x7f967421945f - loader::Context<'a>::extract_one::h945c702fbde346e3Eem
  11:     0x7f9674210acf - loader::Context<'a>::find_library_crate::h30971509c344ad3bD3l
  12:     0x7f96741f5f16 - loader::Context<'a>::load_library_crate::h0ade06e110892b81WSl
  13:     0x7f96741f4d99 - creader::CrateReader<'a>::resolve_crate::he1542772b02d0896E3j
  14:     0x7f96741f658d - creader::CrateReader<'a>::resolve_crate_deps::_$u7b$$u7b$closure$u7d$$u7d$::closure.38792
  15:     0x7f96741f633c - collections::hash::map::HashMap<K, V, S>.FromIterator<(K, V)>::from_iter::h12379182329167058746
  16:     0x7f96741f24dd - creader::CrateReader<'a>::register_crate::h62645edecf14372cTZj
  17:     0x7f96741f745c - creader::CrateReader<'a>::read_extension_crate::hb5e653870a478718Zak
  18:     0x7f96741f96b5 - creader::CrateReader<'a>::read_exported_macros::h2322639de24bfa4cUek
  19:     0x7f967421ed0d - macro_import::MacroLoader<'a>.Visitor<'v>::visit_item::hf3405d18ebfdd3eakYm
  20:     0x7f967421d88a - macro_import::read_macro_defs::h345dbb1da4180a9foXm
  21:     0x7f9676aced89 - driver::phase_2_configure_and_expand::h0bbd128ff132a80elBa
  22:     0x7f9676ab4b21 - driver::compile_input::h7ae6a86e23de0774Hca
  23:     0x7f9676aa5b47 - run_compiler::hb0408bcf47642fe6mPc
  24:     0x7f9676aa32c1 - sys_common::unwind::try::try_fn::h7614151098073783683
  25:     0x7f96765b9d1b - __rust_try
  26:     0x7f96765b21fd - sys_common::unwind::inner_try::hadd81c754a64f07ciYt
  27:     0x7f9676aa3b10 - boxed::F.FnBox<A>::call_box::h18261575856772421581
  28:     0x7f96765c3c59 - sys::thread::Thread::new::thread_start::h9bc812305b5e01feFPy
  29:     0x7f966f869181 - start_thread
  30:     0x7f967622a47c - __clone
  31:                0x0 - <unknown>

Could not compile `librdb`.

Caused by:
  Process didn't exit successfully: `rustc src/lib.rs --crate-name librdb --crate-type lib -g --cfg feature="default" --out-dir /mnt/data/src/rdb/target/debug --emit=dep-info,link -L dependency=/mnt/data/src/rdb/target/debug -L dependency=/mnt/data/src/rdb/target/debug/deps --extern libc=/mnt/data/src/rdb/target/debug/deps/liblibc-d2268fc21ed63f2c.rlib --extern log=/mnt/data/src/rdb/target/debug/deps/liblog-30a8a27ec161f1be.rlib --extern nix=/mnt/data/src/rdb/target/debug/deps/libnix-4281f0d4c5a4bab6.rlib --extern getopts=/mnt/data/src/rdb/target/debug/deps/libgetopts-852fe11dd444cf81.rlib -L native=/mnt/data/src/rdb/thirdparty/libdwarf/libdwarf -l static=dwarf -l elf -l z` (exit code: 101)
