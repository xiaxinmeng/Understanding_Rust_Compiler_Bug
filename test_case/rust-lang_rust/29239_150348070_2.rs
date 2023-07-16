 bash
...
rustc: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front
time: 0.104; rss: 42MB  parsing
time: 0.027; rss: 42MB  configuration 1
time: 0.000; rss: 42MB  recursion limit
time: 0.003; rss: 42MB  gated macro checking
time: 0.000; rss: 42MB  crate injection
time: 0.022; rss: 44MB  macro loading
time: 0.000; rss: 44MB  plugin loading
time: 0.000; rss: 44MB  plugin registration
time: 0.634; rss: 61MB  expansion
time: 0.016; rss: 61MB  complete gated feature checking 1
time: 0.128; rss: 61MB  configuration 2
time: 0.000; rss: 61MB  gated configuration checking
time: 0.067; rss: 61MB  maybe building test harness
time: 0.062; rss: 61MB  prelude injection
time: 0.011; rss: 61MB  checking that all macro invocations are gone
time: 0.000; rss: 61MB  checking for inline asm in case the target doesn't support it
time: 0.017; rss: 61MB  complete gated feature checking 2
time: 0.064; rss: 61MB  assigning node ids
time: 0.061; rss: 78MB  lowering ast -> hir
time: 0.067; rss: 82MB  indexing hir
time: 0.000; rss: 82MB  attribute checking
time: 0.030; rss: 82MB  early lint checks
time: 0.014; rss: 82MB  external crate/lib resolution
time: 0.017; rss: 82MB  language item collection
time: 0.310; rss: 110MB resolution
time: 0.011; rss: 110MB lifetime resolution
time: 0.000; rss: 110MB looking for entry point
time: 0.007; rss: 110MB looking for plugin registrar
time: 0.062; rss: 115MB region resolution
time: 0.007; rss: 115MB loop checking
time: 0.007; rss: 115MB static item recursion checking
time: 0.077; rss: 119MB type collecting
time: 0.016; rss: 119MB variance inference
time: 0.180; rss: 138MB coherence checking
time: 0.080; rss: 139MB wf checking (old)
time: 0.086; rss: 139MB item-types checking
time: 11.824; rss: 184MB    item-bodies checking
time: 0.000; rss: 184MB drop-impl checking
time: 0.381; rss: 184MB wf checking (new)
time: 1.159; rss: 184MB const checking
time: 0.062; rss: 184MB privacy checking
time: 0.002; rss: 184MB stability index
time: 0.028; rss: 184MB intrinsic checking
time: 0.019; rss: 184MB effect checking
time: 0.129; rss: 184MB match checking
time: 0.863; rss: 240MB MIR dump
time: 0.086; rss: 246MB liveness checking
time: 2.469; rss: 246MB borrow checking
time: 1.729; rss: 246MB rvalue checking
time: 0.020; rss: 246MB reachability checking
time: 0.060; rss: 246MB death checking
time: 0.055; rss: 246MB stability checking
time: 0.000; rss: 246MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x38ad7869a38 - sys::backtrace::tracing::imp::write::h899a6e2564d068526tt
   2:      0x38ad7871f5a - panicking::log_panic::h87fed04ca3a1e1aeFux
   3:      0x38ad78285fa - sys_common::unwind::begin_unwind_inner::h39173c97d971c415xls
   4:      0x38ad78292a7 - sys_common::unwind::begin_unwind_fmt::h843e47803017c429Dks
   5:      0x38ad548822d - metadata::csearch::get_item_path::haf254411e6af40b2Vgs
   6:      0x38ad5446d1e - middle::ty::ctxt<'tcx>::item_path_str::h8743a7dc564b4820rFh
   7:      0x38ad53c5704 - middle::def_id::DefId.fmt..Debug::fmt::h68bfa4ce601e4c2dRfs
   8:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
   9:      0x38ad7890323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  10:      0x38ad5476d88 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::h64f95cc58162add8k8B
  11:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  12:      0x38ad7890323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  13:      0x38ad52c5ed4 - util::ppaux::ty..Region.fmt..Debug::fmt::hc7ea9fe49075b353saC
  14:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  15:      0x38ad7890323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  16:      0x38ad548b880 - util::ppaux::ty..Region.fmt..Display::fmt::hc7b656446610bdadkkC
  17:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  18:      0x38ad56cf2f1 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::haaa666102cb1810etKB
  19:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x38ad7890323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x38ad56d2bb5 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::ha7c91db2a712dae4sQC
  22:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  23:      0x38ad7890323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  24:      0x38ad52c6457 - fmt::_&'a T.Display::fmt::h11067680234946478933
  25:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  26:      0x38ad7890323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  27:      0x38ad56d2bb5 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::ha7c91db2a712dae4sQC
  28:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  29:      0x38ad7890323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  30:      0x38ad52c6457 - fmt::_&'a T.Display::fmt::h11067680234946478933
  31:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  32:      0x38ad7890323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  33:      0x38ad56cc18d - util::ppaux::fn_sig::haee0879fb5f346417eB
  34:      0x38ad56d0a86 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::h2d2456cefe42d74ahuC
  35:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  36:      0x38ad7890323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  37:      0x38ad56d3fd1 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::ha7c91db2a712dae4sQC
  38:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  39:      0x38ad7890323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  40:      0x38ad52c64b4 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hcf27bda9b6b5e170WgD
  41:      0x38ad78adec5 - fmt::write::h4c1f14d6ead6957fW5U
  42:      0x38ad7873ae2 - fmt::format::hdd6e6b41627bc4c0b6d
  43:      0x38ad71a5f33 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  44:      0x38ad71a6269 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  45:      0x38ad56ac36f - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hec3f20ad390aa884FOz
  46:      0x38ad56ac8ad - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hec3f20ad390aa884FOz
  47:      0x38ad56ac88e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hec3f20ad390aa884FOz
  48:      0x38ad56acc3b - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::hdb4396bb9bcbff6dLQz
  49:      0x38ad56a7ab0 - lint::context::LintContext::with_lint_attrs::h4984301093664340773
  50:      0x38ad56acf1e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h69eb9b55153d6977UZz
  51:      0x38ad56a7ae2 - lint::context::LintContext::with_lint_attrs::h4984301093664340773
  52:      0x38ad56acf1e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h69eb9b55153d6977UZz
  53:      0x38ad56a7ae2 - lint::context::LintContext::with_lint_attrs::h4984301093664340773
  54:      0x38ad56acf1e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h69eb9b55153d6977UZz
  55:      0x38ad56c88ca - lint::context::check_crate::h39448bc385c4ecf3zPA
  56:      0x38ad7d872ab - driver::phase_3_run_analysis_passes::closure.21957
  57:      0x38ad7d7fed2 - middle::ty::context::ctxt<'tcx>::create_and_enter::h16917164059176643116
  58:      0x38ad7d7b18d - driver::phase_3_run_analysis_passes::h15409790394034338059
  59:      0x38ad7d5d3fb - driver::compile_input::h0b1b46ede8c440c17ba
  60:      0x38ad7e4a62f - run_compiler::hbef8d0681748b40dNtc
  61:      0x38ad7e47bf8 - sys_common::unwind::try::try_fn::h9841971821030604291
  62:      0x38ad78670d8 - __rust_try
  63:      0x38ad785689b - sys_common::unwind::try::inner_try::hd160594970b417325hs
  64:      0x38ad7e47f20 - boxed::F.FnBox<A>::call_box::h14551616490890038848
  65:      0x38ad78706e3 - sys::thread::Thread::new::thread_start::h1b6f054d8b7016bdRMw
  66:      0x38ad0df164b - start_thread
                        at /usr/src/debug/sys-libs/glibc-2.22-r1/glibc-2.22/nptl/pthread_create.c:334
  67:      0x38ad74ea9bc - clone
                        at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
  68:                0x0 - <unknown>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    43m59.465s
user    41m30.936s
sys 2m11.740s


