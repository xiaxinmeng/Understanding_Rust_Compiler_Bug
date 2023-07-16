
[abeingessner@localhost test]$ cargo build
   Compiling test v0.0.1 (file:///home/abeingessner/dev/test)
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'assertion failed: did.krate != ast::LOCAL_CRATE', /home/abeingessner/dev/rust/src/librustc/middle/ty.rs:5461

stack backtrace:
   1:     0x7f10e24ddf70 - sys::backtrace::write::h8fe124aa62d4666cKlC
   2:     0x7f10e2506820 - panicking::on_panic::hbe49ffcf4865682ftXL
   3:     0x7f10e2447480 - rt::unwind::begin_unwind_inner::h76247a696cedaf77RBL
   4:     0x7f10e02c98f0 - rt::unwind::begin_unwind::h18381420555743533883
   5:     0x7f10e060ab00 - middle::ty::lookup_trait_def::h681d8657b73814a8Nla
   6:     0x7f10e062ff60 - middle::ty::predicates_for_trait_ref::h573132b5faa99585Vna
   7:     0x7f10e0629ce0 - middle::traits::util::Elaborator<'cx, 'tcx>.Iterator::next::h0e03729c4060de94QkV
   8:     0x7f10e062f960 - middle::traits::util::Supertraits<'cx, 'tcx>.Iterator::next::hfc2d17e5bc9957barnV
   9:     0x7f10e1c93450 - astconv::ast_ty_to_ty::closure.34582
  10:     0x7f10e1c30490 - astconv::ast_ty_to_ty::h7bd9623e042b5e3eNcw
  11:     0x7f10e1c8b930 - vec::Vec<T>.FromIterator<T>::from_iter::h182839420622209455
  12:     0x7f10e1c88800 - astconv::convert_angle_bracketed_parameters::h9bc668b404bddf99biv
  13:     0x7f10e1c8d020 - astconv::ast_path_to_trait_ref::h877deb0e17991561Vvv
  14:     0x7f10e1c3c230 - astconv::instantiate_trait_ref::h12efe6a871d055d0Csv
  15:     0x7f10e1c8ca50 - astconv::instantiate_poly_trait_ref::hdf3227e7a74ba7561qv
  16:     0x7f10e1cbb700 - collect::compute_bounds::ha18f23051e2a6717cTy
  17:     0x7f10e1c9cd80 - collect::trait_def_of_item::hb7f0f767c7cffe73Q2x
  18:     0x7f10e1c9c310 - collect::CollectTraitDefVisitor<'a, 'tcx>.visit..Visitor<'v>::visit_item::hbcfae1845ee115ebS8w
  19:     0x7f10e1ce6a00 - check_crate::closure.35776
  20:     0x7f10e1ce4a10 - check_crate::hc6d19f098edbe89aVjC
  21:     0x7f10e2b60bb0 - driver::phase_3_run_analysis_passes::hb4dd797330c68d2cgHa
  22:     0x7f10e2b46ef0 - driver::compile_input::he482374663bf0bd3Gba
  23:     0x7f10e2c15eb0 - run_compiler::h734ffa62c7eabdf8Zbc
  24:     0x7f10e2c147b0 - thunk::F.Invoke<A, R>::invoke::h15492659192422867035
  25:     0x7f10e2c136a0 - rt::unwind::try::try_fn::h15963018374249765158
  26:     0x7f10e257c570 - rust_try_inner
  27:     0x7f10e257c560 - rust_try
  28:     0x7f10e2c139a0 - thunk::F.Invoke<A, R>::invoke::h18067408346278163587
  29:     0x7f10e24f28c0 - sys::thread::thread_start::h45e232408687fc4093G
  30:     0x7f10dc53c460 - start_thread
  31:     0x7f10e20cd769 - clone
  32:                0x0 - <unknown>

Could not compile `test`.
