
error: internal compiler error: librustc/infer/error_reporting/mod.rs:184: impossible case reached

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
stack backtrace:
error: internal compiler error: librustc/infer/error_reporting/mod.rs:184: impossible case reached

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
stack backtrace:
   0:     0x7f6793e60f4b - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h88d4b1300e862de6
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f6793e2ea00 - std::sys_common::backtrace::print::ha4b25ea03535b6bc
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7f6793e5442d - std::panicking::default_hook::{{closure}}::h89f6c65d34e612c1
                               at libstd/panicking.rs:207
   3:     0x7f6793e54185 - std::panicking::default_hook::h97a3d20007e3cd68
                               at libstd/panicking.rs:223
   4:     0x7f679048a0bd - core::ops::function::Fn::call::h21c5f1174484de1a
   5:     0x7f6793e54979 - std::panicking::rust_panic_with_hook::h47bfd51197e881f4
                               at libstd/panicking.rs:403
   6:     0x7f678eea1269 - std::panicking::begin_panic::he66ba5a4f20558a9
   7:     0x7f678eebb4a1 - rustc_errors::Handler::bug::hd2b47daace8879c7
   8:     0x7f679057aabf - rustc::session::opt_span_bug_fmt::{{closure}}::h7a285e675f7ecad3
   9:     0x7f679058ee35 - rustc::ty::context::tls::with_opt::{{closure}}::hb0c60bbc299dec38
  10:     0x7f679052b643 - <std::thread::local::LocalKey<T>>::try_with::h0a97f593c5dcf1c7
  11:     0x7f6790522466 - <std::thread::local::LocalKey<T>>::with::h726e9ee6cbe6f2db
  12:     0x7f679034146d - rustc::ty::context::tls::with::hc95b50ac92e9f143
  13:     0x7f67903419d7 - rustc::ty::context::tls::with_opt::ha16a77063b42a873
  14:     0x7f67906bc6f4 - rustc::session::opt_span_bug_fmt::h85e42a1462a7acb2
  15:     0x7f67906bc666 - rustc::session::bug_fmt::h9e6b1e77a7b65211
  16:     0x7f67902a9ed3 - rustc::infer::error_reporting::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::msg_span_from_free_region::he2ac2a4633d5dfff
  17:     0x7f67901595d7 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_region::hd2f22229a00c57c3
  18:     0x7f679059491e - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  19:     0x7f6790159876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  20:     0x7f6790478284 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h123b1b0451a30840
  21:     0x7f6790583969 - rustc::ty::fold::TypeFoldable::fold_with::hadc69145d18b9346
  22:     0x7f679059477f - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  23:     0x7f6790159876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  24:     0x7f679047c6ec - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h8377cf69a3c2c1f0
  25:     0x7f6790580c69 - rustc::ty::fold::TypeFoldable::fold_with::h2dc9610f10be1f49
  26:     0x7f67905948dd - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  27:     0x7f6790159876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  28:     0x7f6790124e56 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &'a mut F>::call_once::h3a462fd54cf08b8d
  29:     0x7f679047ac13 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h61f2f9442aee3a46
  30:     0x7f67906da2b0 - <T as rustc::ty::context::InternIteratorElement<T, R>>::intern_with::hddaaaf54af2f5922
  31:     0x7f6790159828 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  32:     0x7f6790478284 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h123b1b0451a30840
  33:     0x7f6790583969 - rustc::ty::fold::TypeFoldable::fold_with::hadc69145d18b9346
  34:     0x7f679059477f - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  35:     0x7f6790159876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  36:     0x7f6790478284 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h123b1b0451a30840
  37:     0x7f6790583969 - rustc::ty::fold::TypeFoldable::fold_with::hadc69145d18b9346
  38:     0x7f679059477f - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  39:     0x7f6790159876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  40:     0x7f6790478284 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h123b1b0451a30840
  41:     0x7f6790583969 - rustc::ty::fold::TypeFoldable::fold_with::hadc69145d18b9346
  42:     0x7f679059477f - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  43:     0x7f6790159876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  44:     0x7f6790478284 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h123b1b0451a30840
  45:     0x7f6790583969 - rustc::ty::fold::TypeFoldable::fold_with::hadc69145d18b9346
  46:     0x7f679059477f - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  47:     0x7f6790159876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  48:     0x7f67901a0464 - rustc::infer::anon_types::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::infer_anon_definition_from_instantiation::h4382306e1f859274
  49:     0x7f679260c322 - rustc_typeck::check::writeback::WritebackCx::visit_anon_types::hb5eb94c81332802f
  50:     0x7f67925a40a7 - rustc_typeck::check::writeback::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::resolve_type_vars_in_body::h340dbe93fe6a6d61
  51:     0x7f6792649726 - rustc::ty::context::tls::enter::ha8ae9527661ca966
  52:     0x7f679261b560 - rustc::infer::InferCtxtBuilder::enter::h384a6cfe140c0e55
  53:     0x7f67925ba441 - rustc_typeck::check::typeck_tables_of::h045a25f5cc18b4e1
  54:     0x7f679005b29a - rustc::dep_graph::graph::DepGraph::with_task_impl::h38209dd06e555259
  55:     0x7f6790013009 - rustc_errors::Handler::track_diagnostics::hb92a0cb7ed53a4fb
  56:     0x7f67902e3657 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h4e245e41f290c214
  57:     0x7f6790395d3e - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::force::h577074b5c2a71819
  58:     0x7f67903965e2 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get::h663470b06f1a9af4
  59:     0x7f679023365a - rustc::ty::maps::TyCtxtAt::typeck_tables_of::hef9e3f61ef0afd39
  60:     0x7f6790395c31 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure::h7a7f7d4e96880ddf
  61:     0x7f67927105bd - rustc::session::Session::track_errors::h16333b127c93c6e4
  62:     0x7f67925b9fba - rustc_typeck::check::typeck_item_bodies::hf17344a5285ef991
  63:     0x7f679008638b - rustc::dep_graph::graph::DepGraph::with_task_impl::hd12fc88c46806a2c
  64:     0x7f6790004306 - rustc_errors::Handler::track_diagnostics::h816b34bde43fe45f
  65:     0x7f67902fb377 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h8c005ba7475fc1fb
  66:     0x7f6790394a9c - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::force::h97a52420d1392948
  67:     0x7f67903952af - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get::h53db8e36934be5c4
  68:     0x7f6790233575 - rustc::ty::maps::TyCtxtAt::typeck_item_bodies::h0ff25c11fabef17e
  69:     0x7f679035338e - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies::he8b170ee0ad4618c
  70:     0x7f67927355c2 - rustc_typeck::check_crate::h9abc42612774df7a
  71:     0x7f6794268fc2 - <std::thread::local::LocalKey<T>>::with::hff6cfdad62c33763
  72:     0x7f6794268886 - <std::thread::local::LocalKey<T>>::with::hf73e2e411d5e1bbd
  73:     0x7f67942db5cb - rustc::ty::context::TyCtxt::create_and_enter::h4882ed8ffc6c0dcf
  74:     0x7f67941d8166 - rustc_driver::driver::compile_input::h976478050b890adf
  75:     0x7f679428fc36 - rustc_driver::run_compiler_impl::h1d61215e89cf59af
  76:     0x7f67941d5205 - syntax::with_globals::h5f781b83399af32f
  77:     0x7f679420809d - std::sys_common::backtrace::__rust_begin_short_backtrace::h3d7c503aecc53ac6
  78:     0x7f6793e6f06e - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:102
  79:     0x7f67942160f6 - <F as alloc::boxed::FnBox<A>>::call_box::h5cb8e83b4182087c
  80:     0x7f6793e661e7 - std::sys_common::thread::start_thread::hf4cb05e0f65cf12c
                               at /checkout/src/liballoc/boxed.rs:793
                               at libstd/sys_common/thread.rs:24
  81:     0x7f6793e34f98 - std::sys::unix::thread::Thread::new::thread_start::h7f0887f31dcc98be
                               at libstd/sys/unix/thread.rs:90
  82:     0x7f678e1dd7fb - start_thread
  83:     0x7f6793b0bb5e - clone
  84:                0x0 - <unknown>
   0:     0x7f6f458c5f4b - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h88d4b1300e862de6
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f6f45893a00 - std::sys_common::backtrace::print::ha4b25ea03535b6bc
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7f6f458b942d - std::panicking::default_hook::{{closure}}::h89f6c65d34e612c1
                               at libstd/panicking.rs:207
   3:     0x7f6f458b9185 - std::panicking::default_hook::h97a3d20007e3cd68
                               at libstd/panicking.rs:223
   4:     0x7f6f41eef0bd - core::ops::function::Fn::call::h21c5f1174484de1a
   5:     0x7f6f458b9979 - std::panicking::rust_panic_with_hook::h47bfd51197e881f4
                               at libstd/panicking.rs:403
   6:     0x7f6f40906269 - std::panicking::begin_panic::he66ba5a4f20558a9
   7:     0x7f6f409204a1 - rustc_errors::Handler::bug::hd2b47daace8879c7
   8:     0x7f6f41fdfabf - rustc::session::opt_span_bug_fmt::{{closure}}::h7a285e675f7ecad3
   9:     0x7f6f41ff3e35 - rustc::ty::context::tls::with_opt::{{closure}}::hb0c60bbc299dec38
  10:     0x7f6f41f90643 - <std::thread::local::LocalKey<T>>::try_with::h0a97f593c5dcf1c7
  11:     0x7f6f41f87466 - <std::thread::local::LocalKey<T>>::with::h726e9ee6cbe6f2db
  12:     0x7f6f41da646d - rustc::ty::context::tls::with::hc95b50ac92e9f143
  13:     0x7f6f41da69d7 - rustc::ty::context::tls::with_opt::ha16a77063b42a873
  14:     0x7f6f421216f4 - rustc::session::opt_span_bug_fmt::h85e42a1462a7acb2
  15:     0x7f6f42121666 - rustc::session::bug_fmt::h9e6b1e77a7b65211
  16:     0x7f6f41d0eed3 - rustc::infer::error_reporting::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::msg_span_from_free_region::he2ac2a4633d5dfff
  17:     0x7f6f41bbe5d7 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_region::hd2f22229a00c57c3
  18:     0x7f6f41ff991e - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  19:     0x7f6f41bbe876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  20:     0x7f6f41edd284 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h123b1b0451a30840
  21:     0x7f6f41fe8969 - rustc::ty::fold::TypeFoldable::fold_with::hadc69145d18b9346
  22:     0x7f6f41ff977f - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  23:     0x7f6f41bbe876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  24:     0x7f6f41ee16ec - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h8377cf69a3c2c1f0
  25:     0x7f6f41fe5c69 - rustc::ty::fold::TypeFoldable::fold_with::h2dc9610f10be1f49
  26:     0x7f6f41ff98dd - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  27:     0x7f6f41bbe876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  28:     0x7f6f41b89e56 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &'a mut F>::call_once::h3a462fd54cf08b8d
  29:     0x7f6f41edfc13 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h61f2f9442aee3a46
  30:     0x7f6f4213f2b0 - <T as rustc::ty::context::InternIteratorElement<T, R>>::intern_with::hddaaaf54af2f5922
  31:     0x7f6f41bbe828 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  32:     0x7f6f41edd284 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h123b1b0451a30840
  33:     0x7f6f41fe8969 - rustc::ty::fold::TypeFoldable::fold_with::hadc69145d18b9346
  34:     0x7f6f41ff977f - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  35:     0x7f6f41bbe876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  36:     0x7f6f41edd284 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h123b1b0451a30840
  37:     0x7f6f41fe8969 - rustc::ty::fold::TypeFoldable::fold_with::hadc69145d18b9346
  38:     0x7f6f41ff977f - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  39:     0x7f6f41bbe876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  40:     0x7f6f41edd284 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h123b1b0451a30840
  41:     0x7f6f41fe8969 - rustc::ty::fold::TypeFoldable::fold_with::hadc69145d18b9346
  42:     0x7f6f41ff977f - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  43:     0x7f6f41bbe876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  44:     0x7f6f41edd284 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h123b1b0451a30840
  45:     0x7f6f41fe8969 - rustc::ty::fold::TypeFoldable::fold_with::hadc69145d18b9346
  46:     0x7f6f41ff977f - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with::hef9c449b0c4daaae
  47:     0x7f6f41bbe876 - <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h5c8b9267b7259e21
  48:     0x7f6f41c05464 - rustc::infer::anon_types::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::infer_anon_definition_from_instantiation::h4382306e1f859274
  49:     0x7f6f44071322 - rustc_typeck::check::writeback::WritebackCx::visit_anon_types::hb5eb94c81332802f
  50:     0x7f6f440090a7 - rustc_typeck::check::writeback::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::resolve_type_vars_in_body::h340dbe93fe6a6d61
  51:     0x7f6f440ae726 - rustc::ty::context::tls::enter::ha8ae9527661ca966
  52:     0x7f6f44080560 - rustc::infer::InferCtxtBuilder::enter::h384a6cfe140c0e55
  53:     0x7f6f4401f441 - rustc_typeck::check::typeck_tables_of::h045a25f5cc18b4e1
  54:     0x7f6f41ac029a - rustc::dep_graph::graph::DepGraph::with_task_impl::h38209dd06e555259
  55:     0x7f6f41a78009 - rustc_errors::Handler::track_diagnostics::hb92a0cb7ed53a4fb
  56:     0x7f6f41d48657 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h4e245e41f290c214
  57:     0x7f6f41dfad3e - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::force::h577074b5c2a71819
  58:     0x7f6f41dfb5e2 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get::h663470b06f1a9af4
  59:     0x7f6f41c9865a - rustc::ty::maps::TyCtxtAt::typeck_tables_of::hef9e3f61ef0afd39
  60:     0x7f6f41dfac31 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure::h7a7f7d4e96880ddf
  61:     0x7f6f441755bd - rustc::session::Session::track_errors::h16333b127c93c6e4
  62:     0x7f6f4401efba - rustc_typeck::check::typeck_item_bodies::hf17344a5285ef991
  63:     0x7f6f41aeb38b - rustc::dep_graph::graph::DepGraph::with_task_impl::hd12fc88c46806a2c
  64:     0x7f6f41a69306 - rustc_errors::Handler::track_diagnostics::h816b34bde43fe45f
  65:     0x7f6f41d60377 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h8c005ba7475fc1fb
  66:     0x7f6f41df9a9c - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::force::h97a52420d1392948
  67:     0x7f6f41dfa2af - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get::h53db8e36934be5c4
  68:     0x7f6f41c98575 - rustc::ty::maps::TyCtxtAt::typeck_item_bodies::h0ff25c11fabef17e
  69:     0x7f6f41db838e - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies::he8b170ee0ad4618c
  70:     0x7f6f4419a5c2 - rustc_typeck::check_crate::h9abc42612774df7a
  71:     0x7f6f45ccdfc2 - <std::thread::local::LocalKey<T>>::with::hff6cfdad62c33763
  72:     0x7f6f45ccd886 - <std::thread::local::LocalKey<T>>::with::hf73e2e411d5e1bbd
  73:     0x7f6f45d405cb - rustc::ty::context::TyCtxt::create_and_enter::h4882ed8ffc6c0dcf
  74:     0x7f6f45c3d166 - rustc_driver::driver::compile_input::h976478050b890adf
  75:     0x7f6f45cf4c36 - rustc_driver::run_compiler_impl::h1d61215e89cf59af
  76:     0x7f6f45c3a205 - syntax::with_globals::h5f781b83399af32f
  77:     0x7f6f45c6d09d - std::sys_common::backtrace::__rust_begin_short_backtrace::h3d7c503aecc53ac6
  78:     0x7f6f458d406e - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:102
  79:     0x7f6f45c7b0f6 - <F as alloc::boxed::FnBox<A>>::call_box::h5cb8e83b4182087c
  80:     0x7f6f458cb1e7 - std::sys_common::thread::start_thread::hf4cb05e0f65cf12c
                               at /checkout/src/liballoc/boxed.rs:793
                               at libstd/sys_common/thread.rs:24
  81:     0x7f6f45899f98 - std::sys::unix::thread::Thread::new::thread_start::h7f0887f31dcc98be
                               at libstd/sys/unix/thread.rs:90
  82:     0x7f6f3fc427fb - start_thread
  83:     0x7f6f45570b5e - clone
  84:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (c08480fce 2018-03-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `hivemind`.
warning: build failed, waiting for other jobs to finish...

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (c08480fce 2018-03-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden
