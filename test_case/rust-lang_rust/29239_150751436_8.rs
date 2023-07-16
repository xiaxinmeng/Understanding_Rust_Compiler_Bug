 bash
...
info: now are following matches for libsyntax-*.rlib libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'libsyntax-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes  -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
cp x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.syntax x86_64-unknown-linux-gnu/stage2/lib/stamp.syntax
cp -R x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-*.so x86_64-unknown-linux-gnu/stage2/lib
info: now are following matches for libsyntax-*.so libraries:
x86_64-unknown-linux-gnu/stage2/lib/libsyntax-bb943c5a.so
time: 1.007; rss: 40MB  parsing
time: 0.163; rss: 40MB  configuration 1
time: 0.000; rss: 40MB  recursion limit
time: 0.017; rss: 40MB  gated macro checking
time: 0.000; rss: 40MB  crate injection
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libstd-bb943c5a.rlib" => Duration { secs: 0, nanos: 302134 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcore-bb943c5a.rlib" => Duration { secs: 0, nanos: 237112 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcollections-bb943c5a.rlib" => Duration { secs: 0, nanos: 217066 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_unicode-bb943c5a.rlib" => Duration { secs: 0, nanos: 195555 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc-bb943c5a.rlib" => Duration { secs: 0, nanos: 215600 }
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 325111 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_system-bb943c5a.rlib" => Duration { secs: 0, nanos: 200933 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librand-bb943c5a.rlib" => Duration { secs: 0, nanos: 246400 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblibc-bb943c5a.rlib" => Duration { secs: 0, nanos: 237111 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 385244 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblog-bb943c5a.rlib" => Duration { secs: 0, nanos: 220489 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libsyntax-bb943c5a.rlib" => Duration { secs: 0, nanos: 268889 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libfmt_macros-bb943c5a.rlib" => Duration { secs: 0, nanos: 190667 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libserialize-bb943c5a.rlib" => Duration { secs: 0, nanos: 197022 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libterm-bb943c5a.rlib" => Duration { secs: 0, nanos: 202889 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_bitflags-bb943c5a.rlib" => Duration { secs: 0, nanos: 239067 }
time: 0.192; rss: 43MB  macro loading
time: 0.000; rss: 43MB  plugin loading
time: 0.000; rss: 43MB  plugin registration
time: 4.960; rss: 62MB  expansion
time: 0.225; rss: 62MB  complete gated feature checking 1
time: 0.924; rss: 62MB  configuration 2
time: 0.000; rss: 62MB  gated configuration checking
time: 0.496; rss: 62MB  maybe building test harness
time: 0.443; rss: 62MB  prelude injection
time: 0.057; rss: 62MB  checking that all macro invocations are gone
time: 0.000; rss: 62MB  checking for inline asm in case the target doesn't support it
time: 0.226; rss: 62MB  complete gated feature checking 2
time: 0.487; rss: 62MB  assigning node ids
time: 0.269; rss: 79MB  lowering ast -> hir
time: 1.101; rss: 83MB  indexing hir
time: 0.000; rss: 83MB  attribute checking
time: 0.280; rss: 83MB  early lint checks
time: 0.113; rss: 83MB  external crate/lib resolution
time: 0.100; rss: 83MB  language item collection
time: 4.076; rss: 113MB resolution
time: 0.130; rss: 113MB lifetime resolution
time: 0.000; rss: 113MB looking for entry point
time: 0.051; rss: 113MB looking for plugin registrar
time: 0.791; rss: 119MB region resolution
time: 0.049; rss: 119MB loop checking
time: 0.050; rss: 119MB static item recursion checking
time: 0.994; rss: 126MB type collecting
time: 0.117; rss: 126MB variance inference
time: 2.505; rss: 147MB coherence checking
time: 1.271; rss: 148MB wf checking (old)
time: 1.400; rss: 148MB item-types checking
time: 218.727; rss: 192MB   item-bodies checking
time: 0.001; rss: 192MB drop-impl checking
time: 6.643; rss: 192MB wf checking (new)
time: 23.116; rss: 193MB    const checking
time: 0.545; rss: 193MB privacy checking
time: 0.029; rss: 193MB stability index
time: 0.292; rss: 193MB intrinsic checking
time: 0.147; rss: 193MB effect checking
time: 1.564; rss: 193MB match checking
time: 10.327; rss: 250MB    MIR dump
time: 1.262; rss: 256MB liveness checking
time: 57.194; rss: 256MB    borrow checking
time: 37.176; rss: 256MB    rvalue checking
time: 0.231; rss: 256MB reachability checking
time: 0.536; rss: 256MB death checking
time: 0.936; rss: 256MB stability checking
time: 0.000; rss: 256MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x3511f88206c - sys::backtrace::tracing::imp::write::hcf25773dd1b9b1916tt
   2:      0x3511f8a6036 - panicking::log_panic::closure.39699
   3:      0x3511f8a5851 - panicking::log_panic::h3b637c6b446abf43Fux
   4:      0x3511f877828 - panicking::on_panic::hc25d5876b810d5819xx
   5:      0x3511f7dff6c - sys_common::unwind::begin_unwind_inner::h0e4a96b661d9f889xls
   6:      0x3511f7e1803 - sys_common::unwind::begin_unwind_fmt::h751259f99d216129Dks
   7:      0x3511c873489 - metadata::decoder::crate_metadata::lookup_item::hc6bb07d3788c1eebNlo
   8:      0x3511c911e48 - metadata::decoder::get_item_path::h4fea18eb8af39f4avjp
   9:      0x3511c49c9ab - metadata::csearch::get_item_path::h4c31ad371001de24Vgs
  10:      0x3511c850e3c - middle::ty::ctxt<'tcx>::with_path::h9335003841174771323
  11:      0x3511c3f8c4e - middle::ty::ctxt<'tcx>::item_path_str::h9f9a0e1f65fa6237rFh
  12:      0x3511c3f8a2b - middle::def_id::DefId.fmt..Debug::fmt::closure.91325
  13:      0x3511c3f894f - middle::ty::context::tls::with_opt::closure.91323
  14:      0x3511c3f88ff - middle::ty::context::tls::with::closure.91321
  15:      0x3511c3f88b1 - thread::scoped_tls::ScopedKey<T>::with::h3388241607244058986
  16:      0x3511c3f87fd - middle::ty::context::tls::with::h1576854055909152686
  17:      0x3511c3f86d7 - middle::ty::context::tls::with_opt::h3975270630994914165
  18:      0x3511c262a29 - middle::def_id::DefId.fmt..Debug::fmt::h9380a77c8bcca5e6Rfs
  19:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x3511f9135ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x3511c475e66 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::he8690a35e38e3037k8B
  22:      0x3511c47599d - fmt::_&'a T.Debug::fmt::h9871204069832299809
  23:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  24:      0x3511f9135ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  25:      0x3511c028231 - util::ppaux::ty..Region.fmt..Debug::fmt::h267fab552161f204saC
  26:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  27:      0x3511f9135ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  28:      0x3511c4a1ea7 - util::ppaux::ty..Region.fmt..Display::fmt::ha5175ea75697977akkC
  29:      0x3511ca93cbd - fmt::_&'a T.Display::fmt::h3482439631434626367
  30:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  31:      0x3511bfee264 - fmt::Write::write_fmt::h13212258486848778748
  32:      0x3511ca93c0d - string::T.ToString::to_string::h10930532372612609683
  33:      0x3511ca95c15 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::heab318c128e7d715tKB
  34:      0x3511caad67d - boxed::Box<T>.fmt..Display::fmt::h11790562609537591910
  35:      0x3511caad64d - fmt::_&'a T.Display::fmt::h15967930059134508810
  36:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  37:      0x3511f9135ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  38:      0x3511caa968a - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h2427238ac1d12534sQC
  39:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  40:      0x3511f9135ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  41:      0x3511c02bf79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hfd6eaa4ca73b7345WgD
  42:      0x3511c02be7d - fmt::_&'a T.Display::fmt::h10640878423442315299
  43:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  44:      0x3511f9135ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  45:      0x3511caa7ed2 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h2427238ac1d12534sQC
  46:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  47:      0x3511f9135ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  48:      0x3511c02bf79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hfd6eaa4ca73b7345WgD
  49:      0x3511c02be7d - fmt::_&'a T.Display::fmt::h10640878423442315299
  50:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  51:      0x3511f9135ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  52:      0x3511ca8f81b - util::ppaux::fn_sig::haf18e4d3ce2c6b8f7eB
  53:      0x3511ca9be55 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::he161373bdaca1056huC
  54:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  55:      0x3511f9135ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  56:      0x3511caa9173 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h2427238ac1d12534sQC
  57:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  58:      0x3511f9135ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  59:      0x3511c02bf79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hfd6eaa4ca73b7345WgD
  60:      0x3511f0be01d - fmt::_&'a T.Display::fmt::h16256538461273382626
  61:      0x3511f970d53 - fmt::write::h4c1f14d6ead6957fW5U
  62:      0x3511f8a9a74 - fmt::Write::write_fmt::h13060622517508656124
  63:      0x3511f8a992b - fmt::format::hdd6e6b41627bc4c0b6d
  64:      0x3511f0bd662 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  65:      0x3511f0be420 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  66:      0x3511ca508ed - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::ha0c4b6df3c6dbd76FOz
  67:      0x3511ca56753 - visit::walk_expr::h9391163145995048770
  68:      0x3511ca509c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::ha0c4b6df3c6dbd76FOz
  69:      0x3511ca564ab - visit::walk_expr::h9391163145995048770
  70:      0x3511ca509c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::ha0c4b6df3c6dbd76FOz
  71:      0x3511ca5d65b - visit::walk_block::h17995189004387409405
  72:      0x3511ca57127 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_block::h12d3424f8ec2324d81z
  73:      0x3511ca57c6b - visit::walk_fn::h3093254267512643283
  74:      0x3511ca50d36 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::h34afd124e5c7b149LQz
  75:      0x3511ca4f1be - visit::walk_item::h14534520424903162107
  76:      0x3511ca46d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  77:      0x3511ca43504 - lint::context::LintContext::with_lint_attrs::h2349662627144328648
  78:      0x3511ca41fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h7b8235a55ee9c3e9XKz
  79:      0x3511ca5d0ac - visit::walk_mod::h1390178209529731323
  80:      0x3511ca50fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h0df79d8292e92da4UZz
  81:      0x3511ca4f211 - visit::walk_item::h14534520424903162107
  82:      0x3511ca46d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  83:      0x3511ca43504 - lint::context::LintContext::with_lint_attrs::h2349662627144328648
  84:      0x3511ca41fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h7b8235a55ee9c3e9XKz
  85:      0x3511ca5d0ac - visit::walk_mod::h1390178209529731323
  86:      0x3511ca50fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h0df79d8292e92da4UZz
  87:      0x3511ca4f211 - visit::walk_item::h14534520424903162107
  88:      0x3511ca46d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  89:      0x3511ca43504 - lint::context::LintContext::with_lint_attrs::h2349662627144328648
  90:      0x3511ca41fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h7b8235a55ee9c3e9XKz
  91:      0x3511ca5d0ac - visit::walk_mod::h1390178209529731323
  92:      0x3511ca50fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h0df79d8292e92da4UZz
  93:      0x3511ca89cb5 - visit::walk_crate::h5196432013235849832
  94:      0x3511ca898e5 - lint::context::check_crate::closure.108246
  95:      0x3511ca88d74 - lint::context::LintContext::with_lint_attrs::h13084572717791510283
  96:      0x3511ca87137 - lint::context::check_crate::h7f36c39f87fdf155zPA
  97:      0x3511ff06bc6 - driver::phase_3_run_analysis_passes::closure.23233
  98:      0x3511ff06d8e - util::common::time::closure.23238
  99:      0x3511ff06d32 - time::duration::Duration::span::h9592294553425648252
  100:      0x3511ff065d7 - util::common::time::h10311329967984885887
 ... <frames omitted>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    64m14.333s
user    72m42.606s
sys 6m5.717s

