 bash
$ git clean -dfx
Removing Makefile
Removing config.mk
Removing config.stamp
Removing dist/
Removing dl/
Removing doc/
Removing src/etc/snapshot.pyc
Removing tmp/
Removing x86_64-unknown-linux-gnu/
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --disable-optimize --disable-optimize-cxx --enable-optimize-llvm --disable-debug --disable-debuginfo --disable-debug-assertions --disable-debuginfo-tests --enable-llvm-assertions --disable-debug-jemalloc --disable-local-rust --release-channel=dev --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu
...
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes' RUST_BACKTRACE=1
cfg: version 1.5.0-dev (9a855668f 2015-10-23)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: disabling rustc optimization (CFG_DISABLE_OPTIMIZE)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: disabling C++ optimization (CFG_DISABLE_OPTIMIZE_CXX)
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
...
info: now are following matches for libsyntax-*.rlib libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'libsyntax-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes  -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
cp x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.syntax x86_64-unknown-linux-gnu/stage2/lib/stamp.syntax
cp -R x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-*.so x86_64-unknown-linux-gnu/stage2/lib
info: now are following matches for libsyntax-*.so libraries:
x86_64-unknown-linux-gnu/stage2/lib/libsyntax-bb943c5a.so
time: 1.021; rss: 44MB  parsing
time: 0.162; rss: 44MB  configuration 1
time: 0.000; rss: 44MB  recursion limit
time: 0.017; rss: 44MB  gated macro checking
time: 0.000; rss: 44MB  crate injection
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libstd-bb943c5a.rlib" => Duration { secs: 0, nanos: 299200 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcore-bb943c5a.rlib" => Duration { secs: 0, nanos: 193111 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcollections-bb943c5a.rlib" => Duration { secs: 0, nanos: 210711 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_unicode-bb943c5a.rlib" => Duration { secs: 0, nanos: 203866 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 312400 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc-bb943c5a.rlib" => Duration { secs: 0, nanos: 161822 }
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_system-bb943c5a.rlib" => Duration { secs: 0, nanos: 170134 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librand-bb943c5a.rlib" => Duration { secs: 0, nanos: 248356 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblibc-bb943c5a.rlib" => Duration { secs: 0, nanos: 231733 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 314844 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblog-bb943c5a.rlib" => Duration { secs: 0, nanos: 219511 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libsyntax-bb943c5a.rlib" => Duration { secs: 0, nanos: 252756 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libfmt_macros-bb943c5a.rlib" => Duration { secs: 0, nanos: 276711 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libserialize-bb943c5a.rlib" => Duration { secs: 0, nanos: 251289 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libterm-bb943c5a.rlib" => Duration { secs: 0, nanos: 188711 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_bitflags-bb943c5a.rlib" => Duration { secs: 0, nanos: 253245 }
time: 0.193; rss: 48MB  macro loading
time: 0.000; rss: 48MB  plugin loading
time: 0.000; rss: 48MB  plugin registration
time: 4.995; rss: 66MB  expansion
time: 0.225; rss: 66MB  complete gated feature checking 1
time: 0.917; rss: 66MB  configuration 2
time: 0.000; rss: 66MB  gated configuration checking
time: 0.493; rss: 66MB  maybe building test harness
time: 0.439; rss: 66MB  prelude injection
time: 0.057; rss: 66MB  checking that all macro invocations are gone
time: 0.000; rss: 66MB  checking for inline asm in case the target doesn't support it
time: 0.224; rss: 66MB  complete gated feature checking 2
time: 0.490; rss: 66MB  assigning node ids
time: 0.268; rss: 84MB  lowering ast -> hir
time: 1.115; rss: 88MB  indexing hir
time: 0.000; rss: 88MB  attribute checking
time: 0.279; rss: 88MB  early lint checks
time: 0.115; rss: 88MB  external crate/lib resolution
time: 0.104; rss: 88MB  language item collection
time: 3.984; rss: 117MB resolution
time: 0.134; rss: 117MB lifetime resolution
time: 0.000; rss: 117MB looking for entry point
time: 0.050; rss: 117MB looking for plugin registrar
time: 0.781; rss: 123MB region resolution
time: 0.050; rss: 123MB loop checking
time: 0.051; rss: 123MB static item recursion checking
time: 0.997; rss: 130MB type collecting
time: 0.120; rss: 130MB variance inference
time: 2.495; rss: 150MB coherence checking
time: 1.251; rss: 151MB wf checking (old)
time: 1.401; rss: 151MB item-types checking
time: 219.141; rss: 197MB   item-bodies checking
time: 0.001; rss: 197MB drop-impl checking
time: 6.652; rss: 197MB wf checking (new)
time: 23.165; rss: 198MB    const checking
time: 0.545; rss: 198MB privacy checking
time: 0.029; rss: 198MB stability index
time: 0.294; rss: 198MB intrinsic checking
time: 0.146; rss: 198MB effect checking
time: 1.564; rss: 198MB match checking
time: 10.548; rss: 255MB    MIR dump
time: 1.267; rss: 261MB liveness checking
time: 57.114; rss: 261MB    borrow checking
time: 37.875; rss: 261MB    rvalue checking
time: 0.236; rss: 261MB reachability checking
time: 0.538; rss: 261MB death checking
time: 0.906; rss: 261MB stability checking
time: 0.000; rss: 261MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x3ba0a37406c - sys::backtrace::tracing::imp::write::hcf25773dd1b9b1916tt
   2:      0x3ba0a398036 - panicking::log_panic::closure.39699
   3:      0x3ba0a397851 - panicking::log_panic::h3b637c6b446abf43Fux
   4:      0x3ba0a369828 - panicking::on_panic::hc25d5876b810d5819xx
   5:      0x3ba0a2d1f6c - sys_common::unwind::begin_unwind_inner::h0e4a96b661d9f889xls
   6:      0x3ba0a2d3803 - sys_common::unwind::begin_unwind_fmt::h751259f99d216129Dks
   7:      0x3ba07365489 - metadata::decoder::crate_metadata::lookup_item::h3edfbbe600c5f7d0Nlo
   8:      0x3ba07403e48 - metadata::decoder::get_item_path::he02bb2a5c50df805vjp
   9:      0x3ba06f8e9ab - metadata::csearch::get_item_path::h29b870ea4ea1c9e1Vgs
  10:      0x3ba07342e3c - middle::ty::ctxt<'tcx>::with_path::h10444011914326025041
  11:      0x3ba06eeac4e - middle::ty::ctxt<'tcx>::item_path_str::h91fa39c18cd05990rFh
  12:      0x3ba06eeaa2b - middle::def_id::DefId.fmt..Debug::fmt::closure.91325
  13:      0x3ba06eea94f - middle::ty::context::tls::with_opt::closure.91323
  14:      0x3ba06eea8ff - middle::ty::context::tls::with::closure.91321
  15:      0x3ba06eea8b1 - thread::scoped_tls::ScopedKey<T>::with::h17298581633707603397
  16:      0x3ba06eea7fd - middle::ty::context::tls::with::h14231861589600331442
  17:      0x3ba06eea6d7 - middle::ty::context::tls::with_opt::h3619900487362882043
  18:      0x3ba06d54a29 - middle::def_id::DefId.fmt..Debug::fmt::h7542fe2d89fd05fdRfs
  19:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x3ba0a4055ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x3ba06f67e66 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::he80173a972226fdfk8B
  22:      0x3ba06f6799d - fmt::_&'a T.Debug::fmt::h13596642170438732603
  23:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  24:      0x3ba0a4055ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  25:      0x3ba06b1a231 - util::ppaux::ty..Region.fmt..Debug::fmt::he9b0ed4c63294387saC
  26:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  27:      0x3ba0a4055ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  28:      0x3ba06f93ea7 - util::ppaux::ty..Region.fmt..Display::fmt::hf528900302a71c66kkC
  29:      0x3ba07585cbd - fmt::_&'a T.Display::fmt::h8276151546346955976
  30:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  31:      0x3ba06ae0264 - fmt::Write::write_fmt::h1612019339225692435
  32:      0x3ba07585c0d - string::T.ToString::to_string::h3028340564163228547
  33:      0x3ba07587c15 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::h6d7766da56aaf866tKB
  34:      0x3ba0759f67d - boxed::Box<T>.fmt..Display::fmt::h13442393552261748527
  35:      0x3ba0759f64d - fmt::_&'a T.Display::fmt::h8354122592389246808
  36:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  37:      0x3ba0a4055ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  38:      0x3ba0759b68a - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h28d057ee94230ffcsQC
  39:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  40:      0x3ba0a4055ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  41:      0x3ba06b1df79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hf86ff74f5557d683WgD
  42:      0x3ba06b1de7d - fmt::_&'a T.Display::fmt::h2441297674783162323
  43:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  44:      0x3ba0a4055ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  45:      0x3ba07599ed2 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h28d057ee94230ffcsQC
  46:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  47:      0x3ba0a4055ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  48:      0x3ba06b1df79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hf86ff74f5557d683WgD
  49:      0x3ba06b1de7d - fmt::_&'a T.Display::fmt::h2441297674783162323
  50:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  51:      0x3ba0a4055ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  52:      0x3ba0758181b - util::ppaux::fn_sig::h2477649c274fc6b77eB
  53:      0x3ba0758de55 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::h41714dc40a1bd22chuC
  54:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  55:      0x3ba0a4055ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  56:      0x3ba0759b173 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h28d057ee94230ffcsQC
  57:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  58:      0x3ba0a4055ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  59:      0x3ba06b1df79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hf86ff74f5557d683WgD
  60:      0x3ba09bb001d - fmt::_&'a T.Display::fmt::h12141864195412110725
  61:      0x3ba0a462d53 - fmt::write::h4c1f14d6ead6957fW5U
  62:      0x3ba0a39ba74 - fmt::Write::write_fmt::h13263643800637336612
  63:      0x3ba0a39b92b - fmt::format::hdd6e6b41627bc4c0b6d
  64:      0x3ba09baf662 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  65:      0x3ba09bb0420 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  66:      0x3ba075428ed - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h1f6851dbc69d3e13FOz
  67:      0x3ba07548753 - visit::walk_expr::h2128882793905338554
  68:      0x3ba075429c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h1f6851dbc69d3e13FOz
  69:      0x3ba075484ab - visit::walk_expr::h2128882793905338554
  70:      0x3ba075429c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h1f6851dbc69d3e13FOz
  71:      0x3ba0754f65b - visit::walk_block::h6806271851343953893
  72:      0x3ba07549127 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_block::h0186f8f6a29a3c2781z
  73:      0x3ba07549c6b - visit::walk_fn::h9457552485819512610
  74:      0x3ba07542d36 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::h3b61a5e3f733a59cLQz
  75:      0x3ba075411be - visit::walk_item::h16055444900783480887
  76:      0x3ba07538d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  77:      0x3ba07535504 - lint::context::LintContext::with_lint_attrs::h6875047648209851689
  78:      0x3ba07533fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h05d352ff2460bc3aXKz
  79:      0x3ba0754f0ac - visit::walk_mod::h11143103620898244718
  80:      0x3ba07542fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h2587b9f403a28709UZz
  81:      0x3ba07541211 - visit::walk_item::h16055444900783480887
  82:      0x3ba07538d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  83:      0x3ba07535504 - lint::context::LintContext::with_lint_attrs::h6875047648209851689
  84:      0x3ba07533fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h05d352ff2460bc3aXKz
  85:      0x3ba0754f0ac - visit::walk_mod::h11143103620898244718
  86:      0x3ba07542fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h2587b9f403a28709UZz
  87:      0x3ba07541211 - visit::walk_item::h16055444900783480887
  88:      0x3ba07538d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  89:      0x3ba07535504 - lint::context::LintContext::with_lint_attrs::h6875047648209851689
  90:      0x3ba07533fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h05d352ff2460bc3aXKz
  91:      0x3ba0754f0ac - visit::walk_mod::h11143103620898244718
  92:      0x3ba07542fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h2587b9f403a28709UZz
  93:      0x3ba0757bcb5 - visit::walk_crate::h13804701206288063033
  94:      0x3ba0757b8e5 - lint::context::check_crate::closure.108246
  95:      0x3ba0757ad74 - lint::context::LintContext::with_lint_attrs::h2442293853370300540
  96:      0x3ba07579137 - lint::context::check_crate::h53d8c239ee948b78zPA
  97:      0x3ba0a9f8bc6 - driver::phase_3_run_analysis_passes::closure.23233
  98:      0x3ba0a9f8d8e - util::common::time::closure.23238
  99:      0x3ba0a9f8d32 - time::duration::Duration::span::h3870212656307256546
  100:      0x3ba0a9f85d7 - util::common::time::h14543526651029665280
 ... <frames omitted>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    61m30.461s
user    68m29.630s
sys 4m4.560s
