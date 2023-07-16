
INFO:rustc_trans::base: trans_closure(..., std::fs::OpenOptions::open::<&std::path::Path>)
INFO:rustc_trans::base: trans_closure(..., std::fs::File::open::<&str>)
INFO:rustc_trans::base: trans_closure(..., std::io::read_to_end::<std::fs::File>)
INFO:rustc_trans::base: trans_closure(..., std::io::append_to_string::<[closure@DefId { krate: CrateNum(1), node: DefIndex(4235) => std/75fe2fdafd75f237ff013f4ea8c6fac39cd701ee2ed7455130a3f74d5902d10c::io[0]::Read[0]::read_to_string[0]::{{closure}}[0] } 0:&mut &mut std::fs::File]>)
INFO:rustc_trans::base: trans_closure(..., error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', ../src/libcore/option.rs:323
stack backtrace:
   1:     0x7ff02394c43f - std::sys::backtrace::tracing::imp::write::h22f199c1dbb72ba2
   2:     0x7ff02395b94d - std::panicking::default_hook::{{closure}}::h9a389c462b6a22dd
   3:     0x7ff023958e0e - std::panicking::default_hook::h852b4223c1c00c59
   4:     0x7ff0239594f8 - std::panicking::rust_panic_with_hook::hcd9d05f53fa0dafc
   5:     0x7ff023959392 - std::panicking::begin_panic::hf6c488cee66e7f17
   6:     0x7ff0239592d0 - std::panicking::begin_panic_fmt::hb0a7126ee57cdd27
   7:     0x7ff023959251 - rust_begin_unwind
   8:     0x7ff0239a6a1f - core::panicking::panic_fmt::h9af671b78898cdba
   9:     0x7ff0239a694b - core::panicking::panic::h1a2d1a6b50eaa468
  10:     0x7ff0215c2fce - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::get_generics::hd1475c0f11fab8b0
  11:     0x7ff0215c9e73 - rustc_metadata::csearch::<impl rustc::middle::cstore::CrateStore<'tcx> for rustc_metadata::cstore::CStore>::item_generics::h55da27f9db4e8cc9
  12:     0x7ff020d9ab3f - rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::lookup_generics::h7979816f3698527d
  13:     0x7ff020da1cc3 - rustc::util::ppaux::parameterized::h0bc66e5dfbaa04f2
  14:     0x7ff0239a8c95 - core::fmt::write::he349f75ae1aca1a1
  15:     0x7ff0239a8990 - <core::fmt::Arguments<'a> as core::fmt::Display>::fmt::h0e666855968de910
  16:     0x7ff0239a8c95 - core::fmt::write::he349f75ae1aca1a1
  17:     0x7ff02393b8a5 - <std::io::stdio::Stderr as std::io::Write>::write_fmt::hea40a51831b7e1f3
  18:     0x7ff01f743781 - <log::DefaultLogger as log::Logger>::log::hfc368897069c3744
  19:     0x7ff01f744507 - log::log::h4dee4f0d4cd441b1
  20:     0x7ff0224fa796 - rustc_trans::base::trans_closure::h941de14309416d66
  21:     0x7ff022517e7f - rustc_trans::closure::trans_closure_body_via_mir::hc380e80fb7895dee
  22:     0x7ff02256b219 - rustc_trans::mir::rvalue::<impl rustc_trans::mir::MirContext<'bcx, 'tcx>>::trans_rvalue::h2a14de75e2fd4285
  23:     0x7ff022559749 - rustc_trans::mir::block::<impl rustc_trans::mir::MirContext<'bcx, 'tcx>>::trans_block::he67d3259f79e4177
  24:     0x7ff022557a28 - rustc_trans::mir::trans_mir::h2fb44ecb31cfdffa
  25:     0x7ff0224fa94c - rustc_trans::base::trans_closure::h941de14309416d66
  26:     0x7ff0225775b4 - rustc_trans::trans_item::TransItem::define::ha4a18b94a3d46bf3
  27:     0x7ff0224fdec4 - rustc_trans::base::trans_crate::h9b06de31ed8799d1
  28:     0x7ff023cdebbd - rustc_driver::driver::phase_4_translate_to_llvm::hc3883ea2c4750179
  29:     0x7ff023d19cca - rustc_driver::driver::compile_input::{{closure}}::h9162a2fa292aeb3f
  30:     0x7ff023d10ec3 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h1928c4704cfe9c61
  31:     0x7ff023cdc55d - rustc_driver::driver::phase_3_run_analysis_passes::he578df6b8805151c
  32:     0x7ff023cc8f59 - rustc_driver::driver::compile_input::h5b63ccd49eeeb98b
  33:     0x7ff023cf229a - rustc_driver::run_compiler::h98c7274e7cb1d11d
  34:     0x7ff023c2e0eb - std::panicking::try::do_call::h99ed0da044e497c3
  35:     0x7ff023963846 - __rust_maybe_catch_panic
  36:     0x7ff023c4d091 - <F as alloc::boxed::FnBox<A>>::call_box::hbdd5a14cd8e33b97
  37:     0x7ff023957860 - std::sys::thread::Thread::new::thread_start::h50b05608a499d2b2
  38:     0x7ff01bc286f9 - start_thread
  39:     0x7ff02361bb5c - clone
  40:                0x0 - <unknown>

error: Could not compile `rand`.

To learn more, run the command again with --verbose.
