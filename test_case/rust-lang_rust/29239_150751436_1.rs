 bash
...
info: now are following matches for librbml-*.rlib libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librbml-bb943c5a.rlib
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes  -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
time: 1.010; rss: 44MB  parsing
time: 0.162; rss: 44MB  configuration 1
time: 0.000; rss: 44MB  recursion limit
time: 0.017; rss: 44MB  gated macro checking
time: 0.000; rss: 44MB  crate injection
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libstd-bb943c5a.rlib" => Duration { secs: 0, nanos: 345156 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcore-bb943c5a.rlib" => Duration { secs: 0, nanos: 190178 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcollections-bb943c5a.rlib" => Duration { secs: 0, nanos: 221467 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_unicode-bb943c5a.rlib" => Duration { secs: 0, nanos: 185778 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc-bb943c5a.rlib" => Duration { secs: 0, nanos: 186755 }
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_system-bb943c5a.rlib" => Duration { secs: 0, nanos: 156934 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 277200 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librand-bb943c5a.rlib" => Duration { secs: 0, nanos: 204355 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblibc-bb943c5a.rlib" => Duration { secs: 0, nanos: 190666 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 375956 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblog-bb943c5a.rlib" => Duration { secs: 0, nanos: 250800 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libsyntax-bb943c5a.rlib" => Duration { secs: 0, nanos: 313378 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libfmt_macros-bb943c5a.rlib" => Duration { secs: 0, nanos: 209733 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libserialize-bb943c5a.rlib" => Duration { secs: 0, nanos: 195556 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libterm-bb943c5a.rlib" => Duration { secs: 0, nanos: 194089 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_bitflags-bb943c5a.rlib" => Duration { secs: 0, nanos: 191156 }
time: 0.183; rss: 47MB  macro loading
time: 0.000; rss: 47MB  plugin loading
time: 0.000; rss: 47MB  plugin registration
time: 4.970; rss: 66MB  expansion
time: 0.221; rss: 66MB  complete gated feature checking 1
time: 0.905; rss: 66MB  configuration 2
time: 0.000; rss: 66MB  gated configuration checking
time: 0.488; rss: 66MB  maybe building test harness
time: 0.437; rss: 66MB  prelude injection
time: 0.056; rss: 66MB  checking that all macro invocations are gone
time: 0.000; rss: 66MB  checking for inline asm in case the target doesn't support it
time: 0.222; rss: 66MB  complete gated feature checking 2
time: 0.481; rss: 66MB  assigning node ids
time: 0.268; rss: 83MB  lowering ast -> hir
time: 1.047; rss: 87MB  indexing hir
time: 0.000; rss: 87MB  attribute checking
time: 0.276; rss: 87MB  early lint checks
time: 0.113; rss: 87MB  external crate/lib resolution
time: 0.099; rss: 87MB  language item collection
time: 3.953; rss: 117MB resolution
time: 0.132; rss: 116MB lifetime resolution
time: 0.000; rss: 116MB looking for entry point
time: 0.049; rss: 116MB looking for plugin registrar
time: 0.779; rss: 122MB region resolution
time: 0.049; rss: 122MB loop checking
time: 0.050; rss: 122MB static item recursion checking
time: 0.979; rss: 130MB type collecting
time: 0.116; rss: 130MB variance inference
time: 2.464; rss: 150MB coherence checking
time: 1.241; rss: 151MB wf checking (old)
time: 1.381; rss: 151MB item-types checking
time: 218.363; rss: 196MB   item-bodies checking
time: 0.001; rss: 196MB drop-impl checking
time: 6.667; rss: 196MB wf checking (new)
time: 23.236; rss: 197MB    const checking
time: 0.539; rss: 197MB privacy checking
time: 0.029; rss: 197MB stability index
time: 0.290; rss: 197MB intrinsic checking
time: 0.145; rss: 197MB effect checking
time: 1.559; rss: 197MB match checking
time: 10.396; rss: 254MB    MIR dump
time: 1.246; rss: 260MB liveness checking
time: 57.437; rss: 260MB    borrow checking
time: 37.322; rss: 260MB    rvalue checking
time: 0.230; rss: 260MB reachability checking
time: 0.540; rss: 260MB death checking
time: 0.912; rss: 260MB stability checking
time: 0.000; rss: 260MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x3574773a06c - sys::backtrace::tracing::imp::write::hcf25773dd1b9b1916tt
   2:      0x3574775e036 - panicking::log_panic::closure.39699
   3:      0x3574775d851 - panicking::log_panic::h3b637c6b446abf43Fux
   4:      0x3574772f828 - panicking::on_panic::hc25d5876b810d5819xx
   5:      0x35747697f6c - sys_common::unwind::begin_unwind_inner::h0e4a96b661d9f889xls
   6:      0x35747699803 - sys_common::unwind::begin_unwind_fmt::h751259f99d216129Dks
   7:      0x3574472b459 - metadata::decoder::crate_metadata::lookup_item::ha36c61095099f003Nlo
   8:      0x357447c9e18 - metadata::decoder::get_item_path::ha878c8feaa99505avjp
   9:      0x3574435497b - metadata::csearch::get_item_path::h01d9b3c73b4e5597Vgs
  10:      0x35744708e0c - middle::ty::ctxt<'tcx>::with_path::h10352311365321106756
  11:      0x357442b0c1e - middle::ty::ctxt<'tcx>::item_path_str::h1d8a2d27929dae3drFh
  12:      0x357442b09fb - middle::def_id::DefId.fmt..Debug::fmt::closure.91325
  13:      0x357442b091f - middle::ty::context::tls::with_opt::closure.91323
  14:      0x357442b08cf - middle::ty::context::tls::with::closure.91321
  15:      0x357442b0881 - thread::scoped_tls::ScopedKey<T>::with::h7961186222578779811
  16:      0x357442b07cd - middle::ty::context::tls::with::h12348653215140575377
  17:      0x357442b06a7 - middle::ty::context::tls::with_opt::h18313064416985377286
  18:      0x3574411aa09 - middle::def_id::DefId.fmt..Debug::fmt::h7c22517fbf3892d2Rfs
  19:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x357477cb5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x3574432de36 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::hc40c59961a7defa0k8B
  22:      0x3574432d96d - fmt::_&'a T.Debug::fmt::h10015781735513924289
  23:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  24:      0x357477cb5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  25:      0x35743ee0231 - util::ppaux::ty..Region.fmt..Debug::fmt::h589c6297d96abc17saC
  26:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  27:      0x357477cb5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  28:      0x35744359e77 - util::ppaux::ty..Region.fmt..Display::fmt::hf3f42b2d76dcf2eckkC
  29:      0x3574494bc8d - fmt::_&'a T.Display::fmt::h2719069493102722619
  30:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  31:      0x35743ea6264 - fmt::Write::write_fmt::h9803176015046476954
  32:      0x3574494bbdd - string::T.ToString::to_string::h9856020121485473567
  33:      0x3574494dbe5 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::h99cce19427d20d6etKB
  34:      0x3574496564d - boxed::Box<T>.fmt..Display::fmt::h6562987315473910287
  35:      0x3574496561d - fmt::_&'a T.Display::fmt::h7903674299366236526
  36:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  37:      0x357477cb5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  38:      0x3574496165a - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h9e93f298b8db0112sQC
  39:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  40:      0x357477cb5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  41:      0x35743ee3f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::he3a07def9c733b33WgD
  42:      0x35743ee3e7d - fmt::_&'a T.Display::fmt::h5777667350377432080
  43:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  44:      0x357477cb5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  45:      0x3574495fea2 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h9e93f298b8db0112sQC
  46:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  47:      0x357477cb5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  48:      0x35743ee3f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::he3a07def9c733b33WgD
  49:      0x35743ee3e7d - fmt::_&'a T.Display::fmt::h5777667350377432080
  50:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  51:      0x357477cb5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  52:      0x357449477eb - util::ppaux::fn_sig::hef72edc1e961ebb77eB
  53:      0x35744953e25 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::hc07bcc810f4a5b0dhuC
  54:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  55:      0x357477cb5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  56:      0x35744961143 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h9e93f298b8db0112sQC
  57:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  58:      0x357477cb5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  59:      0x35743ee3f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::he3a07def9c733b33WgD
  60:      0x35746f7601d - fmt::_&'a T.Display::fmt::h15337095780985379402
  61:      0x35747828d53 - fmt::write::h4c1f14d6ead6957fW5U
  62:      0x35747761a74 - fmt::Write::write_fmt::h13270935105889183142
  63:      0x3574776192b - fmt::format::hdd6e6b41627bc4c0b6d
  64:      0x35746f75662 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  65:      0x35746f76420 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  66:      0x357449088bd - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hadb448f50916859cFOz
  67:      0x3574490e723 - visit::walk_expr::h2170439642982396722
  68:      0x35744908997 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hadb448f50916859cFOz
  69:      0x3574490e47b - visit::walk_expr::h2170439642982396722
  70:      0x35744908997 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hadb448f50916859cFOz
  71:      0x3574491562b - visit::walk_block::h8508057565682261575
  72:      0x3574490f0f7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_block::hb9e815e0cc5c8efa81z
  73:      0x3574490fc3b - visit::walk_fn::h4817495977315748029
  74:      0x35744908d06 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::h6f4801145ac36681LQz
  75:      0x3574490718e - visit::walk_item::h4744264594307728230
  76:      0x357448fecd5 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  77:      0x357448fb4d4 - lint::context::LintContext::with_lint_attrs::h8799401426682095770
  78:      0x357448f9fa0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h3cca3667903b2657XKz
  79:      0x3574491507c - visit::walk_mod::h16107070230948499447
  80:      0x35744908fac - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h35119e094710f76eUZz
  81:      0x357449071e1 - visit::walk_item::h4744264594307728230
  82:      0x357448fecd5 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  83:      0x357448fb4d4 - lint::context::LintContext::with_lint_attrs::h8799401426682095770
  84:      0x357448f9fa0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h3cca3667903b2657XKz
  85:      0x3574491507c - visit::walk_mod::h16107070230948499447
  86:      0x35744908fac - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h35119e094710f76eUZz
  87:      0x357449071e1 - visit::walk_item::h4744264594307728230
  88:      0x357448fecd5 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  89:      0x357448fb4d4 - lint::context::LintContext::with_lint_attrs::h8799401426682095770
  90:      0x357448f9fa0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h3cca3667903b2657XKz
  91:      0x3574491507c - visit::walk_mod::h16107070230948499447
  92:      0x35744908fac - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h35119e094710f76eUZz
  93:      0x35744941c85 - visit::walk_crate::h4677652414078288908
  94:      0x357449418b5 - lint::context::check_crate::closure.108246
  95:      0x35744940d44 - lint::context::LintContext::with_lint_attrs::h11098725645949648387
  96:      0x3574493f107 - lint::context::check_crate::he7e98e99dbe76181zPA
  97:      0x35747dbebc6 - driver::phase_3_run_analysis_passes::closure.23233
  98:      0x35747dbed8e - util::common::time::closure.23238
  99:      0x35747dbed32 - time::duration::Duration::span::h14426461393997687636
  100:      0x35747dbe5d7 - util::common::time::h2269135110856453074
 ... <frames omitted>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    71m11.324s
user    71m42.373s
sys 6m18.470s
