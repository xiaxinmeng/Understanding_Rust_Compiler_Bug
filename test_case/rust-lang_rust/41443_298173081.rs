
#0  core::panicking::panic_fmt () at /checkout/src/libcore/panicking.rs:69
#1  0x00003fffb7cfe248 in core::slice::slice_index_order_fail () at /checkout/src/libcore/slice/mod.rs:678
#2  0x00003fffb70d1f18 in .rustc_metadata::index::_$LT$impl$u20$rustc_metadata..schema..LazySeq$LT$rustc_metadata..index..Index$GT$$GT$::lookup::h98cc9de9f25fcb8e ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#3  0x00003fffb70ee804 in .rustc_metadata::decoder::_$LT$impl$u20$rustc_metadata..cstore..CrateMetadata$GT$::entry::h2dcb1ac8f24a6d94 ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#4  0x00003fffb70ef1bc in .rustc_metadata::decoder::_$LT$impl$u20$rustc_metadata..cstore..CrateMetadata$GT$::get_variant::h8e6c02c1598ab18e ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#5  0x00003fffb70ef480 in .rustc_metadata::decoder::_$LT$impl$u20$rustc_metadata..cstore..CrateMetadata$GT$::get_adt_def::h522a31389da1ef4c ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#6  0x00003fffb711d864 in .rustc_metadata::cstore_impl::provide::adt_def::h9de3b2f35340a240 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#7  0x00003fffb6c9f69c in .rustc::ty::maps::_$LT$impl$u20$rustc..ty..maps..queries..adt_def$LT$$u27$tcx$GT$$GT$::try_get::hccb13d4030370060 ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#8  0x00003fffb6cbcacc in .rustc::ty::maps::TyCtxtAt::adt_def::h0fd6b5db2c4ae164 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#9  0x00003fffb6cbb0fc in .rustc::ty::maps::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$tcx$C$$u20$$u27$lcx$GT$$GT$::adt_def::hcf9928fca26a15cc ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#10 0x00003fffb70ecacc in ._$LT$rustc_metadata..decoder..DecodeContext$LT$$u27$a$C$$u20$$u27$tcx$GT$$u20$as$u20$serialize..serialize..SpecializedDecoder$LT$$RF$$u27$tcx$u20$rustc..ty..AdtDef$GT$$GT$::specialized_decode::h13dbc06777fc5fb3
    () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#11 0x00003fffb70eb0d0 in ._$LT$rustc_metadata..decoder..DecodeContext$LT$$u27$a$C$$u20$$u27$tcx$GT$$u20$as$u20$serialize..serialize..SpecializedDecoder$LT$$RF$$u27$tcx$u20$rustc..ty..TyS$LT$$u27$tcx$GT$$GT$$GT$::specialized_decode::hf545939632440f00 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#12 0x00003fffb70aeb6c in .serialize::serialize::Decoder::read_enum_variant_arg::hb425955af9c5980c () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#13 0x00003fffb70eb748 in ._$LT$rustc_metadata..decoder..DecodeContext$LT$$u27$a$C$$u20$$u27$tcx$GT$$u20$as$u20$serialize..serialize..SpecializedDecoder$LT$$RF$$u27$tcx$u20$rustc..ty..TyS$LT$$u27$tcx$GT$$GT$$GT$::specialized_decode::hf545939632440f00 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#14 0x00003fffb70eab18 in ._$LT$rustc_metadata..decoder..DecodeContext$LT$$u27$a$C$$u20$$u27$tcx$GT$$u20$as$u20$serialize..serialize..SpecializedDecoder$LT$$RF$$u27$tcx$u20$rustc..ty..TyS$LT$$u27$tcx$GT$$GT$$GT$::specialized_decode::hf545939632440f00 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#15 0x00003fffb70f1c08 in .rustc_metadata::decoder::_$LT$impl$u20$rustc_metadata..cstore..CrateMetadata$GT$::get_type::h8b681362772ff2a6 ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#16 0x00003fffb711c91c in .rustc_metadata::cstore_impl::provide::type_of::h2fa72f695a64be56 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_metadata-1c582c40937620b6.so
#17 0x00003fffb6c99f70 in .rustc::ty::maps::_$LT$impl$u20$rustc..ty..maps..queries..type_of$LT$$u27$tcx$GT$$GT$::try_get_with::h574417bcc18bda6a ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#18 0x00003fffb6cbac14 in .rustc::ty::maps::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$tcx$C$$u20$$u27$lcx$GT$$GT$::type_of::h9e74d1ddc647e515 ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#19 0x00003fffb750e1c8 in .rustc_typeck::check::FnCtxt::instantiate_value_path::h2e8b3982e4e5835e () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#20 0x00003fffb7504b94 in .rustc_typeck::check::FnCtxt::check_expr_kind::h62f9fa86055e5da9 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#21 0x00003fffb7503864 in .rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::hfb489b8183d9e21b ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#22 0x00003fffb7504690 in .rustc_typeck::check::FnCtxt::check_expr_kind::h62f9fa86055e5da9 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#23 0x00003fffb7503864 in .rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::hfb489b8183d9e21b ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#24 0x00003fffb750ca84 in .rustc_typeck::check::FnCtxt::check_decl_initializer::h08160464d5b163d8 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#25 0x00003fffb750cbc4 in .rustc_typeck::check::FnCtxt::check_decl_local::hb662a33c4fc28485 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#26 0x00003fffb750cea0 in .rustc_typeck::check::FnCtxt::check_stmt::h9d95ba21dd56b6fe () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#27 0x00003fffb750d858 in .rustc_typeck::check::FnCtxt::check_block_with_expected::_$u7b$$u7b$closure$u7d$$u7d$::he2d3af3fee8df64c ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#28 0x00003fffb750d2dc in .rustc_typeck::check::FnCtxt::check_block_with_expected::h7b61aaa2c43cbf74 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#29 0x00003fffb7504640 in .rustc_typeck::check::FnCtxt::check_expr_kind::h62f9fa86055e5da9 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#30 0x00003fffb7503864 in .rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::hfb489b8183d9e21b ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#31 0x00003fffb7500710 in .rustc_typeck::check::FnCtxt::check_return_expr::hfc45bcf017181f2b () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#32 0x00003fffb74f308c in .rustc_typeck::check::check_fn::hadae288080c9dba2 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#33 0x00003fffb74f1b14 in .rustc_typeck::check::typeck_tables_of::h47f169e6f876fdda () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#34 0x00003fffb6cae6e0 in .rustc::ty::maps::_$LT$impl$u20$rustc..ty..maps..queries..typeck_tables_of$LT$$u27$tcx$GT$$GT$::try_get::he76002af05691dcc ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#35 0x00003fffb6cbe1bc in .rustc::ty::maps::TyCtxtAt::typeck_tables_of::h5d3d4fac6d9d48a3 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#36 0x00003fffb6cbbb00 in .rustc::ty::maps::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$tcx$C$$u20$$u27$lcx$GT$$GT$::typeck_tables_of::hb7737ac1df5175f0 ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#37 0x00003fffb74f0a74 in .rustc_typeck::check::typeck_item_bodies::h9ad82d5ce3097452 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#38 0x00003fffb6cad930 in .rustc::ty::maps::_$LT$impl$u20$rustc..ty..maps..queries..typeck_item_bodies$LT$$u27$tcx$GT$$GT$::try_get::h8bea64802a8d8cb9 ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#39 0x00003fffb6cbe054 in .rustc::ty::maps::TyCtxtAt::typeck_item_bodies::h4e75c317234befde () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#40 0x00003fffb6cbbaac in .rustc::ty::maps::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$tcx$C$$u20$$u27$lcx$GT$$GT$::typeck_item_bodies::hddf0462ee9523eaa ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc-ce6284c9885418c9.so
#41 0x00003fffb7540a48 in .rustc_typeck::check_crate::h16f92e6a986fd98b () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-ad206c419aaafe68.so
#42 0x00003fffb7ec5f88 in .rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::ha83538a03579857c ()
   from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#43 0x00003fffb7ebbf40 in .rustc_driver::driver::phase_3_run_analysis_passes::hfea44285662197cc () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#44 0x00003fffb7e90024 in .rustc_driver::driver::compile_input::hc74994fea2a7b3d8 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#45 0x00003fffb7eebaac in .rustc_driver::run_compiler::hccc057d367b00a4b () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#46 0x00003fffb7dc3178 in .std::panicking::try::do_call::hbc2a3c31903904db () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#47 0x00003fffb7ce9bc4 in panic_unwind::__rust_maybe_catch_panic () at /checkout/src/libpanic_unwind/lib.rs:98
#48 0x00003fffb7e105f0 in ._$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h223b45f4646df348 () from /root/.rustup/toolchains/nightly-powerpc64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#49 0x00003fffb7cdbfe8 in alloc::boxed::{{impl}}::call_once<(),()> () at /checkout/src/liballoc/boxed.rs:658
#50 std::sys_common::thread::start_thread () at /checkout/src/libstd/sys_common/thread.rs:21
#51 std::sys::imp::thread::{{impl}}::new::thread_start () at /checkout/src/libstd/sys/unix/thread.rs:84
#52 0x00003fffb1eabe64 in .start_thread () from /lib64/power8/libpthread.so.0
#53 0x00003fffb7b31534 in .__clone () from /lib64/power8/libc.so.6
