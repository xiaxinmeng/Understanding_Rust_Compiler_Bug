
thread 'rustc' panicked at 'Box<Any>', /home/viktor/Documents/rust/rust/src/libsyntax/diagnostic.rs:191

stack backtrace:
1:      0x7fc4fb7689f8 - sys::backtrace::write::ha29d830caaf281c1kHC
2:      0x7fc4fb793f17 - panicking::on_panic::hd4b7cfa54ce15a47uUI
3:      0x7fc4fb6cba63 - rt::unwind::begin_unwind_inner::ha80e7d5af5095813CzI
4:      0x7fc4f8a6521d - rt::unwind::begin_unwind::h14288175298002166887
5:      0x7fc4f8a65972 - diagnostic::Handler::bug::hfe9a62f5ab542276osB
6:      0x7fc4f96a8ca1 - middle::traits::select::SelectionContext<'cx, 'tcx>::rematch_impl::h888ed2605974267aWuT
7:      0x7fc4f96a851f - middle::infer::InferCtxt<'a, 'tcx>::commit_if_ok::h8758431782731900545
8:      0x7fc4f96892ef - middle::traits::select::SelectionContext<'cx, 'tcx>::confirm_candidate::h680de60fb1bd61adSMS
9:      0x7fc4f965c86a - middle::traits::select::SelectionContext<'cx, 'tcx>::select::h105c655576bbde6e9NQ
10:     0x7fc4f966c33a - middle::traits::project::project_type::hf3dcf4dd73f443504tP
11:     0x7fc4f9669d71 - middle::traits::project::opt_normalize_projection_type::h72ea40d2d30b32b8GmP
12:     0x7fc4f9663e1a - middle::traits::project::poly_project_and_unify_type::closure.83715
13:     0x7fc4f9661ccc - middle::traits::project::poly_project_and_unify_type::h07927f85d5854b03P2O
14:     0x7fc4f965893b - middle::traits::fulfill::FulfillmentContext<'tcx>::select::h841d140980122122YEO
15:     0x7fc4f9657f2b - middle::traits::fulfill::FulfillmentContext<'tcx>::select_where_possible::h486388f4f6bf28bccEO
16:     0x7fc4fa3bdcb8 - check::vtable::select_fcx_obligations_where_possible::h9b2c69b3fe91faa2h4b
17:     0x7fc4fa3f622a - check::FnCtxt<'a, 'tcx>::resolve_type_vars_if_possible::h075b04892164c50b5Wo
18:     0x7fc4fa3a14b7 - check::structurally_resolved_type::h764154d1245e2f783xt
19:     0x7fc4fa4d0f9f - check::check_expr_with_unifier::h15570588596475525203
20:     0x7fc4fa47010a - check::op::check_binop::h97ec7d0b9e582ed693m
21:     0x7fc4fa4c7856 - check::check_expr_with_unifier::h11880991079361711379
22:     0x7fc4fa4c886e - check::check_expr_with_unifier::h11880991079361711379
23:     0x7fc4fa4af7a3 - check::check_expr_with_unifier::h13671224543324200731
24:     0x7fc4fa4990e8 - check::check_expr_with_unifier::check_then_else::hc4434b21966ece39ANq
25:     0x7fc4fa4c11f4 - check::check_expr_with_unifier::h1254672801148009773
26:     0x7fc4fa47e3f1 - check::check_block_with_expected::hf352b795571d241fUns
27:     0x7fc4fa4c13b3 - check::check_expr_with_unifier::h1254672801148009773
28:     0x7fc4fa3ad3de - check::_match::check_match::closure.28881
29:     0x7fc4fa3ad112 - check::_match::check_match::h2624ec058d33fc6aZmb
30:     0x7fc4fa4c1357 - check::check_expr_with_unifier::h1254672801148009773
31:     0x7fc4fa47e3f1 - check::check_block_with_expected::hf352b795571d241fUns
32:     0x7fc4fa4ce903 - check::check_expr_with_unifier::h15570588596475525203
33:     0x7fc4fa47e123 - check::check_block_with_expected::hf352b795571d241fUns
34:     0x7fc4fa45b969 - check::check_fn::haaf9eb020ec080d90Zn
35:     0x7fc4fa47905c - check::check_bare_fn::he93355dfc3534a69zPn
36:     0x7fc4fa472f24 - check::check_item::he0e731c9541337aak8n
37:     0x7fc4fa477662 - visit::walk_item::h16976416926307002031
38:     0x7fc4fa47766d - visit::walk_item::h16976416926307002031
39:     0x7fc4fa54c991 - check_crate::closure.36438
40:     0x7fc4fa546ed3 - check_crate::h60772f272ea0d698asC
41:     0x7fc4fbdea80d - driver::phase_3_run_analysis_passes::h71b6dad7a437d3fcgGa
42:     0x7fc4fbdce665 - driver::compile_input::h617a4fae1fdc397dQba
43:     0x7fc4fbe85c65 - run_compiler::h85ca2893f7ce8fe6S4b
44:     0x7fc4fbe8357d - boxed::F.FnBox<A>::call_box::h8037565825972556017
45:     0x7fc4fbe82ab9 - rt::unwind::try::try_fn::h15794509059050188955
46:     0x7fc4fb8109b8 - rust_try_inner
47:     0x7fc4fb8109a5 - rust_try
48:     0x7fc4fbe82d8b - boxed::F.FnBox<A>::call_box::h677285017824924912
49:     0x7fc4fb77ef01 - sys::thread::create::thread_start::he70b0e4e639cf402MuH
50:     0x7fc4f57cd0a3 - start_thread
51:     0x7fc4fb32d06c - clone
52:                0x0 - <unknown>
