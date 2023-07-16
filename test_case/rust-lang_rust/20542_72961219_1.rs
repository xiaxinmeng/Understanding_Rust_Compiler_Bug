
ERROR:rbml::reader: failed to find block with tag 7
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librbml/lib.rs:266

stack backtrace:
   1:     0x7f2fcf698e80 - sys::backtrace::write::h84a1347ec8a3d425Aru
   2:     0x7f2fcf6bc4a0 - failure::on_fail::h1cb2299eb5e255ae9GB
   3:     0x7f2fcf6272d0 - rt::unwind::begin_unwind_inner::ha34a1c7f0630f466IlB
   4:     0x7f2fc8949570 - rt::unwind::begin_unwind::h11618834637780193760
   5:     0x7f2fc8948300 - reader::get_doc::hef77d659484c11a8nLa
   6:     0x7f2fcd950680 - metadata::decoder::item_type::h10f15a8b3984e7dcImi
   7:     0x7f2fcd961580 - metadata::decoder::get_type::h6f32069ab233c7c2jzi
   8:     0x7f2fcd7b8980 - middle::ty::lookup_item_type::h1544033676167d0cHM7
   9:     0x7f2fcee19b90 - check::FnCtxt<'a, 'tcx>.AstConv<'tcx>::get_item_type_scheme::h8a97d062f4aba1bftFn
  10:     0x7f2fceea80f0 - astconv::ast_path_to_ty::h2afb38f1cee6231cCxu
  11:     0x7f2fceea95e0 - astconv::ast_ty_to_ty::closure.33343
  12:     0x7f2fcee44b80 - astconv::ast_ty_to_ty::h848838ea974461b5AYu
  13:     0x7f2fceea17a0 - vec::Vec<T>.FromIterator<T>::from_iter::h13927799770275079786
  14:     0x7f2fcee9f3d0 - astconv::convert_angle_bracketed_parameters::h212169f719b8570cA6t
  15:     0x7f2fcee445f0 - astconv::ast_path_substs_for_ty::h21ba53d4a8da23ffyVt
  16:     0x7f2fceea80f0 - astconv::ast_path_to_ty::h2afb38f1cee6231cCxu
  17:     0x7f2fceea95e0 - astconv::ast_ty_to_ty::closure.33343
  18:     0x7f2fcee44b80 - astconv::ast_ty_to_ty::h848838ea974461b5AYu
  19:     0x7f2fcee37cd0 - check::GatherLocalsVisitor<'a, 'tcx>.Visitor<'tcx>::visit_local::h18f4f372c2aa7e2adEm
  20:     0x7f2fcee1a090 - check::check_fn::h342cf6090c0fe3a6bLm
  21:     0x7f2fcee36680 - check::check_bare_fn::h306f8ecc226858bctAm
  22:     0x7f2fcee2de70 - check::check_item::h7d555ba822408851KTm
  23:     0x7f2fceefcd10 - check_crate::closure.34569
  24:     0x7f2fceef7850 - check_crate::h2469153404b647bauEA
  25:     0x7f2fcfc29710 - driver::phase_3_run_analysis_passes::h198ed79d6101094dtGa
  26:     0x7f2fcfc102b0 - driver::compile_input::h08caf95be513bd6bBba
  27:     0x7f2fcfcdadd0 - run_compiler::hf1a36c26381910e09ac
  28:     0x7f2fcfcd9460 - thunk::F.Invoke<A, R>::invoke::h6996223324138754953
  29:     0x7f2fcfcd8390 - rt::unwind::try::try_fn::h16059966258548481805
  30:     0x7f2fcf727910 - rust_try_inner
  31:     0x7f2fcf727900 - rust_try
  32:     0x7f2fcfcd8640 - thunk::F.Invoke<A, R>::invoke::h12998514134271589737
  33:     0x7f2fcf6a8ca0 - sys::thread::thread_start::h756cedb2df1b4200Pnx
  34:     0x7f2fc98c8250 - start_thread
  35:     0x7f2fcf2b9219 - clone
  36:                0x0 - <unknown>
