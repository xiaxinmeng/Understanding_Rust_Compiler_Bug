 bash
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose -O --cfg rtopt -C debug-assertions=on -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
time: 0.101; rss: 37MB  parsing
time: 0.027; rss: 37MB  configuration 1
time: 0.000; rss: 37MB  recursion limit
time: 0.003; rss: 37MB  gated macro checking
time: 0.000; rss: 37MB  crate injection
time: 0.022; rss: 40MB  macro loading
time: 0.000; rss: 40MB  plugin loading
time: 0.000; rss: 40MB  plugin registration
time: 0.625; rss: 57MB  expansion
time: 0.015; rss: 57MB  complete gated feature checking 1
time: 0.126; rss: 57MB  configuration 2
time: 0.000; rss: 57MB  gated configuration checking
time: 0.066; rss: 57MB  maybe building test harness
time: 0.062; rss: 57MB  prelude injection
time: 0.009; rss: 57MB  checking that all macro invocations are gone
time: 0.000; rss: 57MB  checking for inline asm in case the target doesn't support it
time: 0.016; rss: 57MB  complete gated feature checking 2
time: 0.064; rss: 57MB  assigning node ids
time: 0.058; rss: 73MB  lowering ast -> hir
time: 0.064; rss: 77MB  indexing hir
time: 0.000; rss: 77MB  attribute checking
time: 0.029; rss: 77MB  early lint checks
time: 0.014; rss: 77MB  external crate/lib resolution
time: 0.014; rss: 77MB  language item collection
time: 0.303; rss: 105MB resolution
time: 0.010; rss: 105MB lifetime resolution
time: 0.000; rss: 105MB looking for entry point
time: 0.007; rss: 105MB looking for plugin registrar
time: 0.058; rss: 110MB region resolution
time: 0.007; rss: 110MB loop checking
time: 0.007; rss: 110MB static item recursion checking
time: 0.075; rss: 114MB type collecting
time: 0.015; rss: 114MB variance inference
time: 0.176; rss: 133MB coherence checking
time: 0.078; rss: 134MB wf checking (old)
time: 0.084; rss: 134MB item-types checking
time: 11.873; rss: 179MB    item-bodies checking
time: 0.000; rss: 179MB drop-impl checking
time: 0.384; rss: 179MB wf checking (new)
time: 1.158; rss: 180MB const checking
time: 0.057; rss: 180MB privacy checking
time: 0.002; rss: 180MB stability index
time: 0.028; rss: 180MB intrinsic checking
time: 0.018; rss: 180MB effect checking
time: 0.129; rss: 180MB match checking
time: 0.845; rss: 236MB MIR dump
time: 0.081; rss: 242MB liveness checking
time: 2.478; rss: 242MB borrow checking
time: 1.751; rss: 242MB rvalue checking
time: 0.019; rss: 242MB reachability checking
time: 0.058; rss: 242MB death checking
time: 0.054; rss: 242MB stability checking
time: 0.000; rss: 242MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x38340714a38 - sys::backtrace::tracing::imp::write::hcf25773dd1b9b1916tt
   2:      0x3834071cf5a - panicking::log_panic::h3b637c6b446abf43Fux
   3:      0x383406d35fa - sys_common::unwind::begin_unwind_inner::h0e4a96b661d9f889xls
   4:      0x383406d42a7 - sys_common::unwind::begin_unwind_fmt::h751259f99d216129Dks
   5:      0x3833e33421d - metadata::csearch::get_item_path::h70f5f93d243be122Vgs
   6:      0x3833e2f2d0e - middle::ty::ctxt<'tcx>::item_path_str::h45ce387d608b7db3rFh
   7:      0x3833e271704 - middle::def_id::DefId.fmt..Debug::fmt::he590029d7fb28cbfRfs
   8:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
   9:      0x3834073b323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  10:      0x3833e322d78 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::ha73f605b254004dck8B
  11:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  12:      0x3834073b323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  13:      0x3833e171f14 - util::ppaux::ty..Region.fmt..Debug::fmt::h8b270a32cfea8018saC
  14:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  15:      0x3834073b323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  16:      0x3833e337870 - util::ppaux::ty..Region.fmt..Display::fmt::h35bf08dbe54f9347kkC
  17:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  18:      0x3833e57b2e1 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::h4ddefc70d767ce82tKB
  19:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x3834073b323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x3833e57eba5 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h373d82cc69934918sQC
  22:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  23:      0x3834073b323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  24:      0x3833e172497 - fmt::_&'a T.Display::fmt::h17437513097113132107
  25:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  26:      0x3834073b323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  27:      0x3833e57eba5 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h373d82cc69934918sQC
  28:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  29:      0x3834073b323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  30:      0x3833e172497 - fmt::_&'a T.Display::fmt::h17437513097113132107
  31:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  32:      0x3834073b323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  33:      0x3833e57817d - util::ppaux::fn_sig::h628f78b47dba9c8a7eB
  34:      0x3833e57ca76 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::h99e6103927c92b4bhuC
  35:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  36:      0x3834073b323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  37:      0x3833e57ffc1 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h373d82cc69934918sQC
  38:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  39:      0x3834073b323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  40:      0x3833e1724f4 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::h6564d41ef8431640WgD
  41:      0x38340758ec5 - fmt::write::h4c1f14d6ead6957fW5U
  42:      0x3834071eae2 - fmt::format::hdd6e6b41627bc4c0b6d
  43:      0x38340050f33 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  44:      0x38340051269 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  45:      0x3833e55835f - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h89f40dfeb41e6869FOz
  46:      0x3833e55889d - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h89f40dfeb41e6869FOz
  47:      0x3833e55887e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h89f40dfeb41e6869FOz
  48:      0x3833e558c2b - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::h153b26e628a497e6LQz
  49:      0x3833e553aa0 - lint::context::LintContext::with_lint_attrs::h5962470200543390272
  50:      0x3833e558f0e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h0962538cafb1f1ecUZz
  51:      0x3833e553ad2 - lint::context::LintContext::with_lint_attrs::h5962470200543390272
  52:      0x3833e558f0e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h0962538cafb1f1ecUZz
  53:      0x3833e553ad2 - lint::context::LintContext::with_lint_attrs::h5962470200543390272
  54:      0x3833e558f0e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h0962538cafb1f1ecUZz
  55:      0x3833e5748ba - lint::context::check_crate::hb3002bf504c296f1zPA
  56:      0x38340c3226b - driver::phase_3_run_analysis_passes::closure.21934
  57:      0x38340c2ae92 - middle::ty::context::ctxt<'tcx>::create_and_enter::h5307164291102342190
  58:      0x38340c2614d - driver::phase_3_run_analysis_passes::h4674212053012284899
  59:      0x38340c083bb - driver::compile_input::ha274735457bb915e7ba
  60:      0x38340cf565f - run_compiler::hb2715e90a44741ffNtc
  61:      0x38340cf2c28 - sys_common::unwind::try::try_fn::h11439842781582378645
  62:      0x383407120d8 - __rust_try
  63:      0x3834070189b - sys_common::unwind::try::inner_try::h8be52e7c9f3ddd0b5hs
  64:      0x38340cf2f50 - boxed::F.FnBox<A>::call_box::h15925977592388706307
  65:      0x3834071b6e3 - sys::thread::Thread::new::thread_start::hffba513c641be6c3RMw
  66:      0x3833a62464b - start_thread
                        at /usr/src/debug/sys-libs/glibc-2.22-r1/glibc-2.22/nptl/pthread_create.c:334
  67:      0x383403959bc - clone
                        at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
  68:                0x0 - <unknown>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    47m58.383s
user    48m46.303s
sys 6m25.190s
