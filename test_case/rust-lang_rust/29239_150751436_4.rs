 bash
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
touch /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/llvm-auto-clean-stamp.start_time
...
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes  -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
cp x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.syntax x86_64-unknown-linux-gnu/stage2/lib/stamp.syntax
cp -R x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-*.so x86_64-unknown-linux-gnu/stage2/lib
info: now are following matches for libsyntax-*.so libraries:
x86_64-unknown-linux-gnu/stage2/lib/libsyntax-bb943c5a.so
time: 0.998; rss: 40MB  parsing
time: 0.161; rss: 40MB  configuration 1
time: 0.000; rss: 40MB  recursion limit
time: 0.017; rss: 40MB  gated macro checking
time: 0.000; rss: 40MB  crate injection
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libstd-bb943c5a.rlib" => Duration { secs: 0, nanos: 311911 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcore-bb943c5a.rlib" => Duration { secs: 0, nanos: 189200 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcollections-bb943c5a.rlib" => Duration { secs: 0, nanos: 227823 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_unicode-bb943c5a.rlib" => Duration { secs: 0, nanos: 231733 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc-bb943c5a.rlib" => Duration { secs: 0, nanos: 201422 }
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_system-bb943c5a.rlib" => Duration { secs: 0, nanos: 179912 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 334400 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librand-bb943c5a.rlib" => Duration { secs: 0, nanos: 208267 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblibc-bb943c5a.rlib" => Duration { secs: 0, nanos: 188711 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 347111 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblog-bb943c5a.rlib" => Duration { secs: 0, nanos: 283556 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libsyntax-bb943c5a.rlib" => Duration { secs: 0, nanos: 311422 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libfmt_macros-bb943c5a.rlib" => Duration { secs: 0, nanos: 203378 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libserialize-bb943c5a.rlib" => Duration { secs: 0, nanos: 209734 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libterm-bb943c5a.rlib" => Duration { secs: 0, nanos: 225378 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_bitflags-bb943c5a.rlib" => Duration { secs: 0, nanos: 219022 }
time: 0.193; rss: 43MB  macro loading
time: 0.000; rss: 43MB  plugin loading
time: 0.000; rss: 43MB  plugin registration
time: 4.943; rss: 62MB  expansion
time: 0.223; rss: 62MB  complete gated feature checking 1
time: 0.919; rss: 62MB  configuration 2
time: 0.000; rss: 62MB  gated configuration checking
time: 0.496; rss: 62MB  maybe building test harness
time: 0.441; rss: 62MB  prelude injection
time: 0.058; rss: 62MB  checking that all macro invocations are gone
time: 0.000; rss: 62MB  checking for inline asm in case the target doesn't support it
time: 0.223; rss: 62MB  complete gated feature checking 2
time: 0.487; rss: 62MB  assigning node ids
time: 0.268; rss: 79MB  lowering ast -> hir
time: 1.087; rss: 83MB  indexing hir
time: 0.000; rss: 83MB  attribute checking
time: 0.278; rss: 83MB  early lint checks
time: 0.118; rss: 83MB  external crate/lib resolution
time: 0.100; rss: 83MB  language item collection
time: 4.064; rss: 113MB resolution
time: 0.130; rss: 112MB lifetime resolution
time: 0.000; rss: 112MB looking for entry point
time: 0.049; rss: 112MB looking for plugin registrar
time: 0.785; rss: 118MB region resolution
time: 0.050; rss: 118MB loop checking
time: 0.050; rss: 118MB static item recursion checking
time: 0.981; rss: 126MB type collecting
time: 0.117; rss: 126MB variance inference
time: 2.476; rss: 146MB coherence checking
time: 1.252; rss: 147MB wf checking (old)
time: 1.381; rss: 147MB item-types checking
time: 217.966; rss: 192MB   item-bodies checking
time: 0.001; rss: 192MB drop-impl checking
time: 6.658; rss: 192MB wf checking (new)
time: 23.107; rss: 193MB    const checking
time: 0.547; rss: 193MB privacy checking
time: 0.029; rss: 193MB stability index
time: 0.292; rss: 193MB intrinsic checking
time: 0.148; rss: 193MB effect checking
time: 1.562; rss: 193MB match checking
time: 10.377; rss: 251MB    MIR dump
time: 1.276; rss: 256MB liveness checking
time: 57.318; rss: 256MB    borrow checking
time: 36.958; rss: 256MB    rvalue checking
time: 0.237; rss: 256MB reachability checking
time: 0.538; rss: 256MB death checking
time: 0.946; rss: 256MB stability checking
time: 0.000; rss: 256MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x3bf188f106c - sys::backtrace::tracing::imp::write::hcf25773dd1b9b1916tt
   2:      0x3bf18915036 - panicking::log_panic::closure.39699
   3:      0x3bf18914851 - panicking::log_panic::h3b637c6b446abf43Fux
   4:      0x3bf188e6828 - panicking::on_panic::hc25d5876b810d5819xx
   5:      0x3bf1884ef6c - sys_common::unwind::begin_unwind_inner::h0e4a96b661d9f889xls
   6:      0x3bf18850803 - sys_common::unwind::begin_unwind_fmt::h751259f99d216129Dks
   7:      0x3bf158e2489 - metadata::decoder::crate_metadata::lookup_item::hf08f6b22c0358161Nlo
   8:      0x3bf15980e48 - metadata::decoder::get_item_path::h11de4f3f9b81af0evjp
   9:      0x3bf1550b9ab - metadata::csearch::get_item_path::h53ee6492ffc8aad6Vgs
  10:      0x3bf158bfe3c - middle::ty::ctxt<'tcx>::with_path::h17430554772235534805
  11:      0x3bf15467c4e - middle::ty::ctxt<'tcx>::item_path_str::h7416ccdd0532af28rFh
  12:      0x3bf15467a2b - middle::def_id::DefId.fmt..Debug::fmt::closure.91325
  13:      0x3bf1546794f - middle::ty::context::tls::with_opt::closure.91323
  14:      0x3bf154678ff - middle::ty::context::tls::with::closure.91321
  15:      0x3bf154678b1 - thread::scoped_tls::ScopedKey<T>::with::h12102388316988659667
  16:      0x3bf154677fd - middle::ty::context::tls::with::h13208612757203259507
  17:      0x3bf154676d7 - middle::ty::context::tls::with_opt::h1041192060304589138
  18:      0x3bf152d1a29 - middle::def_id::DefId.fmt..Debug::fmt::h72607eac0e4d34b7Rfs
  19:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x3bf189825ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x3bf154e4e66 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::haf6e822d50b46e0ek8B
  22:      0x3bf154e499d - fmt::_&'a T.Debug::fmt::h12363322123584397570
  23:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  24:      0x3bf189825ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  25:      0x3bf15097231 - util::ppaux::ty..Region.fmt..Debug::fmt::hab3f9c9a14965e8fsaC
  26:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  27:      0x3bf189825ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  28:      0x3bf15510ea7 - util::ppaux::ty..Region.fmt..Display::fmt::hbb3c3e1c9d171669kkC
  29:      0x3bf15b02cbd - fmt::_&'a T.Display::fmt::h7626442323429155647
  30:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  31:      0x3bf1505d264 - fmt::Write::write_fmt::h15306967593098883599
  32:      0x3bf15b02c0d - string::T.ToString::to_string::h313867877109403957
  33:      0x3bf15b04c15 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::hf9568ea6579c6ef3tKB
  34:      0x3bf15b1c67d - boxed::Box<T>.fmt..Display::fmt::h4372649342355664983
  35:      0x3bf15b1c64d - fmt::_&'a T.Display::fmt::h13713692595366427245
  36:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  37:      0x3bf189825ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  38:      0x3bf15b1868a - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::ha01379c5ec9188d7sQC
  39:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  40:      0x3bf189825ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  41:      0x3bf1509af79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::h8d4b201b83cb4c6fWgD
  42:      0x3bf1509ae7d - fmt::_&'a T.Display::fmt::h13363484588731888732
  43:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  44:      0x3bf189825ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  45:      0x3bf15b16ed2 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::ha01379c5ec9188d7sQC
  46:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  47:      0x3bf189825ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  48:      0x3bf1509af79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::h8d4b201b83cb4c6fWgD
  49:      0x3bf1509ae7d - fmt::_&'a T.Display::fmt::h13363484588731888732
  50:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  51:      0x3bf189825ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  52:      0x3bf15afe81b - util::ppaux::fn_sig::h210cf2ffde819e7b7eB
  53:      0x3bf15b0ae55 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::hc6ace3b7a099c0a6huC
  54:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  55:      0x3bf189825ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  56:      0x3bf15b18173 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::ha01379c5ec9188d7sQC
  57:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  58:      0x3bf189825ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  59:      0x3bf1509af79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::h8d4b201b83cb4c6fWgD
  60:      0x3bf1812d01d - fmt::_&'a T.Display::fmt::h9595480147772079528
  61:      0x3bf189dfd53 - fmt::write::h4c1f14d6ead6957fW5U
  62:      0x3bf18918a74 - fmt::Write::write_fmt::h1657682277787206595
  63:      0x3bf1891892b - fmt::format::hdd6e6b41627bc4c0b6d
  64:      0x3bf1812c662 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  65:      0x3bf1812d420 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  66:      0x3bf15abf8ed - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hc5f883419b317aa7FOz
  67:      0x3bf15ac5753 - visit::walk_expr::h6001670156799437367
  68:      0x3bf15abf9c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hc5f883419b317aa7FOz
  69:      0x3bf15ac54ab - visit::walk_expr::h6001670156799437367
  70:      0x3bf15abf9c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hc5f883419b317aa7FOz
  71:      0x3bf15acc65b - visit::walk_block::h8214831767702797224
  72:      0x3bf15ac6127 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_block::ha9f60a8a503d48ba81z
  73:      0x3bf15ac6c6b - visit::walk_fn::h16926648705021976713
  74:      0x3bf15abfd36 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::h3e438dea4a5780cfLQz
  75:      0x3bf15abe1be - visit::walk_item::h3005546349946431523
  76:      0x3bf15ab5d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  77:      0x3bf15ab2504 - lint::context::LintContext::with_lint_attrs::h14856260811062557106
  78:      0x3bf15ab0fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::hdfbc9a8ea4f353c4XKz
  79:      0x3bf15acc0ac - visit::walk_mod::h12415316352534731568
  80:      0x3bf15abffdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::ha3c111d5d9123e2aUZz
  81:      0x3bf15abe211 - visit::walk_item::h3005546349946431523
  82:      0x3bf15ab5d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  83:      0x3bf15ab2504 - lint::context::LintContext::with_lint_attrs::h14856260811062557106
  84:      0x3bf15ab0fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::hdfbc9a8ea4f353c4XKz
  85:      0x3bf15acc0ac - visit::walk_mod::h12415316352534731568
  86:      0x3bf15abffdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::ha3c111d5d9123e2aUZz
  87:      0x3bf15abe211 - visit::walk_item::h3005546349946431523
  88:      0x3bf15ab5d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  89:      0x3bf15ab2504 - lint::context::LintContext::with_lint_attrs::h14856260811062557106
  90:      0x3bf15ab0fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::hdfbc9a8ea4f353c4XKz
  91:      0x3bf15acc0ac - visit::walk_mod::h12415316352534731568
  92:      0x3bf15abffdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::ha3c111d5d9123e2aUZz
  93:      0x3bf15af8cb5 - visit::walk_crate::h516633833914151244
  94:      0x3bf15af88e5 - lint::context::check_crate::closure.108246
  95:      0x3bf15af7d74 - lint::context::LintContext::with_lint_attrs::h15789443379023991964
  96:      0x3bf15af6137 - lint::context::check_crate::ha47c1e2704fada7czPA
  97:      0x3bf18f75bc6 - driver::phase_3_run_analysis_passes::closure.23233
  98:      0x3bf18f75d8e - util::common::time::closure.23238
  99:      0x3bf18f75d32 - time::duration::Duration::span::h8954618039401094357
  100:      0x3bf18f755d7 - util::common::time::h1428647808065421146
 ... <frames omitted>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    62m59.160s
user    72m24.176s
sys 6m5.163s

