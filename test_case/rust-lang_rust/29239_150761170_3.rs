 bash
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 'RUSTFLAGS=-Z verbose' RUST_BACKTRACE=1
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
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 -Z verbose  -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libstd-bb943c5a.rlib" => Duration { secs: 0, nanos: 373022 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcore-bb943c5a.rlib" => Duration { secs: 0, nanos: 250311 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcollections-bb943c5a.rlib" => Duration { secs: 0, nanos: 235645 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_unicode-bb943c5a.rlib" => Duration { secs: 0, nanos: 194089 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_system-bb943c5a.rlib" => Duration { secs: 0, nanos: 193600 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 339778 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc-bb943c5a.rlib" => Duration { secs: 0, nanos: 214133 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librand-bb943c5a.rlib" => Duration { secs: 0, nanos: 276711 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblibc-bb943c5a.rlib" => Duration { secs: 0, nanos: 206800 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 316800 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblog-bb943c5a.rlib" => Duration { secs: 0, nanos: 231244 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libsyntax-bb943c5a.rlib" => Duration { secs: 0, nanos: 242489 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libfmt_macros-bb943c5a.rlib" => Duration { secs: 0, nanos: 240533 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libserialize-bb943c5a.rlib" => Duration { secs: 0, nanos: 206311 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libterm-bb943c5a.rlib" => Duration { secs: 0, nanos: 208756 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_bitflags-bb943c5a.rlib" => Duration { secs: 0, nanos: 244445 }
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x37bbfba506c - sys::backtrace::tracing::imp::write::hcf25773dd1b9b1916tt
   2:      0x37bbfbc9036 - panicking::log_panic::closure.39699
   3:      0x37bbfbc8851 - panicking::log_panic::h3b637c6b446abf43Fux
   4:      0x37bbfb9a828 - panicking::on_panic::hc25d5876b810d5819xx
   5:      0x37bbfb02f6c - sys_common::unwind::begin_unwind_inner::h0e4a96b661d9f889xls
   6:      0x37bbfb04803 - sys_common::unwind::begin_unwind_fmt::h751259f99d216129Dks
   7:      0x37bbcb96489 - metadata::decoder::crate_metadata::lookup_item::hceaab574e3d51844Nlo
   8:      0x37bbcc34e48 - metadata::decoder::get_item_path::h7653a60f4b5eaeb3vjp
   9:      0x37bbc7bf9ab - metadata::csearch::get_item_path::h4248ccc6f7dfad90Vgs
  10:      0x37bbcb73e3c - middle::ty::ctxt<'tcx>::with_path::h11802873549257002857
  11:      0x37bbc71bc4e - middle::ty::ctxt<'tcx>::item_path_str::haf189c5c4c746d49rFh
  12:      0x37bbc71ba2b - middle::def_id::DefId.fmt..Debug::fmt::closure.91325
  13:      0x37bbc71b94f - middle::ty::context::tls::with_opt::closure.91323
  14:      0x37bbc71b8ff - middle::ty::context::tls::with::closure.91321
  15:      0x37bbc71b8b1 - thread::scoped_tls::ScopedKey<T>::with::h8755583549864709264
  16:      0x37bbc71b7fd - middle::ty::context::tls::with::h17277110619345102466
  17:      0x37bbc71b6d7 - middle::ty::context::tls::with_opt::h2247635784626063578
  18:      0x37bbc585a29 - middle::def_id::DefId.fmt..Debug::fmt::h9d21b5ba45368ca7Rfs
  19:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x37bbfc365ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x37bbc798e66 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::h9ead88964c7d3a6fk8B
  22:      0x37bbc79899d - fmt::_&'a T.Debug::fmt::h7893538456906839292
  23:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  24:      0x37bbfc365ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  25:      0x37bbc34b231 - util::ppaux::ty..Region.fmt..Debug::fmt::hf38e0208c656ffc1saC
  26:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  27:      0x37bbfc365ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  28:      0x37bbc7c4ea7 - util::ppaux::ty..Region.fmt..Display::fmt::h0ac6505701ba5076kkC
  29:      0x37bbcdb6cbd - fmt::_&'a T.Display::fmt::h13636915514064095164
  30:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  31:      0x37bbc311264 - fmt::Write::write_fmt::h12489643933446496354
  32:      0x37bbcdb6c0d - string::T.ToString::to_string::h13681150471017236373
  33:      0x37bbcdb8c15 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::hb37e6936c4f0d91dtKB
  34:      0x37bbcdd067d - boxed::Box<T>.fmt..Display::fmt::h8797919735308429487
  35:      0x37bbcdd064d - fmt::_&'a T.Display::fmt::h6108614467168194740
  36:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  37:      0x37bbfc365ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  38:      0x37bbcdcc68a - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h0c81dd8a70e315f6sQC
  39:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  40:      0x37bbfc365ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  41:      0x37bbc34ef79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::h81f4670088062beaWgD
  42:      0x37bbc34ee7d - fmt::_&'a T.Display::fmt::h5052431531228383809
  43:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  44:      0x37bbfc365ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  45:      0x37bbcdcaed2 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h0c81dd8a70e315f6sQC
  46:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  47:      0x37bbfc365ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  48:      0x37bbc34ef79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::h81f4670088062beaWgD
  49:      0x37bbc34ee7d - fmt::_&'a T.Display::fmt::h5052431531228383809
  50:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  51:      0x37bbfc365ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  52:      0x37bbcdb281b - util::ppaux::fn_sig::h11338447830479f77eB
  53:      0x37bbcdbee55 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::hc19bfeb46dd94738huC
  54:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  55:      0x37bbfc365ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  56:      0x37bbcdcc173 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h0c81dd8a70e315f6sQC
  57:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  58:      0x37bbfc365ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  59:      0x37bbc34ef79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::h81f4670088062beaWgD
  60:      0x37bbf3e101d - fmt::_&'a T.Display::fmt::h12512797739877677854
  61:      0x37bbfc93d53 - fmt::write::h4c1f14d6ead6957fW5U
  62:      0x37bbfbcca74 - fmt::Write::write_fmt::h16219922215161971288
  63:      0x37bbfbcc92b - fmt::format::hdd6e6b41627bc4c0b6d
  64:      0x37bbf3e0662 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  65:      0x37bbf3e1420 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  66:      0x37bbcd738ed - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h0b3412db14ae7ae2FOz
  67:      0x37bbcd79753 - visit::walk_expr::h17228878623890741144
  68:      0x37bbcd739c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h0b3412db14ae7ae2FOz
  69:      0x37bbcd794ab - visit::walk_expr::h17228878623890741144
  70:      0x37bbcd739c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h0b3412db14ae7ae2FOz
  71:      0x37bbcd8065b - visit::walk_block::h15485943839253648361
  72:      0x37bbcd7a127 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_block::h0376f125fa46642d81z
  73:      0x37bbcd7ac6b - visit::walk_fn::h478676160674048928
  74:      0x37bbcd73d36 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::hc2cc9a31c418dc01LQz
  75:      0x37bbcd721be - visit::walk_item::h8246449456039621649
  76:      0x37bbcd69d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  77:      0x37bbcd66504 - lint::context::LintContext::with_lint_attrs::h3842921709177915828
  78:      0x37bbcd64fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::hc46558438a52fd0dXKz
  79:      0x37bbcd800ac - visit::walk_mod::h13562637026557672212
  80:      0x37bbcd73fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h4e3d6dd14cb39fe2UZz
  81:      0x37bbcd72211 - visit::walk_item::h8246449456039621649
  82:      0x37bbcd69d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  83:      0x37bbcd66504 - lint::context::LintContext::with_lint_attrs::h3842921709177915828
  84:      0x37bbcd64fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::hc46558438a52fd0dXKz
  85:      0x37bbcd800ac - visit::walk_mod::h13562637026557672212
  86:      0x37bbcd73fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h4e3d6dd14cb39fe2UZz
  87:      0x37bbcd72211 - visit::walk_item::h8246449456039621649
  88:      0x37bbcd69d05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  89:      0x37bbcd66504 - lint::context::LintContext::with_lint_attrs::h3842921709177915828
  90:      0x37bbcd64fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::hc46558438a52fd0dXKz
  91:      0x37bbcd800ac - visit::walk_mod::h13562637026557672212
  92:      0x37bbcd73fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h4e3d6dd14cb39fe2UZz
  93:      0x37bbcdaccb5 - visit::walk_crate::h15877291248654138526
  94:      0x37bbcdac8e5 - lint::context::check_crate::closure.108246
  95:      0x37bbcdabd74 - lint::context::LintContext::with_lint_attrs::h488990810979617694
  96:      0x37bbcdaa137 - lint::context::check_crate::h2dc0186798c698d5zPA
  97:      0x37bc0229bc6 - driver::phase_3_run_analysis_passes::closure.23233
  98:      0x37bc022955b - util::common::time::h16580670055439290156
  99:      0x37bc01ed08a - driver::phase_3_run_analysis_passes::closure.21934
  100:      0x37bc01ec8de - middle::ty::context::tls::enter::closure.21926
 ... <frames omitted>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    6m27.951s
user    6m25.050s
sys 0m2.027s
