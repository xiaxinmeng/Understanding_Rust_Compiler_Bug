 bash
$ time make -j1 -- VERBOSE=1 NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose' RUST_BACKTRACE=1
cfg: version 1.5.0-dev (9a855668f 2015-10-23)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
...
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose -O --cfg rtopt -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
time: 0.093; rss: 37MB  parsing
time: 0.027; rss: 37MB  configuration 1
time: 0.000; rss: 37MB  recursion limit
time: 0.003; rss: 37MB  gated macro checking
time: 0.000; rss: 37MB  crate injection
time: 0.019; rss: 39MB  macro loading
time: 0.000; rss: 39MB  plugin loading
time: 0.000; rss: 39MB  plugin registration
time: 0.591; rss: 56MB  expansion
time: 0.015; rss: 56MB  complete gated feature checking 1
time: 0.122; rss: 56MB  configuration 2
time: 0.000; rss: 56MB  gated configuration checking
time: 0.063; rss: 56MB  maybe building test harness
time: 0.060; rss: 56MB  prelude injection
time: 0.009; rss: 56MB  checking that all macro invocations are gone
time: 0.000; rss: 56MB  checking for inline asm in case the target doesn't support it
time: 0.016; rss: 56MB  complete gated feature checking 2
time: 0.063; rss: 56MB  assigning node ids
time: 0.056; rss: 72MB  lowering ast -> hir
time: 0.062; rss: 76MB  indexing hir
time: 0.000; rss: 76MB  attribute checking
time: 0.030; rss: 76MB  early lint checks
time: 0.013; rss: 76MB  external crate/lib resolution
time: 0.014; rss: 76MB  language item collection
time: 0.258; rss: 104MB resolution
time: 0.011; rss: 104MB lifetime resolution
time: 0.000; rss: 104MB looking for entry point
time: 0.007; rss: 104MB looking for plugin registrar
time: 0.054; rss: 109MB region resolution
time: 0.007; rss: 109MB loop checking
time: 0.007; rss: 109MB static item recursion checking
time: 0.065; rss: 112MB type collecting
time: 0.014; rss: 112MB variance inference
time: 0.151; rss: 132MB coherence checking
time: 0.066; rss: 133MB wf checking (old)
time: 0.072; rss: 134MB item-types checking
time: 10.167; rss: 178MB    item-bodies checking
time: 0.000; rss: 178MB drop-impl checking
time: 0.314; rss: 178MB wf checking (new)
time: 0.936; rss: 179MB const checking
time: 0.056; rss: 179MB privacy checking
time: 0.002; rss: 179MB stability index
time: 0.024; rss: 179MB intrinsic checking
time: 0.018; rss: 179MB effect checking
time: 0.113; rss: 179MB match checking
time: 0.733; rss: 235MB MIR dump
time: 0.074; rss: 240MB liveness checking
time: 2.017; rss: 240MB borrow checking
time: 1.405; rss: 240MB rvalue checking
time: 0.018; rss: 240MB reachability checking
time: 0.054; rss: 240MB death checking
time: 0.048; rss: 240MB stability checking
time: 0.000; rss: 240MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x31d1ab3498e - sys::backtrace::tracing::imp::write::h2d5a2cc7b16951606tt
   2:      0x31d1ab3b725 - panicking::log_panic::closure.39357
   3:      0x31d1ab3b168 - panicking::log_panic::h3f7951020f1e8c6aFux
   4:      0x31d1aafc883 - sys_common::unwind::begin_unwind_inner::h4675f22ec9be8a79xls
   5:      0x31d1aafd207 - sys_common::unwind::begin_unwind_fmt::hc0cb2f54ec1e261dDks
   6:      0x31d18864b2b - metadata::csearch::get_item_path::h226cc1dedfee8adfVgs
   7:      0x31d1882c982 - middle::ty::ctxt<'tcx>::item_path_str::h459f98727bd9a23drFh
   8:      0x31d1882b6a4 - middle::def_id::DefId.fmt..Debug::fmt::hb8020078128825ceRfs
   9:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  10:      0x31d1ab56583 - fmt::Formatter<'a>::write_fmt::h7153214113fae136MsV
  11:      0x31d188996c8 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::he4def2c8fc9294f2k8B
  12:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  13:      0x31d1ab56583 - fmt::Formatter<'a>::write_fmt::h7153214113fae136MsV
  14:      0x31d18838d84 - util::ppaux::ty..Region.fmt..Debug::fmt::hba9ed157a2896600saC
  15:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  16:      0x31d1ab56583 - fmt::Formatter<'a>::write_fmt::h7153214113fae136MsV
  17:      0x31d18868410 - util::ppaux::ty..Region.fmt..Display::fmt::h8c5ae906c91c848ckkC
  18:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  19:      0x31d18a670db - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::h351b5c54ca402ab6tKB
  20:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  21:      0x31d1ab56583 - fmt::Formatter<'a>::write_fmt::h7153214113fae136MsV
  22:      0x31d18a6aed8 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h5433b7e72fc3b71csQC
  23:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  24:      0x31d1ab56583 - fmt::Formatter<'a>::write_fmt::h7153214113fae136MsV
  25:      0x31d186e2b87 - fmt::_&'a T.Display::fmt::h16601277798971783859
  26:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  27:      0x31d1ab56583 - fmt::Formatter<'a>::write_fmt::h7153214113fae136MsV
  28:      0x31d18a6aed8 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h5433b7e72fc3b71csQC
  29:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  30:      0x31d1ab56583 - fmt::Formatter<'a>::write_fmt::h7153214113fae136MsV
  31:      0x31d186e2b87 - fmt::_&'a T.Display::fmt::h16601277798971783859
  32:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  33:      0x31d1ab56583 - fmt::Formatter<'a>::write_fmt::h7153214113fae136MsV
  34:      0x31d18a63fed - util::ppaux::fn_sig::hb42d3375d120930c7eB
  35:      0x31d18a69046 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::h94b4f1146b794b34huC
  36:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  37:      0x31d1ab56583 - fmt::Formatter<'a>::write_fmt::h7153214113fae136MsV
  38:      0x31d18a6c288 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h5433b7e72fc3b71csQC
  39:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  40:      0x31d1ab56583 - fmt::Formatter<'a>::write_fmt::h7153214113fae136MsV
  41:      0x31d186e2be4 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::h081de51f061fc534WgD
  42:      0x31d1ab6f475 - fmt::write::h124c3c5cc3c7b9b4W5U
  43:      0x31d1ab3c702 - fmt::format::ha8e87cf9871fa02cb6d
  44:      0x31d1a483a83 - builtin::BoxPointers::check_heap_type::h539427dde4fb65d2JIa
  45:      0x31d1a483da9 - builtin::BoxPointers.LateLintPass::check_expr::h77f53baeae753019XLa
  46:      0x31d18a43e6f - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::he67b1b5547abcb86FOz
  47:      0x31d18a443ad - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::he67b1b5547abcb86FOz
  48:      0x31d18a4438e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::he67b1b5547abcb86FOz
  49:      0x31d18a4473b - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::h4b059ed74b1f5cb7LQz
  50:      0x31d18a3f9e4 - lint::context::LintContext::with_lint_attrs::h8750270744138178828
  51:      0x31d18a44a1e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h988a57955fb4f50aUZz
  52:      0x31d18a3fa16 - lint::context::LintContext::with_lint_attrs::h8750270744138178828
  53:      0x31d18a44a1e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h988a57955fb4f50aUZz
  54:      0x31d18a3fa16 - lint::context::LintContext::with_lint_attrs::h8750270744138178828
  55:      0x31d18a44a1e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h988a57955fb4f50aUZz
  56:      0x31d18a60d80 - lint::context::check_crate::h0665d99e192496a8zPA
  57:      0x31d1b053e17 - driver::phase_3_run_analysis_passes::closure.21888
  58:      0x31d1b034b63 - middle::ty::context::ctxt<'tcx>::create_and_enter::h1630972532939545665
  59:      0x31d1b02fadf - driver::phase_3_run_analysis_passes::h12145868880165070390
  60:      0x31d1b010ba6 - driver::compile_input::h32d1eac2df74dfb67ba
  61:      0x31d1b16928b - run_compiler::ha0e33d84ca5b13e4Ntc
  62:      0x31d1b166638 - sys_common::unwind::try::try_fn::h3019213119068281766
  63:      0x31d1ab32688 - __rust_try
  64:      0x31d1ab25bcb - sys_common::unwind::try::inner_try::h2f8af910b2af4dc45hs
  65:      0x31d1b166960 - boxed::F.FnBox<A>::call_box::h15447457551766661915
  66:      0x31d1ab39fe3 - sys::thread::Thread::new::thread_start::h7a81dbe415334083RMw
  67:      0x31d14bcb64b - start_thread
                        at /usr/src/debug/sys-libs/glibc-2.22-r1/glibc-2.22/nptl/pthread_create.c:334
  68:      0x31d1a7c89bc - clone
                        at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
  69:                0x0 - <unknown>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    48m9.737s
user    49m11.650s
sys 6m27.133s
