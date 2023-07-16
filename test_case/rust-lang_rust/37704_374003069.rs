
$ RUST_BACKTRACE=full rustc -C target-feature=blup a.rs
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
error: internal compiler error: librustc_trans/mir/operand.rs:165: not immediate: OperandRef(Pair(([0 x i8]*:[0 x i8]* bitcast (<{ [5 x i8] }>* @byte_str.0 to [0 x i8]*)), (i64:i64 5)) @ TyLayout { ty: &str, details: LayoutDetails { variants: Single { index: 0 }, fields: Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 8 }], memory_index: [0, 1] }, abi: ScalarPair(Scalar { value: Pointer, valid_range: 1..=18446744073709551615 }, Scalar { value: Int(I64, false), valid_range: 0..=18446744073709551615 }), align: Align { abi: 3, pref: 3 }, size: Size { raw: 16 } } })

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
stack backtrace:
'blup' is not a recognized feature for this target (ignoring feature)
'blup' is not a recognized feature for this target (ignoring feature)
   0:     0x7ff692c7673b - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h80e3d199effc616f
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7ff692c42620 - std::sys_common::backtrace::print::hc002478693da1476
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7ff692c5fc0d - std::panicking::default_hook::{{closure}}::h37430734c0ea6d9e
                               at libstd/panicking.rs:207
   3:     0x7ff692c5f965 - std::panicking::default_hook::h30eaf17214e8e5c2
                               at libstd/panicking.rs:223
   4:     0x7ff68f317cdd - core::ops::function::Fn::call::heac9f1cc4543f9b2
   5:     0x7ff692c60159 - std::panicking::rust_panic_with_hook::hbafb424a3f50eff9
                               at libstd/panicking.rs:403
   6:     0x7ff68dd47ee9 - std::panicking::begin_panic::h4a0215f3e238d956
   7:     0x7ff68dd5e9a1 - rustc_errors::Handler::bug::heb6dc17d2ab1d95a
   8:     0x7ff68f408f1f - rustc::session::opt_span_bug_fmt::{{closure}}::he9d07775ba17a7b3
   9:     0x7ff68f41cab5 - rustc::ty::context::tls::with_opt::{{closure}}::h9c5275ed6cdbd434
  10:     0x7ff68f3b9653 - <std::thread::local::LocalKey<T>>::try_with::h08f28d3e1b52e2bb
  11:     0x7ff68f3ac836 - <std::thread::local::LocalKey<T>>::with::h7539baa26a4e5e13
  12:     0x7ff68f39aa9d - rustc::ty::context::tls::with::he9e80cb451a7b589
  13:     0x7ff68f39ace7 - rustc::ty::context::tls::with_opt::h149d1d54e5c54288
  14:     0x7ff68f54cd84 - rustc::session::opt_span_bug_fmt::h269a1dc72c25868f
  15:     0x7ff68f54ccf6 - rustc::session::bug_fmt::h11bddeac2e2cdc41
  16:     0x7ff68767cbce - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::hfc607340e2bee885
  17:     0x7ff68769cbfa - rustc_trans::mir::trans_mir::hacd4ae9bd6c10320
  18:     0x7ff6876500a3 - rustc_trans::base::trans_instance::he9988f1cd825eb9e
  19:     0x7ff6876c643b - rustc_trans::trans_item::MonoItemExt::define::h8e363bd9aadb81f3
  20:     0x7ff687653f0e - rustc_trans::base::compile_codegen_unit::h4c64493e311a1ce7
  21:     0x7ff68f2ad5f1 - rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::compute_result::h0c004619646bd047
  22:     0x7ff68eed5e95 - rustc::dep_graph::graph::DepGraph::with_task_impl::h26420e242ad80f20
  23:     0x7ff68ef5617c - rustc_errors::Handler::track_diagnostics::h6b251640a41365f9
  24:     0x7ff68f1798c7 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h42171f076656eab9
  25:     0x7ff68f2ad699 - rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::force::h254a15116e25dd10
  26:     0x7ff68f2adf89 - rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::try_get::h3658396412f4ee0c
  27:     0x7ff68f1e849c - rustc::ty::maps::TyCtxtAt::compile_codegen_unit::h7b6b7aa4068eb96e
  28:     0x7ff68f1e0f49 - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::compile_codegen_unit::h8acc6f298c6f702c
  29:     0x7ff687652242 - rustc_trans::base::trans_crate::h491bd0aa983d77e5
  30:     0x7ff6876ac217 - <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate::h8fa7219d42bbe3d7
  31:     0x7ff693019ed5 - rustc::util::common::time::hbc445b62b4a46695
  32:     0x7ff692ff6a84 - rustc_driver::driver::phase_4_translate_to_llvm::hd6f62df2e1d698fd
  33:     0x7ff693081db9 - rustc_driver::driver::compile_input::{{closure}}::ha27d02edec625da5
  34:     0x7ff69307819b - <std::thread::local::LocalKey<T>>::with::h1a3db1841a6b56bd
  35:     0x7ff69307cd96 - <std::thread::local::LocalKey<T>>::with::ha3878bc1dbb615cd
  36:     0x7ff6930efb15 - rustc::ty::context::TyCtxt::create_and_enter::h10ccec467f18ea66
  37:     0x7ff692fefe36 - rustc_driver::driver::compile_input::ha722bd4f7b807288
  38:     0x7ff6930a61a6 - rustc_driver::run_compiler_impl::ha9e10e026fa1c2a7
  39:     0x7ff692fcac45 - syntax::with_globals::h1ecafa7bcfe0b83d
  40:     0x7ff69301ed6d - std::sys_common::backtrace::__rust_begin_short_backtrace::h93297af0ae7aee56
  41:     0x7ff692c830ae - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:102
  42:     0x7ff693029377 - <F as alloc::boxed::FnBox<A>>::call_box::h2b3206b763bfecab
  43:     0x7ff692c7a237 - std::sys_common::thread::start_thread::h84df27afb6fd881e
                               at /checkout/src/liballoc/boxed.rs:793
                               at libstd/sys_common/thread.rs:24
  44:     0x7ff692c48fd8 - std::sys::unix::thread::Thread::new::thread_start::hadeb1c0dcdc116b6
                               at libstd/sys/unix/thread.rs:90
  45:     0x7ff68d083089 - start_thread
  46:     0x7ff69292847e - __clone
  47:                0x0 - <unknown>


