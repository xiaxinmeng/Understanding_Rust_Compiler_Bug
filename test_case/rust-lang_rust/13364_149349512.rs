 bash
...
"cc" "-Wl,--as-needed" "-m64" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/rbml-bb943c5a.0.o" "-o" "x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librbml-bb943c5a.so" "x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/rbml-bb943c5a.metadata.o" "-Wl,-O1" "-nodefaultlibs" "-L" "x86_64-unknown-linux-gnu/rt" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/.rust/lib/x86_64-unknown-linux-gnu" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/lib/x86_64-unknown-linux-gnu" "-Wl,-Bstatic" "-Wl,-Bdynamic" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "serialize-bb943c5a" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "log-bb943c5a" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "std-bb943c5a" "-l" "dl" "-l" "pthread" "-l" "gcc_s" "-l" "rt" "-l" "pthread" "-l" "c" "-l" "m" "-shared" "-l" "compiler-rt"
  time: 0.170; rss: 118MB running linker
time: 0.214; rss: 118MB linking
rustc: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front
time: 0.103; rss: 41MB  parsing
time: 0.028; rss: 41MB  configuration 1
time: 0.000; rss: 41MB  recursion limit
time: 0.003; rss: 41MB  gated macro checking
time: 0.000; rss: 41MB  crate injection
time: 0.021; rss: 44MB  macro loading
time: 0.000; rss: 44MB  plugin loading
time: 0.000; rss: 44MB  plugin registration
time: 0.625; rss: 61MB  expansion
time: 0.017; rss: 61MB  complete gated feature checking 1
time: 0.128; rss: 61MB  configuration 2
time: 0.000; rss: 61MB  gated configuration checking
time: 0.068; rss: 61MB  maybe building test harness
time: 0.063; rss: 61MB  prelude injection
time: 0.011; rss: 61MB  checking that all macro invocations are gone
time: 0.000; rss: 61MB  checking for inline asm in case the target doesn't support it
time: 0.016; rss: 61MB  complete gated feature checking 2
time: 0.066; rss: 61MB  assigning node ids
time: 0.060; rss: 78MB  lowering ast -> hir
time: 0.067; rss: 82MB  indexing hir
time: 0.000; rss: 82MB  attribute checking
time: 0.030; rss: 82MB  early lint checks
time: 0.015; rss: 82MB  external crate/lib resolution
time: 0.015; rss: 82MB  language item collection
time: 0.310; rss: 110MB resolution
time: 0.011; rss: 108MB lifetime resolution
time: 0.000; rss: 108MB looking for entry point
time: 0.007; rss: 108MB looking for plugin registrar
time: 0.061; rss: 115MB region resolution
time: 0.007; rss: 115MB loop checking
time: 0.007; rss: 115MB static item recursion checking
time: 0.076; rss: 118MB type collecting
time: 0.016; rss: 118MB variance inference
time: 0.175; rss: 138MB coherence checking
time: 0.079; rss: 139MB wf checking (old)
time: 0.085; rss: 139MB item-types checking
time: 11.778; rss: 183MB  item-bodies checking
time: 0.000; rss: 183MB drop-impl checking
time: 0.374; rss: 183MB wf checking (new)
time: 1.133; rss: 184MB const checking
time: 0.061; rss: 184MB privacy checking
time: 0.002; rss: 184MB stability index
time: 0.028; rss: 184MB intrinsic checking
time: 0.018; rss: 184MB effect checking
time: 0.127; rss: 184MB match checking
time: 0.852; rss: 241MB MIR dump
time: 0.093; rss: 241MB liveness checking
time: 2.441; rss: 243MB borrow checking
time: 1.710; rss: 243MB rvalue checking
time: 0.020; rss: 243MB reachability checking
time: 0.060; rss: 243MB death checking
time: 0.055; rss: 243MB stability checking
time: 0.000; rss: 243MB unused lib feature checking
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'lookup_item: id not found: DefIndex(25591)', src/librustc/metadata/decoder.rs:70
stack backtrace:
   1:      0x39b677f0658 - sys::backtrace::tracing::imp::write::h6acdb16e3fcdc374Llt
   2:      0x39b677f8b7a - panicking::log_panic::h55d6d561c9b3f308kmx
   3:      0x39b677afc7a - sys_common::unwind::begin_unwind_inner::hc2221b23d3f472a3cds
   4:      0x39b677b0927 - sys_common::unwind::begin_unwind_fmt::h8cc8814c2d254de2ics
   5:      0x39b65414ddd - metadata::csearch::get_item_path::h73a9d92ce367e2eeSgs
   6:      0x39b653d38ce - middle::ty::ctxt<'tcx>::item_path_str::h70934397b62e3884oFh
   7:      0x39b65352414 - middle::def_id::DefId.fmt..Debug::fmt::h378dc8cac1e8dabfRfs
   8:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
   9:      0x39b67816f43 - fmt::Formatter<'a>::write_fmt::h451c26e46516c241MsV
  10:      0x39b65403938 - util::ppaux::ty..BoundRegion.fmt..Debug::fmt::h9a73f61881422f50h8B
  11:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  12:      0x39b67816f43 - fmt::Formatter<'a>::write_fmt::h451c26e46516c241MsV
  13:      0x39b65252ed4 - util::ppaux::ty..Region.fmt..Debug::fmt::h2d973de65f46263dpaC
  14:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  15:      0x39b67816f43 - fmt::Formatter<'a>::write_fmt::h451c26e46516c241MsV
  16:      0x39b65418430 - util::ppaux::ty..Region.fmt..Display::fmt::h76d64307e8508c70hkC
  17:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  18:      0x39b6565bea1 - util::ppaux::ty..TraitTy<'tcx>.fmt..Display::fmt::h4a63bd3cd220925aqKB
  19:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  20:      0x39b67816f43 - fmt::Formatter<'a>::write_fmt::h451c26e46516c241MsV
  21:      0x39b6565f765 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::hd11a9bc66d0d7870pQC
  22:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  23:      0x39b67816f43 - fmt::Formatter<'a>::write_fmt::h451c26e46516c241MsV
  24:      0x39b65253457 - fmt::_&'a T.Display::fmt::h15942671037336040400
  25:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  26:      0x39b67816f43 - fmt::Formatter<'a>::write_fmt::h451c26e46516c241MsV
  27:      0x39b6565f765 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::hd11a9bc66d0d7870pQC
  28:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  29:      0x39b67816f43 - fmt::Formatter<'a>::write_fmt::h451c26e46516c241MsV
  30:      0x39b65253457 - fmt::_&'a T.Display::fmt::h15942671037336040400
  31:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  32:      0x39b67816f43 - fmt::Formatter<'a>::write_fmt::h451c26e46516c241MsV
  33:      0x39b65658d3d - util::ppaux::fn_sig::h744c0e49457176224eB
  34:      0x39b6565d636 - util::ppaux::ty..FnSig<'tcx>.fmt..Display::fmt::h04329f5b5b3c4d45euC
  35:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  36:      0x39b67816f43 - fmt::Formatter<'a>::write_fmt::h451c26e46516c241MsV
  37:      0x39b65660b81 - util::ppaux::ty..TypeVariants<'tcx>.fmt..Display::fmt::hd11a9bc66d0d7870pQC
  38:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  39:      0x39b67816f43 - fmt::Formatter<'a>::write_fmt::h451c26e46516c241MsV
  40:      0x39b652534b4 - util::ppaux::ty..TyS<'tcx>.fmt..Display::fmt::h7a866d69a974fa3bTgD
  41:      0x39b67834ae5 - fmt::write::hfaf37fa4269e57f8W5U
  42:      0x39b677fa702 - fmt::format::h98eb201c6b81efd6b6d
  43:      0x39b6712ff73 - builtin::BoxPointers::check_heap_type::h3dfb3953def18b7fJIa
  44:      0x39b671302a9 - builtin::BoxPointers.LateLintPass::check_expr::h7497471b623af763XLa
  45:      0x39b65638f1f - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h3761231cad9d3c90COz
  46:      0x39b6563945d - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h3761231cad9d3c90COz
  47:      0x39b6563943e - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_expr::h3761231cad9d3c90COz
  48:      0x39b656397eb - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_fn::hb9a0e2f22e4abd73IQz
  49:      0x39b65634660 - lint::context::LintContext::with_lint_attrs::h15040779467197164097
  50:      0x39b65639ace - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h42023e6788fd95ffRZz
  51:      0x39b65634692 - lint::context::LintContext::with_lint_attrs::h15040779467197164097
  52:      0x39b65639ace - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h42023e6788fd95ffRZz
  53:      0x39b65634692 - lint::context::LintContext::with_lint_attrs::h15040779467197164097
  54:      0x39b65639ace - lint::context::LateContext<'a, 'tcx>.hir_visit..Visitor<'v>::visit_mod::h42023e6788fd95ffRZz
  55:      0x39b6565547a - lint::context::check_crate::hf5a4487b48adfac8wPA
  56:      0x39b67d0ae37 - driver::phase_3_run_analysis_passes::closure.21907
  57:      0x39b67d0443e - middle::ty::context::ctxt<'tcx>::create_and_enter::h7386103329418218730
  58:      0x39b67cff6dc - driver::phase_3_run_analysis_passes::h10954767262469787437
  59:      0x39b67ce18ac - driver::compile_input::heb3c55b612369f8e7ba
  60:      0x39b67dcda3f - run_compiler::ha54fbfbd0c9d87b2wtc
  61:      0x39b67dcb008 - sys_common::unwind::try::try_fn::h15489298796838710061
  62:      0x39b677edcf8 - __rust_try
  63:      0x39b677dd4bb - sys_common::unwind::try::inner_try::h73e0f0125bf34e00K9r
  64:      0x39b67dcb330 - boxed::F.FnBox<A>::call_box::h6336268132757882836
  65:      0x39b677f7303 - sys::thread::Thread::new::thread_start::h56869fc3a7ac2f95wEw
  66:      0x39b60d8150b - start_thread
                        at /usr/src/debug/sys-libs/glibc-2.21-r1/glibc-2.21/nptl/pthread_create.c:333
  67:      0x39b6747314c - clone
                        at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
  68:                0x0 - <unknown>

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real  62m55.268s
user  54m44.073s
sys 7m14.430s

