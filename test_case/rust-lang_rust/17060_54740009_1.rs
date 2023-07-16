
scratch$ RUST_BACKTRACE=1 rustc 17060.rs 
error: internal compiler error: fictitious type T in sizing_type_of()
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/nathan/m/prj/rust/rust/src/libsyntax/ast_util.rs:776

stack backtrace:
   1:     0x7fa99d669e10 - rt::backtrace::imp::write::h51aa68d0663f3b61QLq
   2:     0x7fa99d66cf90 - failure::on_fail::h297efee10e7c3dcby7q
   3:     0x7fa9a1c0c290 - unwind::begin_unwind_inner::h41358ca950f0d61262d
   4:     0x7fa99e404190 - unwind::begin_unwind::h15398001123804529202
   5:     0x7fa99e404820 - diagnostic::Handler::bug::hdef46b4ea651e464ufF
   6:     0x7fa9a2214200 - driver::session::Session::bug::h02a5f48a9d9f9eb1uKC
   7:     0x7fa9a25450b0 - middle::trans::type_of::sizing_type_of::ha11747c57b42fdbfmk9
   8:     0x7fa9a262c640 - middle::trans::adt::mk_struct::h79b77082e9c2eac9tZt
   9:     0x7fa9a253e330 - middle::trans::adt::represent_type::hf72895e4426bae54Kxt
  10:     0x7fa9a25450b0 - middle::trans::type_of::sizing_type_of::ha11747c57b42fdbfmk9
  11:     0x7fa9a2553ef0 - middle::trans::common::type_is_immediate::h3c54d101f2bd0772tU5
  12:     0x7fa9a25a8cd0 - middle::trans::type_of::type_of_rust_fn::h12a37148d1de81c8Xg9
  13:     0x7fa9a25b9260 - middle::trans::base::decl_rust_fn::h12564314a66abf06oGc
  14:     0x7fa9a255f530 - middle::trans::closure::get_or_create_declaration_if_unboxed_closure::hd043bd5c8f2aeb2640i
  15:     0x7fa9a257e770 - middle::trans::closure::trans_unboxed_closure::h8a471658ddbc27c9v6i
  16:     0x7fa9a256b4a0 - middle::trans::expr::trans_rvalue_dps_unadjusted::hbdb735d1cb49bf56PY3
  17:     0x7fa9a2533e70 - middle::trans::expr::trans_into::h123a8cb28868f2addC2
  18:     0x7fa9a2574f00 - middle::trans::expr::trans_uniq_expr::h30b88a9c8d284f8fcL4
  19:     0x7fa9a2575830 - middle::trans::expr::trans_unary::h6c1ede6e229c70fa7G4
  20:     0x7fa9a256a850 - middle::trans::expr::trans_unadjusted::h171ecf2b051fc47dnp3
  21:     0x7fa9a2534de0 - middle::trans::expr::trans::hb43b26a8c8c973dc9F2
  22:     0x7fa9a2533e70 - middle::trans::expr::trans_into::h123a8cb28868f2addC2
  23:     0x7fa9a257ccc0 - middle::trans::expr::trans_adt::hb19dfc5187fcff3azy4
  24:     0x7fa9a257ffc0 - middle::trans::expr::trans_struct::closure.115770
  25:     0x7fa9a256b4a0 - middle::trans::expr::trans_rvalue_dps_unadjusted::hbdb735d1cb49bf56PY3
  26:     0x7fa9a2533e70 - middle::trans::expr::trans_into::h123a8cb28868f2addC2
  27:     0x7fa9a2534130 - middle::trans::controlflow::trans_block::h3153dd2213ad6bf4esY
  28:     0x7fa9a25cbda0 - middle::trans::base::trans_closure::hfafe5698ece5431ccIe
  29:     0x7fa9a252a740 - middle::trans::base::trans_fn::h323ef7584c829916ZTe
  30:     0x7fa9a252ac60 - middle::trans::monomorphize::monomorphic_fn::h318dd6b9156fceeabPX
  31:     0x7fa9a2557fc0 - middle::trans::callee::trans_fn_ref_with_vtables::h60202876a6fead6aQm1
  32:     0x7fa9a2556dd0 - middle::trans::callee::trans_fn_ref::h69bad953cd0bdaa7Y80
  33:     0x7fa9a2560080 - middle::trans::callee::trans_call::closure.115515
  34:     0x7fa9a253b740 - middle::trans::callee::trans_call_inner::hcef07a1a5f7c3620FN1
  35:     0x7fa9a255fe20 - middle::trans::callee::trans_call::h464b09bb44d97480KH1
  36:     0x7fa9a256b4a0 - middle::trans::expr::trans_rvalue_dps_unadjusted::hbdb735d1cb49bf56PY3
  37:     0x7fa9a256a850 - middle::trans::expr::trans_unadjusted::h171ecf2b051fc47dnp3
  38:     0x7fa9a2534de0 - middle::trans::expr::trans::hb43b26a8c8c973dc9F2
  39:     0x7fa9a2533770 - middle::trans::controlflow::trans_stmt_semi::h26cd24ef900b92a3krY
  40:     0x7fa9a2532fa0 - middle::trans::controlflow::trans_stmt::h94dc99c0f5a518693mY
  41:     0x7fa9a2534130 - middle::trans::controlflow::trans_block::h3153dd2213ad6bf4esY
  42:     0x7fa9a25cbda0 - middle::trans::base::trans_closure::hfafe5698ece5431ccIe
  43:     0x7fa9a252a740 - middle::trans::base::trans_fn::h323ef7584c829916ZTe
  44:     0x7fa9a2526950 - middle::trans::base::trans_item::h67be48931734de89Zbf
  45:     0x7fa9a25d5420 - middle::trans::base::trans_crate::h06bc312b46497235q6f
  46:     0x7fa9a2979960 - driver::driver::phase_4_translate_to_llvm::hc08ae48b078f56e24aC
  47:     0x7fa9a2971960 - driver::driver::compile_input::h08ecbcbe3a7bcaeacNB
  48:     0x7fa9a2a047f0 - driver::run_compiler::hfe0d6a906000c83cCCF
  49:     0x7fa9a2a04710 - driver::main_args::closure.136314
  50:     0x7fa9a2a16e60 - task::TaskBuilder<S>::try_future::closure.137484
  51:     0x7fa9a2a16c60 - task::TaskBuilder<S>::spawn_internal::closure.137461
  52:     0x7fa9a1f2b850 - task::spawn_opts::closure.8394
  53:     0x7fa9a1c33f50 - rust_try_inner
  54:     0x7fa9a1c33f40 - rust_try
  55:     0x7fa9a1c09a10 - unwind::try::h99dc0bfd91bbea66gRd
  56:     0x7fa9a1c09870 - task::Task::run::hcec539e97a50e30692c
  57:     0x7fa9a1f2b5c0 - task::spawn_opts::closure.8340
  58:     0x7fa9a1c0b330 - thread::thread_start::h1519d237f0e56dbfgod
  59:     0x7fa99ca09060 - start_thread
  60:     0x7fa9a18dd489 - __clone
  61:                0x0 - <unknown>

scratch$ rustc -v
rustc 0.12.0-pre (c95aa9950 2014-09-04 03:01:03 +0000)
