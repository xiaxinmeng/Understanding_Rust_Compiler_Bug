
$ RUST_BACKTRACE=1 rustc error.rs --crate-type lib
<std macros>:6:1: 6:32 error: the trait `core::convert::From<PythonObjectDowncastError<'_>>` is not implemented for the type `PyErr<'_>` [E0277]
<std macros>:6 $ crate:: convert:: From:: from ( err ) ) } } )
               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 6:48 note: in expansion of try!
error.rs:58:12: 58:41 note: expansion site
<std macros>:6:1: 6:32 help: run `rustc --explain E0277` to see a detailed explanation
error.rs:82:22: 82:36 error: internal compiler error: cannot relate bound region: '_#6r <= ReEarlyBound(315, TypeSpace, 0, 'python)
error.rs:82         E::extract::<Self::Prepared>(&obj)                                      
                                 ^~~~~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/diagnostic.rs:176

stack backtrace:
   1:     0x7fbb207304fe - sys::backtrace::write::hf666ea5132cf7b53Zws
   2:     0x7fbb207387b5 - panicking::on_panic::h831316f04c76720d4lx
   3:     0x7fbb206f9a3e - rt::unwind::begin_unwind_inner::h2cf94599b7bd3df8H1w
   4:     0x7fbb1db1d7ac - rt::unwind::begin_unwind::h6329212554763340604
   5:     0x7fbb1db1d74b - diagnostic::SpanHandler::span_bug::h5af5760a7fae491eb9A
   6:     0x7fbb1e7ec6b2 - middle::infer::region_inference::RegionVarBindings<'a, 'tcx>::make_subregion::h04db0d55be1dde413Vx
   7:     0x7fbb1e7a9a0c - middle::infer::equate::Equate<'a, 'tcx>.TypeRelation<'a, 'tcx>::regions::hd979b4b26580e2f2USs
   8:     0x7fbb1e7ac8d5 - middle::infer::sub::Sub<'a, 'tcx>.TypeRelation<'a, 'tcx>::relate_with_variance::h4446034952128033161
   9:     0x7fbb1e7ac722 - iter::_&'a mut I.Iterator::next::h2609482268122873928
  10:     0x7fbb1e7abdce - middle::ty_relate::relate_substs::h9009371242841443843
  11:     0x7fbb1e7ab480 - middle::ty_relate::relate_item_substs::h10613823414329269412
  12:     0x7fbb1e7ab200 - middle::ty_relate::ty..TraitRef<'tcx>.Relate<'a, 'tcx>::relate::h7757131364328309241
  13:     0x7fbb1e808b5b - middle::infer::InferCtxt<'a, 'tcx>::sub_trait_refs::h8f39013586f6e956TPB
  14:     0x7fbb1e89a3f4 - middle::traits::select::SelectionContext<'cx, 'tcx>::match_impl::hc67e696eb9545d7byuW
  15:     0x7fbb1e89f6ee - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates_from_impls::closure.92164
  16:     0x7fbb1e89ee44 - middle::ty::TraitDef<'tcx>::for_each_relevant_impl::h12250870464500226399
  17:     0x7fbb1e89bbf0 - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates::h5fc2c8c9d01b49c2yfU
  18:     0x7fbb1e88ee2e - middle::traits::select::SelectionContext<'cx, 'tcx>::candidate_from_obligation::h20de267bd3f86ac54TT
  19:     0x7fbb1e76a58f - middle::traits::select::SelectionContext<'cx, 'tcx>::select::h5810505dcd1952f9fyT
  20:     0x7fbb1e8850cc - middle::traits::project::assemble_candidates_from_impls::h490678811ca30ed0XAS
  21:     0x7fbb1e881cef - middle::traits::project::opt_normalize_projection_type::h46208f7641ab277bv6R
  22:     0x7fbb1e87350f - middle::traits::project::normalize_projection_type::h5ba4514321bf3a83j5R
  23:     0x7fbb1e87300d - middle::traits::fulfill::FulfillmentContext<'tcx>::normalize_projection_type::h7d135b74bb6f9b5fSaR
  24:     0x7fbb1ff0bc12 - check::FnCtxt<'a, 'tcx>::normalize_associated_type::h85a12e550530b4e2PVo
  25:     0x7fbb1fee7459 - check::FnCtxt<'a, 'tcx>.AstConv<'tcx>::projected_ty_from_poly_trait_ref::hd6a5f269b842679fazo
  26:     0x7fbb1ff52309 - astconv::finish_resolving_def_to_ty::h9ad99f78bde11eb4jLv
  27:     0x7fbb1ff1179c - astconv::ast_ty_to_ty::hb97056c0a4f0eb997Mv
  28:     0x7fbb1fe88520 - check::instantiate_path::hf0baba98444853ccMrs
  29:     0x7fbb1ff4883b - check::check_expr_with_unifier::h2221075350936743126
  30:     0x7fbb1fee898d - check::callee::check_call::h091b7e0ee90152ff4Ql
  31:     0x7fbb1ff2ed9a - check::check_expr_with_unifier::h3918486800583074951
  32:     0x7fbb1ff04a3b - check::check_block_with_expected::hd1194d6ba43baad7g0r
  33:     0x7fbb1fee827a - check::check_fn::h00277a9d75327c0eOFn
  34:     0x7fbb1fefe838 - check::check_bare_fn::hfddc865292f08a28rvn
  35:     0x7fbb1ff0a32c - check::check_method_body::h30075ff1c310ad04i8n
  36:     0x7fbb1fefc953 - check::check_item_body::h3f2e3887100269894Vn
  37:     0x7fbb1fefe464 - check::check_item_types::he86a18cc64ef1db4Ysn
  38:     0x7fbb1ffbb66b - check_crate::h26b1f5c95f18291cpTC
  39:     0x7fbb20c967e9 - driver::phase_3_run_analysis_passes::closure.16389
  40:     0x7fbb20c9512b - middle::ty::ctxt<'tcx>::create_and_enter::h7731474059721291206
  41:     0x7fbb20c90101 - driver::phase_3_run_analysis_passes::h16775793763798904703
  42:     0x7fbb20c742e0 - driver::compile_input::h94a4cd7284b1abf6Tba
  43:     0x7fbb20d57f53 - run_compiler::h57cf503a33efb4e7A7b
  44:     0x7fbb20d559ce - boxed::F.FnBox<A>::call_box::h1072113940123396054
  45:     0x7fbb20d552f9 - rt::unwind::try::try_fn::h14039827517923182205
  46:     0x7fbb207b38c8 - rust_try_inner
  47:     0x7fbb207b38b5 - rust_try
  48:     0x7fbb20723977 - rt::unwind::try::inner_try::h54ab330eae71d9c1AXw
  49:     0x7fbb20d55518 - boxed::F.FnBox<A>::call_box::h2790679698620608799
  50:     0x7fbb207373f1 - sys::thread::Thread::new::thread_start::hfedf7e48a39f0895D6v
  51:     0x7fbb1a338554 - start_thread
  52:     0x7fbb20392f3c - __clone
  53:                0x0 - <unknown>

