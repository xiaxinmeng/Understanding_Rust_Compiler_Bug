 bash
$ time RUST_LOG=rustc::metadata::loader make -j1 -- VERBOSE=1 NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes' RUST_BACKTRACE=1
cfg: version 1.5.0-dev (9a855668f 2015-10-23)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: disabling rustc optimization (CFG_DISABLE_OPTIMIZE)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes  -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
time: 1.016; rss: 44MB  parsing
time: 0.163; rss: 44MB  configuration 1
time: 0.000; rss: 44MB  recursion limit
time: 0.017; rss: 44MB  gated macro checking
time: 0.000; rss: 44MB  crate injection
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libstd-bb943c5a.rlib" => Duration { secs: 0, nanos: 296755 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcore-bb943c5a.rlib" => Duration { secs: 0, nanos: 234178 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcollections-bb943c5a.rlib" => Duration { secs: 0, nanos: 244934 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_unicode-bb943c5a.rlib" => Duration { secs: 0, nanos: 198977 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 349067 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc-bb943c5a.rlib" => Duration { secs: 0, nanos: 158400 }
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_system-bb943c5a.rlib" => Duration { secs: 0, nanos: 199956 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librand-bb943c5a.rlib" => Duration { secs: 0, nanos: 206311 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblibc-bb943c5a.rlib" => Duration { secs: 0, nanos: 185289 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 308978 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblog-bb943c5a.rlib" => Duration { secs: 0, nanos: 239556 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libsyntax-bb943c5a.rlib" => Duration { secs: 0, nanos: 240533 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libfmt_macros-bb943c5a.rlib" => Duration { secs: 0, nanos: 184311 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libserialize-bb943c5a.rlib" => Duration { secs: 0, nanos: 194089 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libterm-bb943c5a.rlib" => Duration { secs: 0, nanos: 231244 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_bitflags-bb943c5a.rlib" => Duration { secs: 0, nanos: 193111 }
time: 0.184; rss: 48MB  macro loading
time: 0.000; rss: 48MB  plugin loading
time: 0.000; rss: 48MB  plugin registration
time: 4.959; rss: 66MB  expansion
time: 0.226; rss: 66MB  complete gated feature checking 1
time: 0.918; rss: 66MB  configuration 2
time: 0.000; rss: 66MB  gated configuration checking
time: 0.490; rss: 66MB  maybe building test harness
time: 0.442; rss: 66MB  prelude injection
time: 0.057; rss: 66MB  checking that all macro invocations are gone
time: 0.000; rss: 66MB  checking for inline asm in case the target doesn't support it
time: 0.225; rss: 66MB  complete gated feature checking 2
time: 0.481; rss: 66MB  assigning node ids
time: 0.267; rss: 84MB  lowering ast -> hir
time: 1.095; rss: 88MB  indexing hir
time: 0.000; rss: 88MB  attribute checking
time: 0.278; rss: 88MB  early lint checks
time: 0.115; rss: 88MB  external crate/lib resolution
time: 0.099; rss: 88MB  language item collection
time: 4.007; rss: 117MB resolution
time: 0.129; rss: 116MB lifetime resolution
time: 0.000; rss: 116MB looking for entry point
time: 0.050; rss: 116MB looking for plugin registrar
time: 0.747; rss: 122MB region resolution
time: 0.049; rss: 122MB loop checking
time: 0.051; rss: 122MB static item recursion checking
time: 0.979; rss: 130MB type collecting
time: 0.116; rss: 130MB variance inference
time: 2.450; rss: 151MB coherence checking
time: 1.244; rss: 152MB wf checking (old)
time: 1.364; rss: 152MB item-types checking
time: 216.936; rss: 196MB   item-bodies checking
time: 0.001; rss: 196MB drop-impl checking
time: 6.632; rss: 196MB wf checking (new)
time: 23.047; rss: 197MB    const checking
time: 0.544; rss: 197MB privacy checking
time: 0.028; rss: 197MB stability index
time: 0.291; rss: 197MB intrinsic checking
time: 0.147; rss: 197MB effect checking
time: 1.557; rss: 197MB match checking
time: 10.344; rss: 255MB    MIR dump
time: 1.267; rss: 260MB liveness checking
time: 56.987; rss: 260MB    borrow checking
time: 36.848; rss: 260MB    rvalue checking
time: 0.228; rss: 260MB reachability checking
time: 0.535; rss: 260MB death checking
time: 0.931; rss: 260MB stability checking
time: 0.000; rss: 260MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x3841ff0006c - sys::backtrace::tracing::imp::write::hcf25773dd1b9b1916tt
   2:      0x3841ff24036 - panicking::log_panic::closure.39699
   3:      0x3841ff23851 - panicking::log_panic::h3b637c6b446abf43Fux
   4:      0x3841fef5828 - panicking::on_panic::hc25d5876b810d5819xx
   5:      0x3841fe5df6c - sys_common::unwind::begin_unwind_inner::h0e4a96b661d9f889xls
   6:      0x3841fe5f803 - sys_common::unwind::begin_unwind_fmt::h751259f99d216129Dks
   7:      0x3841cef1489 - metadata::decoder::crate_metadata::lookup_item::h8458d7c690c170ccNlo
   8:      0x3841cf8fe48 - metadata::decoder::get_item_path::h946966a169ac7b24vjp
   9:      0x3841cb1a9ab - metadata::csearch::get_item_path::h60727c6baa34e7e5Vgs
  10:      0x3841cecee3c - middle::ty::ctxt<'tcx>::with_path::h14554981355384456658
  11:      0x3841ca76c4e - middle::ty::ctxt<'tcx>::item_path_str::h81905e6d96970dferFh
  12:      0x3841ca76a2b - middle::def_id::DefId.fmt..Debug::fmt::closure.91325
  13:      0x3841ca7694f - middle::ty::context::tls::with_opt::closure.91323
  14:      0x3841ca768ff - middle::ty::context::tls::with::closure.91321
  15:      0x3841ca768b1 - thread::scoped_tls::ScopedKey<T>::with::h12505835326874039887
  16:      0x3841ca767fd - middle::ty::context::tls::with::h14253763777022529485
  17:      0x3841ca766d7 - middle::ty::context::tls::with_opt::h15396217575617304720
  18:      0x3841c8e0a29 - middle::def_id::DefId.fmt..Debug::fmt::h7309ea013736c781Rfs
  19:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x3841ff915ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x3841caf3e66 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::hd78bd6fb8305e912k8B
  22:      0x3841caf399d - fmt::_&'a T.Debug::fmt::h18004347074300142245
  23:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  24:      0x3841ff915ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  25:      0x3841c6a6231 - util::ppaux::ty..Region.fmt..Debug::fmt::h349b1e2e79452933saC
  26:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  27:      0x3841ff915ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  28:      0x3841cb1fea7 - util::ppaux::ty..Region.fmt..Display::fmt::h7e0a453c29708dc8kkC
  29:      0x3841d111cbd - fmt::_&'a T.Display::fmt::h7192573172774523792
  30:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  31:      0x3841c66c264 - fmt::Write::write_fmt::h627681005756376071
  32:      0x3841d111c0d - string::T.ToString::to_string::h16477161664338783167
  33:      0x3841d113c15 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::h2784e5e6ac79831dtKB
  34:      0x3841d12b67d - boxed::Box<T>.fmt..Display::fmt::h18082183109925588610
  35:      0x3841d12b64d - fmt::_&'a T.Display::fmt::h13316276846189391132
  36:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  37:      0x3841ff915ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  38:      0x3841d12768a - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h0ca456edae539601sQC
  39:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  40:      0x3841ff915ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  41:      0x3841c6a9f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::haef18a49c3cb1ee2WgD
  42:      0x3841c6a9e7d - fmt::_&'a T.Display::fmt::h18154365338086587585
  43:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  44:      0x3841ff915ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  45:      0x3841d125ed2 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h0ca456edae539601sQC
  46:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  47:      0x3841ff915ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  48:      0x3841c6a9f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::haef18a49c3cb1ee2WgD
  49:      0x3841c6a9e7d - fmt::_&'a T.Display::fmt::h18154365338086587585
  50:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  51:      0x3841ff915ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  52:      0x3841d10d81b - util::ppaux::fn_sig::h6eca8593cf4785247eB
  53:      0x3841d119e55 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::h7617d88323b7a192huC
  54:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  55:      0x3841ff915ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  56:      0x3841d127173 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h0ca456edae539601sQC
  57:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  58:      0x3841ff915ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  59:      0x3841c6a9f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::haef18a49c3cb1ee2WgD
  60:      0x3841f73c01d - fmt::_&'a T.Display::fmt::h4772995322088294892
  61:      0x3841ffeed53 - fmt::write::h4c1f14d6ead6957fW5U
  62:      0x3841ff27a74 - fmt::Write::write_fmt::h10968932166685698339
  63:      0x3841ff2792b - fmt::format::hdd6e6b41627bc4c0b6d
  64:      0x3841f73b662 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  65:      0x3841f73c420 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  66:      0x3841d0ce8ed - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::he4258933855a2f48FOz
  67:      0x3841d0d4753 - visit::walk_expr::h7202680961895252098
  68:      0x3841d0ce9c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::he4258933855a2f48FOz
  69:      0x3841d0d44ab - visit::walk_expr::h7202680961895252098
  70:      0x3841d0ce9c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::he4258933855a2f48FOz
  71:      0x3841d0db65b - visit::walk_block::h10455732252507424449
  72:      0x3841d0d5127 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_block::ha243b121dc521e1f81z
  73:      0x3841d0d5c6b - visit::walk_fn::h11620098901893989604
  74:      0x3841d0ced36 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::h11df07de935f81d2LQz
  75:      0x3841d0cd1be - visit::walk_item::h18406932623791079324
  76:      0x3841d0c4d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  77:      0x3841d0c1504 - lint::context::LintContext::with_lint_attrs::h11835066362755006506
  78:      0x3841d0bffd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h57d037404f5286f1XKz
  79:      0x3841d0db0ac - visit::walk_mod::h7318219855343182972
  80:      0x3841d0cefdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h13bf901ad767cdf3UZz
  81:      0x3841d0cd211 - visit::walk_item::h18406932623791079324
  82:      0x3841d0c4d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  83:      0x3841d0c1504 - lint::context::LintContext::with_lint_attrs::h11835066362755006506
  84:      0x3841d0bffd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h57d037404f5286f1XKz
  85:      0x3841d0db0ac - visit::walk_mod::h7318219855343182972
  86:      0x3841d0cefdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h13bf901ad767cdf3UZz
  87:      0x3841d0cd211 - visit::walk_item::h18406932623791079324
  88:      0x3841d0c4d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  89:      0x3841d0c1504 - lint::context::LintContext::with_lint_attrs::h11835066362755006506
  90:      0x3841d0bffd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h57d037404f5286f1XKz
  91:      0x3841d0db0ac - visit::walk_mod::h7318219855343182972
  92:      0x3841d0cefdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h13bf901ad767cdf3UZz
  93:      0x3841d107cb5 - visit::walk_crate::h13005888902521590913
  94:      0x3841d1078e5 - lint::context::check_crate::closure.108246
  95:      0x3841d106d74 - lint::context::LintContext::with_lint_attrs::h11456058053036407341
  96:      0x3841d105137 - lint::context::check_crate::h7b1bd09c4649b906zPA
  97:      0x38420583bc6 - driver::phase_3_run_analysis_passes::closure.23233
  98:      0x38420583d8e - util::common::time::closure.23238
  99:      0x38420583d32 - time::duration::Duration::span::h16207229507280863009
  100:      0x384205835d7 - util::common::time::h9240277138020009504
 ... <frames omitted>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    6m25.186s
user    6m23.050s
sys 0m1.360s

