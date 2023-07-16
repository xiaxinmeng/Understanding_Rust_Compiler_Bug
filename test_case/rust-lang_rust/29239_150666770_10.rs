 bash
$ git clean -dfx
...
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --disable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust --release-channel=dev --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu
...
$ time make -j1 -- VERBOSE=1 NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose -C debug-assertions=y' RUST_BACKTRACE=1
...
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose -C debug-assertions=y -O --cfg rtopt -C debug-assertions=on -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
time: 0.103; rss: 37MB  parsing
time: 0.027; rss: 37MB  configuration 1
time: 0.000; rss: 37MB  recursion limit
time: 0.003; rss: 37MB  gated macro checking
time: 0.000; rss: 37MB  crate injection
time: 0.021; rss: 40MB  macro loading
time: 0.000; rss: 40MB  plugin loading
time: 0.000; rss: 40MB  plugin registration
time: 0.620; rss: 57MB  expansion
time: 0.016; rss: 57MB  complete gated feature checking 1
time: 0.126; rss: 57MB  configuration 2
time: 0.000; rss: 57MB  gated configuration checking
time: 0.067; rss: 57MB  maybe building test harness
time: 0.061; rss: 57MB  prelude injection
time: 0.010; rss: 57MB  checking that all macro invocations are gone
time: 0.000; rss: 57MB  checking for inline asm in case the target doesn't support it
time: 0.016; rss: 57MB  complete gated feature checking 2
time: 0.064; rss: 57MB  assigning node ids
time: 0.058; rss: 73MB  lowering ast -> hir
time: 0.064; rss: 77MB  indexing hir
time: 0.000; rss: 77MB  attribute checking
time: 0.029; rss: 77MB  early lint checks
time: 0.014; rss: 77MB  external crate/lib resolution
time: 0.014; rss: 77MB  language item collection
time: 0.302; rss: 106MB resolution
time: 0.010; rss: 105MB lifetime resolution
time: 0.000; rss: 105MB looking for entry point
time: 0.007; rss: 105MB looking for plugin registrar
time: 0.058; rss: 110MB region resolution
time: 0.007; rss: 110MB loop checking
time: 0.007; rss: 110MB static item recursion checking
time: 0.076; rss: 114MB type collecting
time: 0.015; rss: 114MB variance inference
time: 0.175; rss: 134MB coherence checking
time: 0.080; rss: 134MB wf checking (old)
time: 0.084; rss: 135MB item-types checking
time: 11.756; rss: 179MB    item-bodies checking
time: 0.000; rss: 179MB drop-impl checking
time: 0.382; rss: 179MB wf checking (new)
time: 1.158; rss: 180MB const checking
time: 0.056; rss: 180MB privacy checking
time: 0.002; rss: 180MB stability index
time: 0.027; rss: 180MB intrinsic checking
time: 0.018; rss: 180MB effect checking
time: 0.128; rss: 180MB match checking
time: 0.843; rss: 236MB MIR dump
time: 0.080; rss: 242MB liveness checking
time: 2.489; rss: 242MB borrow checking
time: 1.751; rss: 242MB rvalue checking
time: 0.020; rss: 242MB reachability checking
time: 0.057; rss: 242MB death checking
time: 0.053; rss: 242MB stability checking
time: 0.000; rss: 242MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x39e78ec4a38 - sys::backtrace::tracing::imp::write::hcf25773dd1b9b1916tt
   2:      0x39e78eccf5a - panicking::log_panic::h3b637c6b446abf43Fux
   3:      0x39e78e835fa - sys_common::unwind::begin_unwind_inner::h0e4a96b661d9f889xls
   4:      0x39e78e842a7 - sys_common::unwind::begin_unwind_fmt::h751259f99d216129Dks
   5:      0x39e76ae422d - metadata::csearch::get_item_path::h9c91b96e2a77c727Vgs
   6:      0x39e76aa2d1e - middle::ty::ctxt<'tcx>::item_path_str::h9894218abd41693erFh
   7:      0x39e76a21724 - middle::def_id::DefId.fmt..Debug::fmt::h1110f03ecaf5eefeRfs
   8:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
   9:      0x39e78eeb323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  10:      0x39e76ad2d88 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::hdf5575ba84cd10ffk8B
  11:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  12:      0x39e78eeb323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  13:      0x39e76921f34 - util::ppaux::ty..Region.fmt..Debug::fmt::h613eae4dd057955bsaC
  14:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  15:      0x39e78eeb323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  16:      0x39e76ae7880 - util::ppaux::ty..Region.fmt..Display::fmt::hb85e7a053b68bed7kkC
  17:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  18:      0x39e76d2b2f1 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::h0a56c97c6a0d4f79tKB
  19:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  20:      0x39e78eeb323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  21:      0x39e76d2ebb5 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h1d289b50c4cbb105sQC
  22:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  23:      0x39e78eeb323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  24:      0x39e769224b7 - fmt::_&'a T.Display::fmt::h8467757195011904285
  25:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  26:      0x39e78eeb323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  27:      0x39e76d2ebb5 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h1d289b50c4cbb105sQC
  28:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  29:      0x39e78eeb323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  30:      0x39e769224b7 - fmt::_&'a T.Display::fmt::h8467757195011904285
  31:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  32:      0x39e78eeb323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  33:      0x39e76d2818d - util::ppaux::fn_sig::h403e49b6939b86217eB
  34:      0x39e76d2ca86 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::ha6e0c7cebd6858bdhuC
  35:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  36:      0x39e78eeb323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  37:      0x39e76d2ffd1 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::h1d289b50c4cbb105sQC
  38:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  39:      0x39e78eeb323 - fmt::Formatter<'a>::write_fmt::hc619a91f18c6dbc1MsV
  40:      0x39e76922514 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::hde476d80b7bec109WgD
  41:      0x39e78f08ec5 - fmt::write::h4c1f14d6ead6957fW5U
  42:      0x39e78eceae2 - fmt::format::hdd6e6b41627bc4c0b6d
  43:      0x39e78800f33 - builtin::BoxPointers::check_heap_type::h1e674186eb19a6b9JIa
  44:      0x39e78801269 - builtin::BoxPointers.LateLintPass::check_expr::hd56fae423a4daa98XLa
  45:      0x39e76d0836f - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hadef356207c7e855FOz
  46:      0x39e76d088ad - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hadef356207c7e855FOz
  47:      0x39e76d0888e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::hadef356207c7e855FOz
  48:      0x39e76d08c3b - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::h2ccab687944ade1eLQz
  49:      0x39e76d03ab0 - lint::context::LintContext::with_lint_attrs::h16261739438895627980
  50:      0x39e76d08f1e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h79a04eabe9311fd9UZz
  51:      0x39e76d03ae2 - lint::context::LintContext::with_lint_attrs::h16261739438895627980
  52:      0x39e76d08f1e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h79a04eabe9311fd9UZz
  53:      0x39e76d03ae2 - lint::context::LintContext::with_lint_attrs::h16261739438895627980
  54:      0x39e76d08f1e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h79a04eabe9311fd9UZz
  55:      0x39e76d248ca - lint::context::check_crate::h7f46052e8de3cd2bzPA
  56:      0x39e793e226b - driver::phase_3_run_analysis_passes::closure.21934
  57:      0x39e793dae92 - middle::ty::context::ctxt<'tcx>::create_and_enter::h14048162438327005021
  58:      0x39e793d614d - driver::phase_3_run_analysis_passes::h17017259851998588982
  59:      0x39e793b83bb - driver::compile_input::ha274735457bb915e7ba
  60:      0x39e794a56ff - run_compiler::hb2715e90a44741ffNtc
  61:      0x39e794a2cc8 - sys_common::unwind::try::try_fn::h15964241977096575130
  62:      0x39e78ec20d8 - __rust_try
  63:      0x39e78eb189b - sys_common::unwind::try::inner_try::h8be52e7c9f3ddd0b5hs
  64:      0x39e794a2ff0 - boxed::F.FnBox<A>::call_box::h9900394337489069288
  65:      0x39e78ecb6e3 - sys::thread::Thread::new::thread_start::hffba513c641be6c3RMw
  66:      0x39e72dd464b - start_thread
                        at /usr/src/debug/sys-libs/glibc-2.22-r1/glibc-2.22/nptl/pthread_create.c:334
  67:      0x39e78b459bc - clone
                        at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
  68:                0x0 - <unknown>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    47m55.260s
user    48m37.520s
sys 6m26.337s

