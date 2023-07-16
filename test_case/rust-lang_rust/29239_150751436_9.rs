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
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes  -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
time: 1.004; rss: 39MB  parsing
time: 0.162; rss: 39MB  configuration 1
time: 0.000; rss: 39MB  recursion limit
time: 0.017; rss: 39MB  gated macro checking
time: 0.000; rss: 39MB  crate injection
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libstd-bb943c5a.rlib" => Duration { secs: 0, nanos: 304089 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcore-bb943c5a.rlib" => Duration { secs: 0, nanos: 198489 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcollections-bb943c5a.rlib" => Duration { secs: 0, nanos: 204356 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_unicode-bb943c5a.rlib" => Duration { secs: 0, nanos: 270845 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc-bb943c5a.rlib" => Duration { secs: 0, nanos: 231734 }
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 322667 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_system-bb943c5a.rlib" => Duration { secs: 0, nanos: 170133 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librand-bb943c5a.rlib" => Duration { secs: 0, nanos: 246889 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblibc-bb943c5a.rlib" => Duration { secs: 0, nanos: 184311 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 383778 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblog-bb943c5a.rlib" => Duration { secs: 0, nanos: 238577 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libsyntax-bb943c5a.rlib" => Duration { secs: 0, nanos: 367156 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libfmt_macros-bb943c5a.rlib" => Duration { secs: 0, nanos: 226845 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libserialize-bb943c5a.rlib" => Duration { secs: 0, nanos: 201911 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libterm-bb943c5a.rlib" => Duration { secs: 0, nanos: 209245 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_bitflags-bb943c5a.rlib" => Duration { secs: 0, nanos: 233200 }
time: 0.192; rss: 42MB  macro loading
time: 0.000; rss: 42MB  plugin loading
time: 0.000; rss: 42MB  plugin registration
time: 4.988; rss: 61MB  expansion
time: 0.227; rss: 61MB  complete gated feature checking 1
time: 0.924; rss: 61MB  configuration 2
time: 0.000; rss: 61MB  gated configuration checking
time: 0.497; rss: 61MB  maybe building test harness
time: 0.444; rss: 61MB  prelude injection
time: 0.057; rss: 61MB  checking that all macro invocations are gone
time: 0.000; rss: 61MB  checking for inline asm in case the target doesn't support it
time: 0.233; rss: 61MB  complete gated feature checking 2
time: 0.485; rss: 61MB  assigning node ids
time: 0.267; rss: 79MB  lowering ast -> hir
time: 1.095; rss: 83MB  indexing hir
time: 0.000; rss: 83MB  attribute checking
time: 0.278; rss: 83MB  early lint checks
time: 0.113; rss: 83MB  external crate/lib resolution
time: 0.100; rss: 83MB  language item collection
time: 4.062; rss: 113MB resolution
time: 0.129; rss: 112MB lifetime resolution
time: 0.000; rss: 112MB looking for entry point
time: 0.051; rss: 112MB looking for plugin registrar
time: 0.789; rss: 118MB region resolution
time: 0.049; rss: 118MB loop checking
time: 0.051; rss: 118MB static item recursion checking
time: 0.993; rss: 126MB type collecting
time: 0.117; rss: 126MB variance inference
time: 2.483; rss: 146MB coherence checking
time: 1.257; rss: 147MB wf checking (old)
time: 1.382; rss: 147MB item-types checking
time: 219.372; rss: 191MB   item-bodies checking
time: 0.001; rss: 191MB drop-impl checking
time: 6.724; rss: 191MB wf checking (new)
time: 23.367; rss: 192MB    const checking
time: 0.548; rss: 192MB privacy checking
time: 0.029; rss: 192MB stability index
time: 0.292; rss: 192MB intrinsic checking
time: 0.148; rss: 192MB effect checking
time: 1.568; rss: 192MB match checking
time: 10.371; rss: 250MB    MIR dump
time: 1.268; rss: 256MB liveness checking
time: 57.333; rss: 256MB    borrow checking
time: 37.048; rss: 256MB    rvalue checking
time: 0.229; rss: 256MB reachability checking
time: 0.533; rss: 256MB death checking
time: 0.934; rss: 256MB stability checking
time: 0.000; rss: 256MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x378491aa06c - sys::backtrace::tracing::imp::write::hcf25773dd1b9b1916tt
   2:      0x378491ce036 - panicking::log_panic::closure.39699
   3:      0x378491cd851 - panicking::log_panic::h3b637c6b446abf43Fux
   4:      0x3784919f828 - panicking::on_panic::hc25d5876b810d5819xx
   5:      0x37849107f6c - sys_common::unwind::begin_unwind_inner::h0e4a96b661d9f889xls
   6:      0x37849109803 - sys_common::unwind::begin_unwind_fmt::h751259f99d216129Dks
   7:      0x3784619b489 - metadata::decoder::crate_metadata::lookup_item::hc6bb07d3788c1eebNlo
   8:      0x37846239e48 - metadata::decoder::get_item_path::h4fea18eb8af39f4avjp
   9:      0x37845dc49ab - metadata::csearch::get_item_path::h4c31ad371001de24Vgs
  10:      0x37846178e3c - middle::ty::ctxt<'tcx>::with_path::h9335003841174771323
  11:      0x37845d20c4e - middle::ty::ctxt<'tcx>::item_path_str::h9f9a0e1f65fa6237rFh
  12:      0x37845d20a2b - middle::def_id::DefId.fmt..Debug::fmt::closure.91325
  13:      0x37845d2094f - middle::ty::context::tls::with_opt::closure.91323
  14:      0x37845d208ff - middle::ty::context::tls::with::closure.91321
  15:      0x37845d208b1 - thread::scoped_tls::ScopedKey<T>::with::h3388241607244058986
  16:      0x37845d207fd - middle::ty::context::tls::with::h1576854055909152686
  17:      0x37845d206d7 - middle::ty::context::tls::with_opt::h3975270630994914165
  18:      0x37845b8aa29 - middle::def_id::DefId.fmt..Debug::fmt::h9380a77c8bcca5e6Rfs
  19:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x3784923b5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x37845d9de66 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::he8690a35e38e3037k8B
  22:      0x37845d9d99d - fmt::_&'a T.Debug::fmt::h9871204069832299809
  23:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  24:      0x3784923b5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  25:      0x37845950231 - util::ppaux::ty..Region.fmt..Debug::fmt::h267fab552161f204saC
  26:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  27:      0x3784923b5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  28:      0x37845dc9ea7 - util::ppaux::ty..Region.fmt..Display::fmt::ha5175ea75697977akkC
  29:      0x378463bbcbd - fmt::_&'a T.Display::fmt::h3482439631434626367
  30:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  31:      0x37845916264 - fmt::Write::write_fmt::h13212258486848778748
  32:      0x378463bbc0d - string::T.ToString::to_string::h10930532372612609683
  33:      0x378463bdc15 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::heab318c128e7d715tKB
  34:      0x378463d567d - boxed::Box<T>.fmt..Display::fmt::h11790562609537591910
  35:      0x378463d564d - fmt::_&'a T.Display::fmt::h15967930059134508810
  36:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  37:      0x3784923b5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  38:      0x378463d168a - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h2427238ac1d12534sQC
  39:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  40:      0x3784923b5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  41:      0x37845953f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hfd6eaa4ca73b7345WgD
  42:      0x37845953e7d - fmt::_&'a T.Display::fmt::h10640878423442315299
  43:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  44:      0x3784923b5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  45:      0x378463cfed2 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h2427238ac1d12534sQC
  46:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  47:      0x3784923b5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  48:      0x37845953f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hfd6eaa4ca73b7345WgD
  49:      0x37845953e7d - fmt::_&'a T.Display::fmt::h10640878423442315299
  50:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  51:      0x3784923b5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  52:      0x378463b781b - util::ppaux::fn_sig::haf18e4d3ce2c6b8f7eB
  53:      0x378463c3e55 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::he161373bdaca1056huC
  54:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  55:      0x3784923b5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  56:      0x378463d1173 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h2427238ac1d12534sQC
  57:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  58:      0x3784923b5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  59:      0x37845953f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hfd6eaa4ca73b7345WgD
  60:      0x378489e601d - fmt::_&'a T.Display::fmt::h16256538461273382626
  61:      0x37849298d53 - fmt::write::h4c1f14d6ead6957fW5U
  62:      0x378491d1a74 - fmt::Write::write_fmt::h13060622517508656124
  63:      0x378491d192b - fmt::format::hdd6e6b41627bc4c0b6d
  64:      0x378489e5662 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  65:      0x378489e6420 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  66:      0x378463788ed - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::ha0c4b6df3c6dbd76FOz
  67:      0x3784637e753 - visit::walk_expr::h9391163145995048770
  68:      0x378463789c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::ha0c4b6df3c6dbd76FOz
  69:      0x3784637e4ab - visit::walk_expr::h9391163145995048770
  70:      0x378463789c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::ha0c4b6df3c6dbd76FOz
  71:      0x3784638565b - visit::walk_block::h17995189004387409405
  72:      0x3784637f127 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_block::h12d3424f8ec2324d81z
  73:      0x3784637fc6b - visit::walk_fn::h3093254267512643283
  74:      0x37846378d36 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::h34afd124e5c7b149LQz
  75:      0x378463771be - visit::walk_item::h14534520424903162107
  76:      0x3784636ed05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  77:      0x3784636b504 - lint::context::LintContext::with_lint_attrs::h2349662627144328648
  78:      0x37846369fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h7b8235a55ee9c3e9XKz
  79:      0x378463850ac - visit::walk_mod::h1390178209529731323
  80:      0x37846378fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h0df79d8292e92da4UZz
  81:      0x37846377211 - visit::walk_item::h14534520424903162107
  82:      0x3784636ed05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  83:      0x3784636b504 - lint::context::LintContext::with_lint_attrs::h2349662627144328648
  84:      0x37846369fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h7b8235a55ee9c3e9XKz
  85:      0x378463850ac - visit::walk_mod::h1390178209529731323
  86:      0x37846378fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h0df79d8292e92da4UZz
  87:      0x37846377211 - visit::walk_item::h14534520424903162107
  88:      0x3784636ed05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  89:      0x3784636b504 - lint::context::LintContext::with_lint_attrs::h2349662627144328648
  90:      0x37846369fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h7b8235a55ee9c3e9XKz
  91:      0x378463850ac - visit::walk_mod::h1390178209529731323
  92:      0x37846378fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h0df79d8292e92da4UZz
  93:      0x378463b1cb5 - visit::walk_crate::h5196432013235849832
  94:      0x378463b18e5 - lint::context::check_crate::closure.108246
  95:      0x378463b0d74 - lint::context::LintContext::with_lint_attrs::h13084572717791510283
  96:      0x378463af137 - lint::context::check_crate::h7f36c39f87fdf155zPA
  97:      0x3784982ebc6 - driver::phase_3_run_analysis_passes::closure.23233
  98:      0x3784982ed8e - util::common::time::closure.23238
  99:      0x3784982ed32 - time::duration::Duration::span::h9592294553425648252
  100:      0x3784982e5d7 - util::common::time::h10311329967984885887
 ... <frames omitted>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    6m28.768s
user    6m25.177s
sys 0m2.627s

