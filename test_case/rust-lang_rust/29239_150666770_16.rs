 bash
$ make -j1 -- VERBOSE=1 NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes' RUST_BACKTRACE=1
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
touch /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/llvm-auto-clean-stamp.start_time
...
info: now are following matches for librbml-*.rlib libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librbml-bb943c5a.rlib
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes  -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
time: 0.895; rss: 44MB  parsing
time: 0.160; rss: 44MB  configuration 1
time: 0.000; rss: 44MB  recursion limit
time: 0.016; rss: 44MB  gated macro checking
time: 0.000; rss: 44MB  crate injection
time: 0.165; rss: 47MB  macro loading
time: 0.000; rss: 47MB  plugin loading
time: 0.000; rss: 47MB  plugin registration
time: 4.833; rss: 65MB  expansion
time: 0.214; rss: 65MB  complete gated feature checking 1
time: 0.860; rss: 65MB  configuration 2
time: 0.000; rss: 65MB  gated configuration checking
time: 0.446; rss: 65MB  maybe building test harness
time: 0.411; rss: 65MB  prelude injection
time: 0.056; rss: 65MB  checking that all macro invocations are gone
time: 0.000; rss: 65MB  checking for inline asm in case the target doesn't support it
time: 0.213; rss: 65MB  complete gated feature checking 2
time: 0.455; rss: 65MB  assigning node ids
time: 0.268; rss: 83MB  lowering ast -> hir
time: 0.998; rss: 87MB  indexing hir
time: 0.000; rss: 87MB  attribute checking
time: 0.278; rss: 87MB  early lint checks
time: 0.114; rss: 87MB  external crate/lib resolution
time: 0.098; rss: 87MB  language item collection
time: 3.699; rss: 117MB resolution
time: 0.104; rss: 117MB lifetime resolution
time: 0.000; rss: 117MB looking for entry point
time: 0.049; rss: 117MB looking for plugin registrar
time: 0.587; rss: 123MB region resolution
time: 0.050; rss: 123MB loop checking
time: 0.049; rss: 123MB static item recursion checking
time: 0.882; rss: 130MB type collecting
time: 0.111; rss: 130MB variance inference
time: 1.919; rss: 150MB coherence checking
time: 1.002; rss: 151MB wf checking (old)
time: 1.073; rss: 151MB item-types checking
time: 169.469; rss: 196MB   item-bodies checking
time: 0.001; rss: 196MB drop-impl checking
time: 5.156; rss: 196MB wf checking (new)
time: 16.428; rss: 197MB    const checking
time: 0.485; rss: 197MB privacy checking
time: 0.021; rss: 197MB stability index
time: 0.259; rss: 197MB intrinsic checking
time: 0.127; rss: 197MB effect checking
time: 1.332; rss: 197MB match checking
time: 8.091; rss: 255MB MIR dump
time: 1.008; rss: 260MB liveness checking
time: 38.793; rss: 260MB    borrow checking
time: 25.934; rss: 260MB    rvalue checking
time: 0.227; rss: 260MB reachability checking
time: 0.531; rss: 260MB death checking
time: 0.585; rss: 260MB stability checking
time: 0.000; rss: 260MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x3b04044a06c - sys::backtrace::tracing::imp::write::hcf25773dd1b9b1916tt
   2:      0x3b04046e036 - panicking::log_panic::closure.39699
   3:      0x3b04046d851 - panicking::log_panic::h3b637c6b446abf43Fux
   4:      0x3b04043f828 - panicking::on_panic::hc25d5876b810d5819xx
   5:      0x3b0403a7f6c - sys_common::unwind::begin_unwind_inner::h0e4a96b661d9f889xls
   6:      0x3b0403a9803 - sys_common::unwind::begin_unwind_fmt::h751259f99d216129Dks
   7:      0x3b03d43b489 - metadata::decoder::crate_metadata::lookup_item::h8458d7c690c170ccNlo
   8:      0x3b03d4d9e48 - metadata::decoder::get_item_path::h946966a169ac7b24vjp
   9:      0x3b03d0649ab - metadata::csearch::get_item_path::h60727c6baa34e7e5Vgs
  10:      0x3b03d418e3c - middle::ty::ctxt<'tcx>::with_path::h14554981355384456658
  11:      0x3b03cfc0c4e - middle::ty::ctxt<'tcx>::item_path_str::h81905e6d96970dferFh
  12:      0x3b03cfc0a2b - middle::def_id::DefId.fmt..Debug::fmt::closure.91325
  13:      0x3b03cfc094f - middle::ty::context::tls::with_opt::closure.91323
  14:      0x3b03cfc08ff - middle::ty::context::tls::with::closure.91321
  15:      0x3b03cfc08b1 - thread::scoped_tls::ScopedKey<T>::with::h12505835326874039887
  16:      0x3b03cfc07fd - middle::ty::context::tls::with::h14253763777022529485
  17:      0x3b03cfc06d7 - middle::ty::context::tls::with_opt::h15396217575617304720
  18:      0x3b03ce2aa29 - middle::def_id::DefId.fmt..Debug::fmt::h7309ea013736c781Rfs
  19:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x3b0404db5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x3b03d03de66 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::hd78bd6fb8305e912k8B
  22:      0x3b03d03d99d - fmt::_&'a T.Debug::fmt::h18004347074300142245
  23:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  24:      0x3b0404db5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  25:      0x3b03cbf0231 - util::ppaux::ty..Region.fmt..Debug::fmt::h349b1e2e79452933saC
  26:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  27:      0x3b0404db5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  28:      0x3b03d069ea7 - util::ppaux::ty..Region.fmt..Display::fmt::h7e0a453c29708dc8kkC
  29:      0x3b03d65bcbd - fmt::_&'a T.Display::fmt::h7192573172774523792
  30:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  31:      0x3b03cbb6264 - fmt::Write::write_fmt::h627681005756376071
  32:      0x3b03d65bc0d - string::T.ToString::to_string::h16477161664338783167
  33:      0x3b03d65dc15 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::h2784e5e6ac79831dtKB
  34:      0x3b03d67567d - boxed::Box<T>.fmt..Display::fmt::h18082183109925588610
  35:      0x3b03d67564d - fmt::_&'a T.Display::fmt::h13316276846189391132
  36:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  37:      0x3b0404db5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  38:      0x3b03d67168a - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h0ca456edae539601sQC
  39:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  40:      0x3b0404db5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  41:      0x3b03cbf3f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::haef18a49c3cb1ee2WgD
  42:      0x3b03cbf3e7d - fmt::_&'a T.Display::fmt::h18154365338086587585
  43:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  44:      0x3b0404db5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  45:      0x3b03d66fed2 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h0ca456edae539601sQC
  46:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  47:      0x3b0404db5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  48:      0x3b03cbf3f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::haef18a49c3cb1ee2WgD
  49:      0x3b03cbf3e7d - fmt::_&'a T.Display::fmt::h18154365338086587585
  50:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  51:      0x3b0404db5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  52:      0x3b03d65781b - util::ppaux::fn_sig::h6eca8593cf4785247eB
  53:      0x3b03d663e55 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::h7617d88323b7a192huC
  54:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  55:      0x3b0404db5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  56:      0x3b03d671173 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h0ca456edae539601sQC
  57:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  58:      0x3b0404db5ff - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  59:      0x3b03cbf3f79 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::haef18a49c3cb1ee2WgD
  60:      0x3b03fc8601d - fmt::_&'a T.Display::fmt::h4772995322088294892
  61:      0x3b040538d53 - fmt::write::h4c1f14d6ead6957fW5U
  62:      0x3b040471a74 - fmt::Write::write_fmt::h10968932166685698339
  63:      0x3b04047192b - fmt::format::hdd6e6b41627bc4c0b6d
  64:      0x3b03fc85662 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  65:      0x3b03fc86420 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  66:      0x3b03d6188ed - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::he4258933855a2f48FOz
  67:      0x3b03d61e753 - visit::walk_expr::h7202680961895252098
  68:      0x3b03d6189c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::he4258933855a2f48FOz
  69:      0x3b03d61e4ab - visit::walk_expr::h7202680961895252098
  70:      0x3b03d6189c7 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::he4258933855a2f48FOz
  71:      0x3b03d62565b - visit::walk_block::h10455732252507424449
  72:      0x3b03d61f127 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_block::ha243b121dc521e1f81z
  73:      0x3b03d61fc6b - visit::walk_fn::h11620098901893989604
  74:      0x3b03d618d36 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::h11df07de935f81d2LQz
  75:      0x3b03d6171be - visit::walk_item::h18406932623791079324
  76:      0x3b03d60ed05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  77:      0x3b03d60b504 - lint::context::LintContext::with_lint_attrs::h11835066362755006506
  78:      0x3b03d609fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h57d037404f5286f1XKz
  79:      0x3b03d6250ac - visit::walk_mod::h7318219855343182972
  80:      0x3b03d618fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h13bf901ad767cdf3UZz
  81:      0x3b03d617211 - visit::walk_item::h18406932623791079324
  82:      0x3b03d60ed05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  83:      0x3b03d60b504 - lint::context::LintContext::with_lint_attrs::h11835066362755006506
  84:      0x3b03d609fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h57d037404f5286f1XKz
  85:      0x3b03d6250ac - visit::walk_mod::h7318219855343182972
  86:      0x3b03d618fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h13bf901ad767cdf3UZz
  87:      0x3b03d617211 - visit::walk_item::h18406932623791079324
  88:      0x3b03d60ed05 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::closure.108164
  89:      0x3b03d60b504 - lint::context::LintContext::with_lint_attrs::h11835066362755006506
  90:      0x3b03d609fd0 - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_item::h57d037404f5286f1XKz
  91:      0x3b03d6250ac - visit::walk_mod::h7318219855343182972
  92:      0x3b03d618fdc - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h13bf901ad767cdf3UZz
  93:      0x3b03d651cb5 - visit::walk_crate::h13005888902521590913
  94:      0x3b03d6518e5 - lint::context::check_crate::closure.108246
  95:      0x3b03d650d74 - lint::context::LintContext::with_lint_attrs::h11456058053036407341
  96:      0x3b03d64f137 - lint::context::check_crate::h7b1bd09c4649b906zPA
  97:      0x3b040acdbc6 - driver::phase_3_run_analysis_passes::closure.23233
  98:      0x3b040acdd8e - util::common::time::closure.23238
  99:      0x3b040acdd32 - time::duration::Duration::span::h16207229507280863009
  100:      0x3b040acd5d7 - util::common::time::h9240277138020009504
 ... <frames omitted>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    58m45.696s
user    59m44.783s
sys 6m19.740s

