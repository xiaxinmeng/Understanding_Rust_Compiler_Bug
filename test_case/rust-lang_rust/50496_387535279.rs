
#0  __memmove_avx_unaligned () at ../sysdeps/x86_64/multiarch/memcpy-avx-unaligned.S:325
#1  0x00007ffff49a23b3 in rustc::mir::BasicBlockData::expand_statements::hf1d4ebe84ecd6e03 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc_mir-6950d58feb41fa53.so
#2  0x00007ffff48b9bb5 in _$LT$rustc_mir..transform..deaggregator..Deaggregator$u20$as$u20$rustc_mir..transform..MirPass$GT$::run_pass::hbb873b9298ecbcc7 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc_mir-6950d58feb41fa53.so
#3  0x00007ffff4900f48 in rustc_mir::transform::optimized_mir::_$u7b$$u7b$closure$u7d$$u7d$::h31fd21092d0543e7 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc_mir-6950d58feb41fa53.so
#4  0x00007ffff48ea8d1 in rustc_mir::transform::optimized_mir::h1131a13b2ae69a43 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc_mir-6950d58feb41fa53.so
#5  0x00007ffff3cdb059 in rustc::ty::maps::_$LT$impl$u20$rustc..ty..maps..config..QueryConfig$LT$$u27$tcx$GT$$u20$for$u20$rustc..ty..maps..queries..optimized_mir$LT$$u27$tcx$GT$$GT$::compute::hc1ffed4360780ff2 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#6  0x00007ffff37e2a31 in rustc::dep_graph::graph::DepGraph::with_task_impl::h3ce3074a9b38daa4 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#7  0x00007ffff39a83d7 in rustc::ty::context::tls::with_related_context::h2449d2461fb7e66f () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#8  0x00007ffff3a301a2 in rustc::ty::maps::plumbing::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::force_query_with_job::h6c5f6a6ee29b4c8c ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#9  0x00007ffff3a9fa43 in rustc::ty::maps::plumbing::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::get_query::h6b180b58d3e77599 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#10 0x00007ffff3b0d17b in rustc::ty::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::maybe_optimized_mir::hbb43c33ed1c2e50f ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#11 0x00007ffff492c71b in _$LT$rustc_mir..interpret..eval_context..EvalContext$LT$$u27$a$C$$u20$$u27$mir$C$$u20$$u27$tcx$C$$u20$M$GT$$GT$::load_mir::hdadbb8aaab5a0add ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc_mir-6950d58feb41fa53.so
#12 0x00007ffff4775200 in rustc_mir::interpret::const_eval::eval_body_and_ecx::hb4b25a904afc078e () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc_mir-6950d58feb41fa53.so
#13 0x00007ffff4779068 in rustc_mir::interpret::const_eval::const_eval_provider::he924d56230e7faca () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc_mir-6950d58feb41fa53.so
#14 0x00007ffff3cdbb57 in rustc::ty::maps::_$LT$impl$u20$rustc..ty..maps..config..QueryConfig$LT$$u27$tcx$GT$$u20$for$u20$rustc..ty..maps..queries..const_eval$LT$$u27$tcx$GT$$GT$::compute::h8881ebdb8a6b3fc9 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#15 0x00007ffff37f9789 in rustc::dep_graph::graph::DepGraph::with_task_impl::h970e6f3457ee9779 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#16 0x00007ffff39fd5cf in rustc::ty::context::tls::with_related_context::hf6aa86ba669caacf () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#17 0x00007ffff3a315d8 in rustc::ty::maps::plumbing::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::force_query_with_job::h767d969b981cd0e1 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#18 0x00007ffff3a898a6 in rustc::ty::maps::plumbing::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::get_query::h402310c1d566e53b ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#19 0x00007ffff3b0eacd in rustc::ty::maps::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$tcx$C$$u20$$u27$lcx$GT$$GT$::const_eval::h0dc1da25520f77c1 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#20 0x00007ffff4868556 in rustc_mir::monomorphize::collector::collect_items_rec::h62a919462e2120d0 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc_mir-6950d58feb41fa53.so
#21 0x00007ffff4867e82 in rustc_mir::monomorphize::collector::collect_crate_mono_items::h95367eff4f34f085 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc_mir-6950d58feb41fa53.so
#22 0x00007fffed1dd4ba in rustc::util::common::time::h2e056e8595b0c79b () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#23 0x00007fffed2a02c9 in rustc_trans::base::collect_and_partition_translation_items::h675aeb9c6f31a5ac ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#24 0x00007ffff37da225 in rustc::dep_graph::graph::DepGraph::with_task_impl::h2495eb20a211973b () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#25 0x00007ffff39e38fc in rustc::ty::context::tls::with_related_context::hb6f352ea70136898 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#26 0x00007ffff3a231f0 in rustc::ty::maps::plumbing::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::force_query_with_job::h20d94168c3140725 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#27 0x00007ffff3a8f472 in rustc::ty::maps::plumbing::_$LT$impl$u20$rustc..ty..context..TyCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::get_query::h47577a0ffde54cdb ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/../lib/librustc-6264c07cbba9680a.so
#28 0x00007fffed29e541 in rustc_trans::base::trans_crate::hee88ee89ffeda7af () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#29 0x00007fffed201440 in _$LT$rustc_trans..LlvmTransCrate$u20$as$u20$rustc_trans_utils..trans_crate..TransCrate$GT$::trans_crate::h800dcda488afb790 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#30 0x00007ffff7a2ed36 in rustc::util::common::time::hcf488eceeda0a9cc () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#31 0x00007ffff7a1ea4d in rustc_driver::driver::phase_4_translate_to_llvm::h13463ad7ae997e4b () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#32 0x00007ffff7ac3016 in rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::h31e9ff1f64156b74 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#33 0x00007ffff7ab6af5 in rustc::ty::context::tls::enter_context::hb86cbd2c401dfb18 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#34 0x00007ffff7a818be in _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::h19074a28b3607093 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#35 0x00007ffff79ceb5c in rustc::ty::context::TyCtxt::create_and_enter::h3499c83bc74e5346 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#36 0x00007ffff7a17a0b in rustc_driver::driver::compile_input::ha789cddfb4d7cf8b () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#37 0x00007ffff7ad5735 in rustc_driver::run_compiler_impl::he0bf21247a724f12 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#38 0x00007ffff7a36cf5 in _$LT$scoped_tls..ScopedKey$LT$T$GT$$GT$::set::h0a4a9cc70840e625 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#39 0x00007ffff79eae95 in syntax::with_globals::h426e57a0d566387f () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#40 0x00007ffff79dde2e in _$LT$std..panic..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h31900b1f5e62de36 ()
   from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#41 0x00007ffff766ff7a in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:105
#42 0x00007ffff7ad2814 in rustc_driver::run::hbcc80b82a59ea630 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#43 0x00007ffff7ae02bb in rustc_driver::main::hd5d5ac3338638be9 () from /home/kennytm/.rustup/toolchains/c1168be5360f17516b233be85ebb193bb4e612bf/bin/../lib/librustc_driver-faeae2bec58daae3.so
#44 0x0000555555554c13 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbdc018091d795d35 ()
#45 0x00007ffff7630073 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h0e173cc915a5091f () at libstd/rt.rs:59
#46 std::panicking::try::do_call::h40752f40c51cf13d () at libstd/panicking.rs:310
#47 0x00007ffff766ff7a in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:105
#48 0x00007ffff7645cf6 in std::panicking::try::h431a7a5b8edb0415 () at libstd/panicking.rs:289
#49 std::panic::catch_unwind::h1eb313d318d299fa () at libstd/panic.rs:374
#50 std::rt::lang_start_internal::h83a5cbcfc3241381 () at libstd/rt.rs:58
#51 0x0000555555554c74 in main ()
