
thread 'rustc' panicked at 'Could not find drop glue for Ty([closure@src/librustc_driver/lib.rs:494:59: 506:18 ppm:pretty::PpMode, opt_uii:std::option::Option<pretty::UserIdentifiedItem>]) -- DropGlue(Ty: 4560228208) -- rustc_driver.cgu-0.', src/librustc_trans/glue.rs:219
stack backtrace:
   1:        0x10aa2d5b9 - std::sys::backtrace::tracing::imp::write::habd974eec51179ec
   2:        0x10aa3bfc0 - std::panicking::default_hook::{{closure}}::h723c2c1871f63513
   3:        0x10aa3a3d4 - std::panicking::default_hook::hd1cbe478d89db943
   4:        0x10aa3aac6 - std::panicking::rust_panic_with_hook::hb9e51cc78bf6920a
   5:        0x10aa3a914 - std::panicking::begin_panic::h1bc9c2b22b63e84a
   6:        0x10aa3a882 - std::panicking::begin_panic_fmt::ha843df1a651a1d04
   7:        0x1069c81cc - rustc_trans::glue::get_drop_glue_core::h7fe6df48d8013110
   8:        0x1069773f1 - rustc_trans::base::unsized_info::ha7784d67670ae524
   9:        0x106978218 - rustc_trans::base::unsize_thin_ptr::h722eb22b27e72d21
  10:        0x1069f7a0f - rustc_trans::mir::rvalue::<impl rustc_trans::mir::MirContext<'bcx, 'tcx>>::trans_rvalue_operand::h685ea1526e348d6b
  11:        0x1069e308b - rustc_trans::mir::block::<impl rustc_trans::mir::MirContext<'bcx, 'tcx>>::trans_block::hdeeee4856b5f0733
  12:        0x1069e0a76 - rustc_trans::mir::trans_mir::h0fc2d97a6125939e
  13:        0x10697d443 - rustc_trans::base::trans_closure::hc3c07ca31f9ed2e0
  14:        0x106a02cb9 - rustc_trans::trans_item::TransItem::define::hd2591d5f4d59e9a6
  15:        0x1069808be - rustc_trans::base::trans_crate::heae1d3de2b6c43f8
  16:        0x106497f33 - rustc_driver::driver::phase_4_translate_to_llvm::h9a6ace50899bdd34
  17:        0x1064d44ec - rustc_driver::driver::compile_input::{{closure}}::h0382874667e5d93d
  18:        0x1064bda9d - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h0de946251dacf778
  19:        0x10642d092 - rustc::ty::context::TyCtxt::create_and_enter::hd60a32a123ed477e
  20:        0x1064883c7 - rustc_driver::driver::compile_input::h4a9c4465bb71b3f3
  21:        0x1064ac1c6 - rustc_driver::run_compiler::he7e28d4ec9619a0f
  22:        0x1063fd3cf - std::panicking::try::do_call::hca6f65893d2350d0
  23:        0x10aa42c9a - __rust_maybe_catch_panic
  24:        0x10641d424 - <F as alloc::boxed::FnBox<A>>::call_box::h6ce4b753f356ad3c
  25:        0x10aa391b5 - std::sys::thread::Thread::new::thread_start::hf6ecc70f34c9b620
  26:     0x7fff8b15099c - _pthread_body
  27:     0x7fff8b150919 - _pthread_start

error: Could not compile `rustc_driver`.
