
#0  0x00007ffff478fc02 in _$LT$rustc_data_structures..obligation_forest..ObligationForest$LT$O$GT$$GT$::process_obligations::h0ff99cdf3dcc3ab2 () from /home/shum/.local/bin/../lib/../lib/librustc-579b052cb6742dd6.so
#1  0x00007ffff48edcf0 in rustc::traits::fulfill::FulfillmentContext::select_where_possible::hd82f7b0c63e2b550 () from /home/shum/.local/bin/../lib/../lib/librustc-579b052cb6742dd6.so
#2  0x00007ffff4f9097a in rustc_typeck::check::FnCtxt::select_obligations_where_possible::hac13e60e80dbcba0 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#3  0x00007ffff4f8b6bb in rustc_typeck::check::FnCtxt::resolve_type_vars_with_obligations::h2178d7ff44c1db76 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#4  0x00007ffff4f3ad28 in rustc_typeck::check::coercion::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::try_coerce::hd1aefddffb2298ea () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#5  0x00007ffff4f3c8de in rustc_typeck::check::demand::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::demand_coerce::hc4a73b302852d680 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#6  0x00007ffff4ed3e9e in core::ops::impls::_$LT$impl$u20$core..ops..FnOnce$LT$A$GT$$u20$for$u20$$RF$$u27$a$u20$mut$u20$F$GT$::call_once::h4cb23ef6f4635259 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#7  0x00007ffff4eba56d in _$LT$rustc_data_structures..accumulate_vec..AccumulateVec$LT$A$GT$$u20$as$u20$core..iter..traits..FromIterator$LT$$LT$A$u20$as$u20$rustc_data_structures..array_vec..Array$GT$..Element$GT$$GT$::from_iter::h29dc4c517cc097a4 ()
   from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#8  0x00007ffff4f12ca8 in _$LT$T$u20$as$u20$rustc..ty..context..InternIteratorElement$LT$T$C$$u20$R$GT$$GT$::intern_with::h6ca07eab6913ecc4 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#9  0x00007ffff4f9f42f in rustc_typeck::check::FnCtxt::check_expr_kind::hadde62228837913a () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#10 0x00007ffff4f9cd0e in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h64489ddc97d36346 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#11 0x00007ffff4ed3e8d in core::ops::impls::_$LT$impl$u20$core..ops..FnOnce$LT$A$GT$$u20$for$u20$$RF$$u27$a$u20$mut$u20$F$GT$::call_once::h4cb23ef6f4635259 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#12 0x00007ffff4eba56d in _$LT$rustc_data_structures..accumulate_vec..AccumulateVec$LT$A$GT$$u20$as$u20$core..iter..traits..FromIterator$LT$$LT$A$u20$as$u20$rustc_data_structures..array_vec..Array$GT$..Element$GT$$GT$::from_iter::h29dc4c517cc097a4 ()
   from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#13 0x00007ffff4f12ca8 in _$LT$T$u20$as$u20$rustc..ty..context..InternIteratorElement$LT$T$C$$u20$R$GT$$GT$::intern_with::h6ca07eab6913ecc4 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#14 0x00007ffff4f9f42f in rustc_typeck::check::FnCtxt::check_expr_kind::hadde62228837913a () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#15 0x00007ffff4f9cd0e in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h64489ddc97d36346 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#16 0x00007ffff4fa1f44 in rustc_typeck::check::FnCtxt::check_expr_kind::hadde62228837913a () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#17 0x00007ffff4f9cd0e in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h64489ddc97d36346 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#18 0x00007ffff4fa09e5 in rustc_typeck::check::FnCtxt::check_expr_kind::hadde62228837913a () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#19 0x00007ffff4f9cd0e in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h64489ddc97d36346 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#20 0x00007ffff4f91df7 in rustc_typeck::check::FnCtxt::check_argument_types::h985e4ddeee75a37e () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#21 0x00007ffff4f6b9f7 in rustc_typeck::check::callee::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::confirm_builtin_call::h2cd5aff1bed1fea3 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#22 0x00007ffff4f6a8ae in rustc_typeck::check::callee::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::check_call::he10eb01456e5f5af () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#23 0x00007ffff4f9dab8 in rustc_typeck::check::FnCtxt::check_expr_kind::hadde62228837913a () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#24 0x00007ffff4f9cd0e in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h64489ddc97d36346 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#25 0x00007ffff4f9a0e1 in rustc_typeck::check::FnCtxt::check_expr_struct_fields::h931680d8935d7d17 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#26 0x00007ffff4fa00c1 in rustc_typeck::check::FnCtxt::check_expr_kind::hadde62228837913a () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#27 0x00007ffff4f9cd0e in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h64489ddc97d36346 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#28 0x00007ffff4f896df in rustc_typeck::check::check_const_with_type::hcaf874c983aa4780 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#29 0x00007ffff4f8371d in rustc_typeck::check::check_item_type::h46851e4b2a32991a () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#30 0x00007ffff4f7c6ed in _$LT$rustc_typeck..check..CheckItemTypesVisitor$LT$$u27$a$C$$u20$$u27$tcx$GT$$u20$as$u20$rustc..hir..intravisit..Visitor$LT$$u27$tcx$GT$$GT$::visit_item::h224c1bbad7d277a3 ()
   from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#31 0x00007ffff4f7ea82 in rustc_typeck::check::check_item_types::h2c121166792403e4 () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#32 0x00007ffff4feb8ef in rustc_typeck::check_crate::hcb32a063b7119bae () from /home/shum/.local/bin/../lib/../lib/librustc_typeck-46efec171d67cc9c.so
#33 0x00007ffff7b460a5 in rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h3312aec6762531cd () from /home/shum/.local/bin/../lib/librustc_driver-fba78511ce6cdc51.so
#34 0x00007ffff7aaebf5 in rustc::ty::context::TyCtxt::create_and_enter::hda1e948f4c0f1e0f () from /home/shum/.local/bin/../lib/librustc_driver-fba78511ce6cdc51.so
#35 0x00007ffff7b24044 in rustc_driver::driver::compile_input::h2990bdc0fc07a424 () from /home/shum/.local/bin/../lib/librustc_driver-fba78511ce6cdc51.so
#36 0x00007ffff7b6b60e in rustc_driver::run_compiler::h5bf788ee0bf13463 () from /home/shum/.local/bin/../lib/librustc_driver-fba78511ce6cdc51.so
#37 0x00007ffff7a759ac in std::panicking::try::do_call::hf0803e51e3e750e8 () from /home/shum/.local/bin/../lib/librustc_driver-fba78511ce6cdc51.so
#38 0x00007ffff77a6207 in __rust_maybe_catch_panic () from /home/shum/.local/bin/../lib/libstd-413b1fd81b83156f.so
#39 0x00007ffff7a9dd73 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hcc35ddf0d1c1f13d () from /home/shum/.local/bin/../lib/librustc_driver-fba78511ce6cdc51.so
#40 0x00007ffff779be31 in std::sys::imp::thread::Thread::new::thread_start::h1f6ee946a79c10d0 () from /home/shum/.local/bin/../lib/libstd-413b1fd81b83156f.so
#41 0x00007ffff042d1f4 in start_thread () from /nix/store/amjgskg17wv125v9kahqdfxh8sx6mxgp-glibc-2.24/lib/libpthread.so.0
#42 0x00007ffff745d12f in clone () from /nix/store/amjgskg17wv125v9kahqdfxh8sx6mxgp-glibc-2.24/lib/libc.so.6
