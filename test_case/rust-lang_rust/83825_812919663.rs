plain
.................................................................................................... 9400/11734
.................................................................................................... 9500/11734
...........................................................................i......i................. 9600/11734
.................................................................................................... 9700/11734
......................iiiiiii.iiiiii.i.............................................................. 9800/11734
.................................................................................................... 10000/11734
.................................................................................................... 10100/11734
.................................................................................................... 10200/11734
.................................................................................................... 10300/11734
---
failures:

---- [ui] ui/const-generics/issues/issue-64494.rs#full stdout ----

error in revision `full`: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-64494.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-64494.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-64494.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', /checkout/compiler/rustc_middle/src/ty/sty.rs:968:9
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (dee28142f 2021-04-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'substs of instance DefId(0:4 ~ issue_64494[317d]::Foo::VAL) not normalized for codegen: [^0]', compiler/rustc_middle/src/ty/instance.rs:285:9
stack backtrace:
   0:     0x7f1ad08deaa0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcfda7e4adc91aa9e
   1:     0x7f1ad0952a4c - core::fmt::write::h483ce1f2810e1bcd
   2:     0x7f1ad08d2986 - std::io::Write::write_fmt::h1bef5be356d3a70b
   3:     0x7f1ad08e2e77 - std::panicking::default_hook::{{closure}}::hee2e6e09bd3a7fc0
   4:     0x7f1ad08e2884 - std::panicking::default_hook::ha29955f411b2bc1a
   5:     0x7f1ad11afc5d - rustc_driver::report_ice::h46c588f3a20ef158
   6:     0x7f1ad08e37b2 - std::panicking::rust_panic_with_hook::h796d2956dd9fabb8
   7:     0x7f1ad08e32e7 - std::panicking::begin_panic_handler::{{closure}}::h174689cf7c2e93b7
   8:     0x7f1ad08def5c - std::sys_common::backtrace::__rust_end_short_backtrace::h88b4e7e0135f3798
   9:     0x7f1ad08e3249 - rust_begin_unwind
  10:     0x7f1ad08e31fb - std::panicking::begin_panic_fmt::h1fedc0ca3e470d82
  11:     0x7f1ad35d5da2 - rustc_middle::ty::instance::Instance::new::hbb7af5c7355e1c37
  12:     0x7f1ad223b0a0 - std::thread::local::LocalKey<T>::with::hc970e49ef7e654b6
  13:     0x7f1ad2111027 - rustc_query_impl::plumbing::<impl rustc_query_system::query::config::QueryDescription<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::resolve_instance>::describe::hbbc67ee44afe4821
  14:     0x7f1ad22625fe - rustc_query_impl::make_query::resolve_instance::h88793008ff9ccf42
  15:     0x7f1ad1fe5732 - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::hcbea749ea26e6cc2
  16:     0x7f1ad22eccea - <hashbrown::map::HashMap<K,V,S,A> as core::iter::traits::collect::Extend<(K,V)>>::extend::he21c17b30fd6554c
  17:     0x7f1ad1e57e7c - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h657ea34860288716
  18:     0x7f1ad211ddd2 - rustc_query_impl::Queries::try_collect_active_jobs::h469b0c6570bd24ac
  19:     0x7f1ad1f66081 - rustc_query_system::query::job::print_query_stack::hfbafca162e920412
  20:     0x7f1ad12d41f4 - rustc_interface::interface::try_print_query_stack::h5a586946203775ab
  21:     0x7f1ad11b04ba - rustc_driver::report_ice::h46c588f3a20ef158
  22:     0x7f1ad08e37b2 - std::panicking::rust_panic_with_hook::h796d2956dd9fabb8
  23:     0x7f1ad08e32b7 - std::panicking::begin_panic_handler::{{closure}}::h174689cf7c2e93b7
  24:     0x7f1ad08def5c - std::sys_common::backtrace::__rust_end_short_backtrace::h88b4e7e0135f3798
  25:     0x7f1ad08e3249 - rust_begin_unwind
  26:     0x7f1ad094f011 - core::panicking::panic_fmt::hb23983f3a1f37585
  27:     0x7f1ad094ef5d - core::panicking::panic::hc30c9fb9bf97767b
  28:     0x7f1ad1524842 - rustc_middle::ty::sty::Binder<T>::dummy::hf1060e5340406926
  29:     0x7f1ad1530b9a - rustc_ty_utils::instance::inner_resolve_instance::hbdf5ef0a6d7db162
  30:     0x7f1ad152fdf9 - rustc_ty_utils::instance::resolve_instance::h8475dc3b5c7dd04e
  31:     0x7f1ad1ebd6c3 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::resolve_instance>::compute::h6731fe9e59239dd1
  32:     0x7f1ad21ccd29 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h83f2ffb165510c53
  33:     0x7f1ad21f9c51 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::hf61dc6fc2d5cfd8c
  34:     0x7f1ad1f70429 - rustc_data_structures::stack::ensure_sufficient_stack::h3c44b42b56e52519
  35:     0x7f1ad1dfd948 - rustc_query_system::query::plumbing::force_query_with_job::hf252dc77004a2b57
  36:     0x7f1ad1d5e62c - rustc_query_system::query::plumbing::get_query_impl::h87fe3e7d71db5d32
  37:     0x7f1ad1e94e34 - rustc_query_system::query::plumbing::get_query::h4c47d56d15d4be61
  38:     0x7f1ad211f8ae - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance::h439fee9410dc31bd
  39:     0x7f1ad35e78c6 - rustc_middle::ty::instance::Instance::resolve_opt_const_arg::ha5c30e382a9c83c9
  40:     0x7f1ad3512e90 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::h56a4dbbf31abe224
  41:     0x7f1ad3194cbd - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const::h220c83eefb5b70a8
  42:     0x7f1ad321e475 - rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::super_fold_with::h02a32de59a69fa4c
  43:     0x7f1ad3194faf - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const::h2944f3c69bce0325
  44:     0x7f1ad1692557 - <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize::ha76090742bc08164
  45:     0x7f1ad15ae626 - rustc_infer::infer::InferCtxtBuilder::enter::h4d420b8826ae5f62
  46:     0x7f1ad1687916 - core::ops::function::FnOnce::call_once::h5cf541f603b303fa
  47:     0x7f1ad1ebd2f9 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::normalize_mir_const_after_erasing_regions>::compute::h987d130adbf2a380
  48:     0x7f1ad21d9225 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::haf2ed24ca8c3b613
  49:     0x7f1ad21f83d5 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h802dbb0fbe4e8f35
  50:     0x7f1ad1f691f7 - rustc_data_structures::stack::ensure_sufficient_stack::h1011fb37293d991b
  51:     0x7f1ad1de75ab - rustc_query_system::query::plumbing::force_query_with_job::h7ad8eab0a3e34403
  52:     0x7f1ad1d72038 - rustc_query_system::query::plumbing::get_query_impl::hb405fd00755cbf4c
  53:     0x7f1ad1eaf065 - rustc_query_system::query::plumbing::get_query::hd56a2a2ec1aace4d
  54:     0x7f1ad211f412 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions::hb883d6cdc4ef649d
  55:     0x7f1ad295a2f5 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions::hc39cc8f6e94d052f
  56:     0x7f1ad295b804 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::he0a453740c026574
  57:     0x7f1ad28341c2 - rustc_mir::interpret::eval_context::InterpCx<M>::push_stack_frame::habfbdb4322abdc43
  58:     0x7f1ad251e6f4 - rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider::hf91cdbbd6f0baf7b
  59:     0x7f1ad1ebc2e0 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::eval_to_allocation_raw>::compute::h01839278fc54c5d3
  60:     0x7f1ad21d20b9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h95c767e30b77fe65
  61:     0x7f1ad21f92cc - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::hd4ed9928c2d332ed
  62:     0x7f1ad1f907eb - rustc_data_structures::stack::ensure_sufficient_stack::he7497600a5fb1eb6
  63:     0x7f1ad1df5e35 - rustc_query_system::query::plumbing::force_query_with_job::hcbba16283276d786
  64:     0x7f1ad1d926f7 - rustc_query_system::query::plumbing::get_query_impl::hf443b30f525c8a83
  65:     0x7f1ad1e86fdd - rustc_query_system::query::plumbing::get_query::h03befbac113aa1a4
  66:     0x7f1ad211e846 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw::h71bcce0431570995
  67:     0x7f1ad251ccb5 - rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider::hc6fb942e2aab85db
  68:     0x7f1ad1ebc320 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::eval_to_const_value_raw>::compute::h3eb9d7e06d6383fa
  69:     0x7f1ad21ca2f9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h743d36972a4f14b9
  70:     0x7f1ad21f85cc - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h87f9f584ebe47fb0
  71:     0x7f1ad1f8eb9f - rustc_data_structures::stack::ensure_sufficient_stack::hde76a24df774144d
  72:     0x7f1ad1dd2347 - rustc_query_system::query::plumbing::force_query_with_job::h00c62fe29293c070
  73:     0x7f1ad1d7a139 - rustc_query_system::query::plumbing::get_query_impl::hc4f03b2c738e4e27
  74:     0x7f1ad1e9e4b3 - rustc_query_system::query::plumbing::get_query::h7f6a0e6f8336aa21
  75:     0x7f1ad211e8a6 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw::h3f6e8f2d06110b5b
  76:     0x7f1ad34eaf4e - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id::hb76bee44b52f4882
  77:     0x7f1ad351313e - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::h56a4dbbf31abe224
  78:     0x7f1ad3357696 - rustc_infer::infer::InferCtxt::const_eval_resolve::h9d1d6b5f2f8f24e8
  79:     0x7f1ad3250901 - rustc_data_structures::stack::ensure_sufficient_stack::h8eb5f30272ad6e40
  80:     0x7f1ad31fdd23 - rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively::h95eb6879f7cd0ab8
  81:     0x7f1ad31e5cd4 - rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively::h1d51aed0a3c9c2fb
  82:     0x7f1ad3125c24 - rustc_infer::infer::InferCtxt::probe::h2f565eccd4215fe4
  83:     0x7f1ad31fe313 - rustc_trait_selection::traits::select::SelectionContext::evaluate_candidate::hb23960f84f76960b
  84:     0x7f1ad31ea741 - rustc_trait_selection::traits::select::SelectionContext::evaluate_stack::hb2f5a19ec9fb31a0
  85:     0x7f1ad31c7b06 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_anon_task::h5ac8b5067e741106
  86:     0x7f1ad31e6b3d - rustc_trait_selection::traits::select::SelectionContext::evaluate_trait_predicate_recursively::h166a800eb36346e2
  87:     0x7f1ad3250dac - rustc_data_structures::stack::ensure_sufficient_stack::h8eb5f30272ad6e40
  88:     0x7f1ad31fdd23 - rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively::h95eb6879f7cd0ab8
  89:     0x7f1ad3126bdc - rustc_infer::infer::InferCtxt::probe::h62bae0407521bc98
  90:     0x7f1ad31e58f7 - rustc_trait_selection::traits::select::SelectionContext::predicate_may_hold_fatal::hf5d2b26c87bff9ff
  91:     0x7f1ad31c9109 - core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut::h737a05988baba6ef
  92:     0x7f1ad3184015 - <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold::h7fbc033b130b3d81
  93:     0x7f1ad321667b - <core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold::h02bc142b3f223296
  94:     0x7f1ad31c3729 - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold::hb5eb63270916ee2a
  95:     0x7f1ad3284ae1 - rustc_trait_selection::traits::coherence::overlap_within_probe::h43bb7c69298e7179
  96:     0x7f1ad3124715 - rustc_infer::infer::InferCtxt::probe_maybe_skip_leak_check::h35f5f94fe76cd4ae
  97:     0x7f1ad328464e - rustc_trait_selection::traits::coherence::overlap::hc0d78cc0f94ed8f8
  98:     0x7f1ad3115146 - rustc_infer::infer::InferCtxtBuilder::enter::h0c08c65ac86a7f24
  99:     0x7f1ad32831c9 - rustc_trait_selection::traits::coherence::overlapping_impls::hb4ca7f3680631c8d
 100:     0x7f1ad326ec63 - <rustc_middle::traits::specialization_graph::Children as rustc_trait_selection::traits::specialize::specialization_graph::ChildrenExt>::insert::h80ed1fb982a8b5db
 101:     0x7f1ad3270638 - <rustc_middle::traits::specialization_graph::Graph as rustc_trait_selection::traits::specialize::specialization_graph::GraphExt>::insert::h893e50aebda59534
 102:     0x7f1ad328e680 - rustc_trait_selection::traits::specialize::specialization_graph_provider::h05d08e73b857906e
 103:     0x7f1ad1ebc80c - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::specialization_graph_of>::compute::h18af39fd85b53036
 104:     0x7f1ad21add15 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h0b57399698fcf53c
 105:     0x7f1ad1f81b1b - rustc_data_structures::stack::ensure_sufficient_stack::h9811d8dc1e718d76
 106:     0x7f1ad1ddb097 - rustc_query_system::query::plumbing::force_query_with_job::h3e1639c325a0fd5f
 107:     0x7f1ad1d46dc1 - rustc_query_system::query::plumbing::get_query_impl::h566799cb98520e3f
 108:     0x7f1ad1eadaac - rustc_query_system::query::plumbing::get_query::hceecdcea0bf75150
 109:     0x7f1ad192d953 - rustc_typeck::coherence::coherent_trait::hde73258d970cfa98
 110:     0x7f1ad21cf0a0 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h8e3c76fdd35ada43
 111:     0x7f1ad1f7f71f - rustc_data_structures::stack::ensure_sufficient_stack::h8b205e6015b87704
 112:     0x7f1ad1dea95d - rustc_query_system::query::plumbing::force_query_with_job::h8aeb66ad391d87c9
 113:     0x7f1ad1d8dfd9 - rustc_query_system::query::plumbing::get_query_impl::heba9121488e04cf6
 114:     0x7f1ad1e94b0c - rustc_query_system::query::plumbing::get_query::h48ed44d060c85390
 115:     0x7f1ad19305ca - rustc_typeck::coherence::check_coherence::h7849b3837b0c5c63
 116:     0x7f1ad193e50d - rustc_session::session::Session::track_errors::h73c8f984e41e01e9
 117:     0x7f1ad199c10e - rustc_typeck::check_crate::hec033a6d7b1d19eb
 118:     0x7f1ad12e613f - rustc_interface::passes::analysis::h1dc6077b3518aa43
 119:     0x7f1ad21c2acc - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h49e22dcabf426654
 120:     0x7f1ad21f2c79 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h76fbf85c00c152ad
 121:     0x7f1ad1f69446 - rustc_data_structures::stack::ensure_sufficient_stack::h10afa8e46caf008b
 122:     0x7f1ad1dfe897 - rustc_query_system::query::plumbing::force_query_with_job::hf7f42d8427e9a6d5
 123:     0x7f1ad1d4e5c7 - rustc_query_system::query::plumbing::get_query_impl::h69bbf6baab5add86
 124:     0x7f1ad1eb4d53 - rustc_query_system::query::plumbing::get_query::hfb1abdd81880abed
 125:     0x7f1ad11ea01c - rustc_interface::passes::QueryContext::enter::hc5239499ac91c0dc
 126:     0x7f1ad11c836c - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h0de99c193e2c54da
 127:     0x7f1ad11b1200 - rustc_span::with_source_map::h089611d3aa780b19
 128:     0x7f1ad11c711c - scoped_tls::ScopedKey<T>::set::hf1288dda44ea1fab
 129:     0x7f1ad11b1f47 - rustc_span::with_session_globals::h850f4dbd8f1f97a7
 130:     0x7f1ad11ec502 - std::sys_common::backtrace::__rust_begin_short_backtrace::h0c6b3ad136458e50
 131:     0x7f1ad11ed8ea - core::ops::function::FnOnce::call_once{{vtable.shim}}::hebe5949fe38ebdc0
 132:     0x7f1ad08f4f88 - std::sys::unix::thread::Thread::new::thread_start::h16c17e1aadac4afd
 133:     0x7f1acb5a96db - start_thread
 134:     0x7f1ad057571f - __clone
 135:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (dee28142f 2021-04-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 11637 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 132.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:59
