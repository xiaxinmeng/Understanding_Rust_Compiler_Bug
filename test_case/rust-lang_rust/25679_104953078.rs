
stack backtrace:
   1:     0x7fef2dd7a1d9 - sys::backtrace::write::ha86cd627a22f455205r
                        at /home/ariel/Rust/source.master/src/libstd/sys/unix/backtrace.rs:158
   2:     0x7fef2dd836cd - panicking::on_panic::h662858ec46f876968Lw
                        at /home/ariel/Rust/source.master/src/libstd/panicking.rs:47
   3:     0x7fef2dd417eb - rt::unwind::begin_unwind_inner::h496e2ee4be82b2feirw
                        at /home/ariel/Rust/source.master/src/libstd/rt/unwind/mod.rs:273
   4:     0x7fef2dd428ad - rt::unwind::begin_unwind_fmt::hb66a627b5352ee99oqw
                        at /home/ariel/Rust/source.master/src/libstd/rt/unwind/mod.rs:212
   5:     0x7fef2dd83237 - rust_begin_unwind
   6:     0x7fef2dde34aa - panicking::panic_fmt::h130c36cdf6ed19des0B
                        at /home/ariel/Rust/source.master/src/libcore/panicking.rs:64
   7:     0x7fef2dddde34 - panicking::panic_bounds_check::h97c07c7c114b10a5yZB
                        at /home/ariel/Rust/source.master/src/libcore/panicking.rs:52
   8:     0x7fef2be0e57c - middle::infer::InferCtxt<'a, 'tcx>::shallow_resolve::h56e9f53812e21da7nCz
                        at /home/ariel/Rust/source.master/src/libcollections/vec.rs:1357
                        at /home/ariel/Rust/source.master/src/librustc_data_structures/snapshot_vec.rs:95
                        at /home/ariel/Rust/source.master/src/librustc/middle/infer/type_variable.rs:126
                        at /home/ariel/Rust/source.master/src/librustc/middle/infer/mod.rs:892
   9:     0x7fef2bdd3bce - middle::infer::resolve::OpportunisticTypeResolver<'a, 'tcx>.ty_fold..TypeFolder<'tcx>::fold_ty::h2c428c2906d7983fyBx
                        at /home/ariel/Rust/source.master/src/librustc/middle/infer/resolve.rs:43
  10:     0x7fef2bdd4699 - middle::ty_fold::TypeFolder::fold_substs::h1287471176495252976
                        at /home/ariel/Rust/source.master/src/librustc/middle/ty_fold.rs:236
                        at /home/ariel/Rust/source.master/src/librustc/middle/ty_fold.rs:224
                        at /home/ariel/Rust/source.master/src/libcore/iter.rs:1693
                        at /home/ariel/Rust/source.master/src/libcore/option.rs:420
                        at /home/ariel/Rust/source.master/src/libcore/iter.rs:1693
                        at /home/ariel/Rust/source.master/src/libcollections/vec.rs:1501
                        at /home/ariel/Rust/source.master/src/libcore/iter.rs:567
                        at /home/ariel/Rust/source.master/src/librustc/middle/subst.rs:445
                        at /home/ariel/Rust/source.master/src/librustc/middle/ty_fold.rs:219
                        at /home/ariel/Rust/source.master/src/librustc/middle/ty_fold.rs:677
                        at /home/ariel/Rust/source.master/src/librustc/middle/ty_fold.rs:97
  11:     0x7fef2bea32dc - middle::traits::project::opt_normalize_projection_type::hb8a09e9b2e53377dfWP
                        at /home/ariel/Rust/source.master/src/librustc/middle/ty_fold.rs:293
                        at /home/ariel/Rust/source.master/src/librustc/middle/ty_fold.rs:722
                        at /home/ariel/Rust/source.master/src/librustc/middle/ty_fold.rs:91
                        at /home/ariel/Rust/source.master/src/librustc/middle/ty_fold.rs:272
                        at /home/ariel/Rust/source.master/src/librustc/middle/infer/mod.rs:931
                        at /home/ariel/Rust/source.master/src/librustc/middle/traits/project.rs:452
                        at /home/ariel/Rust/source.master/src/librustc/middle/traits/project.rs:357
  12:     0x7fef2be9dc75 - middle::traits::project::poly_project_and_unify_type::closure.93245
                        at /home/ariel/Rust/source.master/src/librustc/middle/traits/project.rs:116
                        at /home/ariel/Rust/source.master/src/librustc/middle/traits/project.rs:88
  13:     0x7fef2be9c398 - middle::traits::project::poly_project_and_unify_type::h42e4cd0d7749283coCP
                        at /home/ariel/Rust/source.master/src/librustc/middle/infer/mod.rs:575
                        at /home/ariel/Rust/source.master/src/librustc/middle/traits/project.rs:83
  14:     0x7fef2be937e6 - middle::traits::fulfill::FulfillmentContext<'tcx>::select::h541a7c10f723947dIeP
                        at /home/ariel/Rust/source.master/src/librustc/middle/traits/fulfill.rs:396
                        at /home/ariel/Rust/source.master/src/librustc/middle/traits/fulfill.rs:273
                        at /home/ariel/Rust/source.master/src/libcollections/vec.rs:601
                        at /home/ariel/Rust/source.master/src/librustc/middle/traits/fulfill.rs:268
  15:     0x7fef2be93283 - middle::traits::fulfill::FulfillmentContext<'tcx>::select_new_obligations::h7a5ae48603f1abd2ndP
                        at /home/ariel/Rust/source.master/src/librustc/middle/traits/fulfill.rs:218
  16:     0x7fef2d520546 - check::FnCtxt<'a, 'tcx>::select_new_obligations::h8b913ca888d49e450Mp
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:1854
  17:     0x7fef2d54ae66 - check::check_argument_types::h10dce567107b606bwgq
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:2308
  18:     0x7fef2d54c8b7 - check::check_method_argument_types::hade9045e5e301411Vdq
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:2176
  19:     0x7fef2d577317 - check::check_expr_with_unifier::check_method_call::h0a9b0a37195319d12Pq
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:2646
  20:     0x7fef2d59dd79 - check::check_expr_with_unifier::h12507636429091130913
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:3390
  21:     0x7fef2d5ad57d - check::check_stmt::h3f64930c48de8110aos
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:2494
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:3946
  22:     0x7fef2d5635fa - check::check_block_with_expected::hbc58b92bc05f127flss
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:3988
  23:     0x7fef2d5440e6 - check::check_fn::h478e5dc8b923df63W4n
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:705
  24:     0x7fef2d55d4d9 - check::check_bare_fn::h9b9c796f0d1e44a6wUn
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:524
  25:     0x7fef2d55b48f - check::check_item_body::h26d9292503efa05amlo
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:801
  26:     0x7fef2d55d049 - check::check_item_types::h141edac9d460c3eb3Rn
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:463
                        at /home/ariel/Rust/source.master/src/libsyntax/visit.rs:160
                        at /home/ariel/Rust/source.master/src/libsyntax/visit.rs:64
                        at /home/ariel/Rust/source.master/src/libsyntax/visit.rs:152
                        at /home/ariel/Rust/source.master/src/librustc_typeck/check/mod.rs:483
  27:     0x7fef2d6143e9 - check_crate::hbe43b6c5b585bffdp7C
                        at /home/ariel/Rust/source.master/src/librustc_typeck/lib.rs:341
                        at /home/ariel/Rust/source.master/src/librustc/util/common.rs:39
                        at /home/ariel/Rust/source.master/src/librustc_typeck/lib.rs:340
  28:     0x7fef2e2f4a7b - driver::phase_3_run_analysis_passes::h89f35bf8a39eb6d0tGa
                        at /home/ariel/Rust/source.master/src/librustc_driver/driver.rs:643
  29:     0x7fef2e2df119 - driver::compile_input::h3827478501655be2Qba
                        at /home/ariel/Rust/source.master/src/librustc_driver/driver.rs:122
  30:     0x7fef2e37d475 - run_compiler::h65b602076525bee675b
                        at /home/ariel/Rust/source.master/src/librustc_driver/lib.rs:156
  31:     0x7fef2e37b03c - boxed::F.FnBox<A>::call_box::h10880508204101052423
                        at /home/ariel/Rust/source.master/src/librustc_driver/lib.rs:99
                        at /home/ariel/Rust/source.master/src/librustc_driver/lib.rs:813
                        at /home/ariel/Rust/source.master/src/liballoc/boxed.rs:379
  32:     0x7fef2e37a87e - rt::unwind::try::try_fn::h8650079312810836346
  33:     0x7fef2de1f818 - rust_try_inner
  34:     0x7fef2de1f805 - rust_try
  35:     0x7fef2dd6e0bc - rt::unwind::try::inner_try::he54be6185e919225bnw
                        at /home/ariel/Rust/source.master/src/libstd/rt/unwind/mod.rs:147
  36:     0x7fef2e37aac0 - boxed::F.FnBox<A>::call_box::h10028273920383414105
                        at /home/ariel/Rust/source.master/src/libstd/rt/unwind/mod.rs:130
                        at /home/ariel/Rust/source.master/src/libstd/thread/mod.rs:346
                        at /home/ariel/Rust/source.master/src/liballoc/boxed.rs:379
  37:     0x7fef2dd8244e - sys::thread::Thread::new::thread_start::hd9103ec8865158f2rxv
  38:     0x7fef2882b0a3 - start_thread
  39:     0x7fef2d9be04c - clone
  40:                0x0 - <unknown>
