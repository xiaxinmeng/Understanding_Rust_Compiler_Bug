text
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:212
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:479
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc::hir::map::Map::ty_param_owner
  17: rustc_typeck::collect::type_param_predicates
  18: rustc::ty::query::__query_compute::type_param_predicates
  19: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_param_predicates>::compute
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  22: <rustc_typeck::collect::ItemCtxt as rustc_typeck::astconv::AstConv>::get_type_parameter_bounds
  23: <dyn rustc_typeck::astconv::AstConv>::associated_path_to_ty
  24: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty
  25: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
  26: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  27: <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_path
  28: rustc::hir::PathSegment::with_generic_args
  29: <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_trait_ref
  30: <dyn rustc_typeck::astconv::AstConv>::instantiate_poly_trait_ref_inner
  31: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
  32: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  33: <dyn rustc_typeck::astconv::AstConv>::conv_object_ty_poly_trait_ref
  34: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty
  35: rustc_typeck::collect::checked_type_of
  36: rustc_typeck::collect::type_of
  37: rustc::ty::query::__query_compute::type_of
  38: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  39: rustc::dep_graph::graph::DepGraph::with_task_impl
  40: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  41: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_item
  42: rustc::hir::map::Map::visit_item_likes_in_module
  43: rustc_typeck::collect::collect_mod_item_types
  44: rustc::ty::query::__query_compute::collect_mod_item_types
  45: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  46: rustc::dep_graph::graph::DepGraph::with_task_impl
  47: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  48: rustc_typeck::check_crate::{{closure}}::{{closure}}
  49: rustc::util::common::time
  50: rustc_typeck::check_crate
  51: rustc_interface::passes::analysis
  52: rustc::ty::query::__query_compute::analysis
  53: rustc::dep_graph::graph::DepGraph::with_task_impl
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  55: rustc::ty::context::tls::enter_global
  56: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  57: rustc_interface::passes::create_global_ctxt::{{closure}}
  58: rustc_interface::interface::run_compiler_in_existing_thread_pool
  59: std::thread::local::LocalKey<T>::with
  60: scoped_tls::ScopedKey<T>::set
  61: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
error: internal compiler error: src/librustc/hir/map/mod.rs:526: ty_param_name: ty Foo (hir_id=HirId { owner: DefIndex(12), local_id: 0 }) not a type parameter

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
stack backtrace:
   0:     0x7fed8c71012b - backtrace::backtrace::libunwind::trace::h1e46ebaf3c192a61
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1:     0x7fed8c71012b - backtrace::backtrace::trace_unsynchronized::h832e176a24c9a40a
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2:     0x7fed8c71012b - std::sys_common::backtrace::_print::h92fe8656fc855359
                               at src/libstd/sys_common/backtrace.rs:47
   3:     0x7fed8c71012b - std::sys_common::backtrace::print::h9353bb76a55ce6c2
                               at src/libstd/sys_common/backtrace.rs:36
   4:     0x7fed8c71012b - std::panicking::default_hook::{{closure}}::h38992ec49c46f045
                               at src/libstd/panicking.rs:198
   5:     0x7fed8c70fe2e - std::panicking::default_hook::hb191f9c753615e7d
                               at src/libstd/panicking.rs:212
   6:     0x7fed89c70a41 - rustc::util::common::panic_hook::h758e3e3e1747ddb4
   7:     0x7fed8c710989 - std::panicking::rust_panic_with_hook::h2dced0d9ecdbd485
                               at src/libstd/panicking.rs:479
   8:     0x7fed888c84ad - std::panicking::begin_panic::h3acf2b387aafb7bb
   9:     0x7fed888e867f - rustc_errors::Handler::bug::hdaa9408759ecc54f
  10:     0x7fed899cbd73 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::hd883cfc35b923dfe
  11:     0x7fed899bdcba - rustc::ty::context::tls::with_opt::{{closure}}::hc82dad6dd233aaae
  12:     0x7fed899bdbd5 - rustc::ty::context::tls::with_context_opt::h4ad33b4ba52d58a8
  13:     0x7fed899bdc67 - rustc::ty::context::tls::with_opt::hb752583dbd10a5a4
  14:     0x7fed899cbc88 - rustc::util::bug::opt_span_bug_fmt::h9c367d1623169f7b
  15:     0x7fed899cbbf2 - rustc::util::bug::bug_fmt::hd7be37cb9753c073
  16:     0x7fed89821740 - rustc::hir::map::Map::ty_param_name::hf4531b63b89d25fb
  17:     0x7fed89d8713d - rustc::query::<impl rustc::ty::query::config::QueryDescription for rustc::ty::query::queries::type_param_predicates>::describe::h213aad71f56d83a5
  18:     0x7fed89db2e73 - rustc::ty::query::Query::describe::h3c58a14dc67c66a4
  19:     0x7fed89a0f1d1 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_print_query_stack::h3b8d87be3a68b0bd
  20:     0x7fed89c70aba - rustc::util::common::panic_hook::h758e3e3e1747ddb4
  21:     0x7fed8c710989 - std::panicking::rust_panic_with_hook::h2dced0d9ecdbd485
                               at src/libstd/panicking.rs:479
  22:     0x7fed888c84ad - std::panicking::begin_panic::h3acf2b387aafb7bb
  23:     0x7fed888e867f - rustc_errors::Handler::bug::hdaa9408759ecc54f
  24:     0x7fed899cbd73 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::hd883cfc35b923dfe
  25:     0x7fed899bdcba - rustc::ty::context::tls::with_opt::{{closure}}::hc82dad6dd233aaae
  26:     0x7fed899bdbd5 - rustc::ty::context::tls::with_context_opt::h4ad33b4ba52d58a8
  27:     0x7fed899bdc67 - rustc::ty::context::tls::with_opt::hb752583dbd10a5a4
  28:     0x7fed899cbc88 - rustc::util::bug::opt_span_bug_fmt::h9c367d1623169f7b
  29:     0x7fed899cbbf2 - rustc::util::bug::bug_fmt::hd7be37cb9753c073
  30:     0x7fed8982151a - rustc::hir::map::Map::ty_param_owner::h0041e014cfc33293
  31:     0x7fed86a5273b - rustc_typeck::collect::type_param_predicates::hc16b2b96fb16f5f2
  32:     0x7fed8688739c - rustc::ty::query::__query_compute::type_param_predicates::hfc99112e16affa1f
  33:     0x7fed86951320 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_param_predicates>::compute::he239ce2cbff902be
  34:     0x7fed868c3f0c - rustc::dep_graph::graph::DepGraph::with_task_impl::hf6ec3157c1611232
  35:     0x7fed869a8496 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::ha016e0f4a718f225
  36:     0x7fed86a5208f - <rustc_typeck::collect::ItemCtxt as rustc_typeck::astconv::AstConv>::get_type_parameter_bounds::h771625fddc7d5355
  37:     0x7fed8693a2d0 - <dyn rustc_typeck::astconv::AstConv>::associated_path_to_ty::h3bdb0a22119cc0a6
  38:     0x7fed8693f8a4 - <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty::hc673151c4ca9f5a3
  39:     0x7fed86a8c727 - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h9538616d2f022a81
  40:     0x7fed86a0a716 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter::h34bef08b33b06bb5
  41:     0x7fed869331d0 - <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_path::he5cb80fd273966e1
  42:     0x7fed86a79267 - rustc::hir::PathSegment::with_generic_args::hf5b24075601f3dc9
  43:     0x7fed86936443 - <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_trait_ref::h5467cb3f2cfbf909
  44:     0x7fed86933b31 - <dyn rustc_typeck::astconv::AstConv>::instantiate_poly_trait_ref_inner::hafc3a7f6f49e48b9
  45:     0x7fed86a8afc9 - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h6981637606cfae57
  46:     0x7fed86a14031 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter::hdbb21299b19f5bb9
  47:     0x7fed86937569 - <dyn rustc_typeck::astconv::AstConv>::conv_object_ty_poly_trait_ref::hebfabc59dc99f9ad
  48:     0x7fed8693f8f3 - <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty::hc673151c4ca9f5a3
  49:     0x7fed86a558d7 - rustc_typeck::collect::checked_type_of::h6c433a9fb178bc7a
  50:     0x7fed86a550bd - rustc_typeck::collect::type_of::hf2f44f3eb361e7af
  51:     0x7fed8688834d - rustc::ty::query::__query_compute::type_of::h4710af560105f8fb
  52:     0x7fed86950c8d - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute::h2234ac72e993e35c
  53:     0x7fed868c2487 - rustc::dep_graph::graph::DepGraph::with_task_impl::he83e5eaf3cfafcb1
  54:     0x7fed86962155 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h050c71118c0be594
  55:     0x7fed86a50b38 - <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_item::h8d6cce7b308a1644
  56:     0x7fed86ab1213 - rustc::hir::map::Map::visit_item_likes_in_module::hedc475f59c6a5e2d
  57:     0x7fed86a5098d - rustc_typeck::collect::collect_mod_item_types::h3c601da4f46ed144
  58:     0x7fed8688757e - rustc::ty::query::__query_compute::collect_mod_item_types::h1d438d32aef373a6
  59:     0x7fed8695137d - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute::h7de730ccf2369324
  60:     0x7fed868b9f82 - rustc::dep_graph::graph::DepGraph::with_task_impl::h7e6648c9b9c87028
  61:     0x7fed8697bdf5 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h49aaca2adad2bea0
  62:     0x7fed868a8408 - rustc_typeck::check_crate::{{closure}}::{{closure}}::hfcc20b5b98c19e4f
  63:     0x7fed8688b361 - rustc::util::common::time::h7f512c590b4aa935
  64:     0x7fed86a9cadd - rustc_typeck::check_crate::h9c0450d6bb4c1260
  65:     0x7fed8b8f4485 - rustc_interface::passes::analysis::h9d1d9d04973244c1
  66:     0x7fed8ca6f596 - rustc::ty::query::__query_compute::analysis::h586e069be04d1cc7
  67:     0x7fed8c9db926 - rustc::dep_graph::graph::DepGraph::with_task_impl::hbcdc4c571d9f4a3e
  68:     0x7fed8c9d768c - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::he4a6f9da9c9c2145
  69:     0x7fed8c9e4546 - rustc::ty::context::tls::enter_global::h9bae3a08ae469b54
  70:     0x7fed8ca031b7 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h84e7023a23dfaced
  71:     0x7fed8b951085 - rustc_interface::passes::create_global_ctxt::{{closure}}::h5df8d050b23e11a4
  72:     0x7fed8ca0574a - rustc_interface::interface::run_compiler_in_existing_thread_pool::h8b6c9083dc9dd440
  73:     0x7fed8ca75716 - std::thread::local::LocalKey<T>::with::h8ed9ca8e61af6bd7
  74:     0x7fed8ca0e94e - scoped_tls::ScopedKey<T>::set::h3dc5c9d2f5dc9dab
  75:     0x7fed8ca41424 - syntax::with_globals::hd5804ce6ce845ef3
  76:     0x7fed8ca5fcfd - std::sys_common::backtrace::__rust_begin_short_backtrace::h4f160f23c3112619
  77:     0x7fed8c72182a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:82
  78:     0x7fed8c9ec459 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h7ca0938593f67d78
  79:     0x7fed8c6f3e2f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h21ff578a0d337c71
                               at /rustc/b25ee644971a168287ee166edbd11642dbcfeab8/src/liballoc/boxed.rs:746
  80:     0x7fed8c720510 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h5f312234fee81f04
                               at /rustc/b25ee644971a168287ee166edbd11642dbcfeab8/src/liballoc/boxed.rs:746
  81:     0x7fed8c720510 - std::sys_common::thread::start_thread::hbb06d0cae8d971b5
                               at src/libstd/sys_common/thread.rs:13
  82:     0x7fed8c720510 - std::sys::unix::thread::Thread::new::thread_start::hdc276c5ed6369660
                               at src/libstd/sys/unix/thread.rs:79
  83:     0x7fed8c4906db - start_thread
  84:     0x7fed8bdad88f - __clone
  85:                0x0 - <unknown>
query stack during panic:
thread panicked while processing panic. aborting.
error: Could not compile `playground`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name playground src/lib.rs --color never --crate-type lib --emit=dep-info,metadata,link -C codegen-units=1 -C debuginfo=2 -C metadata=19f522786ba73a83 -C extra-filename=-19f522786ba73a83 --out-dir /playground/target/debug/deps -L dependency=/playground/target/debug/deps --extern adler32=/playground/target/debug/deps/libadler32-7c094a8813a358b0.rlib --extern aho_corasick=/playground/target/debug/deps/libaho_corasick-80ba074d31b8de5e.rlib --extern ansi_term=/playground/target/debug/deps/libansi_term-41f52cfda1aa1469.rlib --extern argon2rs=/playground/target/debug/deps/libargon2rs-d3d08a28a81ef101.rlib --extern arrayvec=/playground/target/debug/deps/libarrayvec-87acc7b6c6359645.rlib --extern atty=/playground/target/debug/deps/libatty-da8e87b143f7a1ac.rlib --extern autocfg=/playground/target/debug/deps/libautocfg-1f7e8c7c8d4f5a85.rlib --extern backtrace=/playground/target/debug/deps/libbacktrace-d9bd9f7d8111b897.rlib --extern backtrace_sys=/playground/target/debug/deps/libbacktrace_sys-284e16fc4a19465f.rlib --extern base64=/playground/target/debug/deps/libbase64-5b5a361f12d0fdba.rlib --extern bit_set=/playground/target/debug/deps/libbit_set-1eb9882640f101e1.rlib --extern bit_vec=/playground/target/debug/deps/libbit_vec-81ff29ad4d27b2c0.rlib --extern bitflags=/playground/target/debug/deps/libbitflags-aefbbc04a2f7273e.rlib --extern blake2_rfc=/playground/target/debug/deps/libblake2_rfc-7325d3bc117e23a8.rlib --extern build_const=/playground/target/debug/deps/libbuild_const-6d3a851f8c9f37b4.rlib --extern byteorder=/playground/target/debug/deps/libbyteorder-e83f634064758d01.rlib --extern bytes=/playground/target/debug/deps/libbytes-297d2533f68e1611.rlib --extern cc=/playground/target/debug/deps/libcc-4590db2ada8cf1c7.rlib --extern cfg_if=/playground/target/debug/deps/libcfg_if-f893b2a396238f73.rlib --extern chrono=/playground/target/debug/deps/libchrono-4531ac7f15b8cd41.rlib --extern clap=/playground/target/debug/deps/libclap-0c9bae5de0130cd4.rlib --extern cloudabi=/playground/target/debug/deps/libcloudabi-b7a2e93d2b0616d5.rlib --extern color_quant=/playground/target/debug/deps/libcolor_quant-154f768c106e9e42.rlib --extern constant_time_eq=/playground/target/debug/deps/libconstant_time_eq-384f5a44eb41b9d6.rlib --extern cookie=/playground/target/debug/deps/libcookie-ab933473ef2dfe2c.rlib --extern cookie_store=/playground/target/debug/deps/libcookie_store-40d1373a9b817d8d.rlib --extern crc=/playground/target/debug/deps/libcrc-8c45b1642f34cf47.rlib --extern crc32fast=/playground/target/debug/deps/libcrc32fast-1e431770b65c5964.rlib --extern crossbeam=/playground/target/debug/deps/libcrossbeam-39e2293123695cbc.rlib --extern crossbeam_channel=/playground/target/debug/deps/libcrossbeam_channel-6199f89430602936.rlib --extern crossbeam_deque=/playground/target/debug/deps/libcrossbeam_deque-ff1537855da4cb65.rlib --extern crossbeam_epoch=/playground/target/debug/deps/libcrossbeam_epoch-8609ba92eb8e458a.rlib --extern crossbeam_queue=/playground/target/debug/deps/libcrossbeam_queue-18d1c2f473c77198.rlib --extern crossbeam_utils=/playground/target/debug/deps/libcrossbeam_utils-51d596997013f8f7.rlib --extern csv=/playground/target/debug/deps/libcsv-b76a83ac17ddfc1f.rlib --extern csv_core=/playground/target/debug/deps/libcsv_core-1afc2661dfad21f7.rlib --extern data_encoding=/playground/target/debug/deps/libdata_encoding-e5485d390d21fafe.rlib --extern debug_unreachable=/playground/target/debug/deps/libdebug_unreachable-9f86ee3a7179ede7.rlib --extern deflate=/playground/target/debug/deps/libdeflate-32db26709505ab88.rlib --extern dirs=/playground/target/debug/deps/libdirs-5d4a07a1620e4df0.rlib --extern dtoa=/playground/target/debug/deps/libdtoa-98f773e33edbe6c2.rlib --extern either=/playground/target/debug/deps/libeither-47c788993350f6d7.rlib --extern encoding_rs=/playground/target/debug/deps/libencoding_rs-da3fdbf3e7081cc8.rlib --extern env_logger=/playground/target/debug/deps/libenv_logger-e8530fda051b51d6.rlib --extern error_chain=/playground/target/debug/deps/liberror_chain-6cf66e3466812308.rlib --extern failure=/playground/target/debug/deps/libfailure-4cc85b776975b736.rlib --extern failure_derive=/playground/target/debug/deps/libfailure_derive-5fbaef3cb89906e9.so --extern filetime=/playground/target/debug/deps/libfiletime-cfa8b0c35cf3c73a.rlib --extern fixedbitset=/playground/target/debug/deps/libfixedbitset-208118ad38b465b4.rlib --extern flate2=/playground/target/debug/deps/libflate2-e81d1342def3a87c.rlib --extern fnv=/playground/target/debug/deps/libfnv-40c91b76b26fd3fc.rlib --extern foreign_types=/playground/target/debug/deps/libforeign_types-df10bc65cbec7646.rlib --extern foreign_types_shared=/playground/target/debug/deps/libforeign_types_shared-b53a6119936cc1ea.rlib --extern fuchsia_cprng=/playground/target/debug/deps/libfuchsia_cprng-866e70994f23d3d1.rlib --extern fuchsia_zircon=/playground/target/debug/deps/libfuchsia_zircon-2a8f3b0ae6737d54.rlib --extern fuchsia_zircon_sys=/playground/target/debug/deps/libfuchsia_zircon_sys-18fd25a3743e2105.rlib --extern futf=/playground/target/debug/deps/libfutf-c227f2c12241078d.rlib --extern futures=/playground/target/debug/deps/libfutures-37c5255a4dadedc5.rlib --extern futures_cpupool=/playground/target/debug/deps/libfutures_cpupool-f6fd5543fceecf91.rlib --extern gcc=/playground/target/debug/deps/libgcc-2d221a06fc7ae214.rlib --extern gif=/playground/target/debug/deps/libgif-32181c5095c07be2.rlib --extern glob=/playground/target/debug/deps/libglob-add8e3b2742fb409.rlib --extern h2=/playground/target/debug/deps/libh2-f42fa1f3bf512db7.rlib --extern html5ever=/playground/target/debug/deps/libhtml5ever-8e672afad2145a74.rlib --extern http=/playground/target/debug/deps/libhttp-873d0a417a8fee20.rlib --extern http_body=/playground/target/debug/deps/libhttp_body-6fe7c218b20764d1.rlib --extern httparse=/playground/target/debug/deps/libhttparse-6fe857d0e33b923e.rlib --extern humantime=/playground/target/debug/deps/libhumantime-a50f48c8f58a6fe6.rlib --extern hyper=/playground/target/debug/deps/libhyper-02fcda9e5bdb1d33.rlib --extern hyper_tls=/playground/target/debug/deps/libhyper_tls-66d44ac25e995172.rlib --extern idna=/playground/target/debug/deps/libidna-5d2e53dfa4e3d682.rlib --extern image=/playground/target/debug/deps/libimage-218efaae80b88445.rlib --extern indexmap=/playground/target/debug/deps/libindexmap-83a99e7c10ef21e3.rlib --extern inflate=/playground/target/debug/deps/libinflate-a1bbbccaef60d85d.rlib --extern iovec=/playground/target/debug/deps/libiovec-ceb6ba56bab9f968.rlib --extern itertools=/playground/target/debug/deps/libitertools-f0613ef8dd5fddd9.rlib --extern itoa=/playground/target/debug/deps/libitoa-fc05a61be5e6965b.rlib --extern jpeg_decoder=/playground/target/debug/deps/libjpeg_decoder-61b5c33f3a26c3e4.rlib --extern kernel32=/playground/target/debug/deps/libkernel32-5b20158dce940e8d.rlib --extern lazy_static=/playground/target/debug/deps/liblazy_static-1daba977043c76b5.rlib --extern lazycell=/playground/target/debug/deps/liblazycell-76ddd4078a039b02.rlib --extern libc=/playground/target/debug/deps/liblibc-19326ae327507cab.rlib --extern lock_api=/playground/target/debug/deps/liblock_api-6f53243471463d97.rlib --extern log=/playground/target/debug/deps/liblog-67cc573507f464ca.rlib --extern lzw=/playground/target/debug/deps/liblzw-68b231192673c983.rlib --extern mac=/playground/target/debug/deps/libmac-ccd3bb73c6f3d548.rlib --extern markup5ever=/playground/target/debug/deps/libmarkup5ever-70d22ca4d02da5a2.rlib --extern matches=/playground/target/debug/deps/libmatches-a4d0b3d89aa31ddb.rlib --extern memchr=/playground/target/debug/deps/libmemchr-f554add1cd42bd14.rlib --extern memmap=/playground/target/debug/deps/libmemmap-755ccc2982ebff57.rlib --extern memoffset=/playground/target/debug/deps/libmemoffset-866ceec7bf87ae5d.rlib --extern mime=/playground/target/debug/deps/libmime-a84e7bd537559855.rlib --extern mime_guess=/playground/target/debug/deps/libmime_guess-66bc2fe3c289ecb4.rlib --extern miniz_sys=/playground/target/debug/deps/libminiz_sys-362e1849789dd4ff.rlib --extern miniz_oxide=/playground/target/debug/deps/libminiz_oxide-822d4688d3945492.rlib --extern miniz_oxide_c_api=/playground/target/debug/deps/libminiz_oxide_c_api-abb986c2586cda3b.rlib --extern mio=/playground/target/debug/deps/libmio-6a846ebd4af517b3.rlib --extern miow=/playground/target/debug/deps/libmiow-dccef74983c82bcb.rlib --extern native_tls=/playground/target/debug/deps/libnative_tls-8ff3b1e8886b74ef.rlib --extern net2=/playground/target/debug/deps/libnet2-abfd43b3703a8e59.rlib --extern debug_unreachable=/playground/target/debug/deps/libdebug_unreachable-53cbbc8451f8a571.rlib --extern nodrop=/playground/target/debug/deps/libnodrop-4d44853f417f87a5.rlib --extern num=/playground/target/debug/deps/libnum-72e9c329dccaa5b0.rlib --extern num_bigint=/playground/target/debug/deps/libnum_bigint-59bc2578e113e318.rlib --extern num_complex=/playground/target/debug/deps/libnum_complex-a64d3159ce7d4997.rlib --extern num_derive=/playground/target/debug/deps/libnum_derive-3b941f9d1fcbd8ca.so --extern num_integer=/playground/target/debug/deps/libnum_integer-9e3f86d6fcf73635.rlib --extern num_iter=/playground/target/debug/deps/libnum_iter-b15dc1b5b553c360.rlib --extern num_rational=/playground/target/debug/deps/libnum_rational-5907a9e8135107f5.rlib --extern num_traits=/playground/target/debug/deps/libnum_traits-c47e74a9bce834bb.rlib --extern num_cpus=/playground/target/debug/deps/libnum_cpus-10acd7070ee0f0e0.rlib --extern numtoa=/playground/target/debug/deps/libnumtoa-c7466c56fb2fadc6.rlib --extern openssl_probe=/playground/target/debug/deps/libopenssl_probe-808351013732df20.rlib --extern ordermap=/playground/target/debug/deps/libordermap-e44da526dfa57395.rlib --extern owning_ref=/playground/target/debug/deps/libowning_ref-df88320234e0437d.rlib --extern parking_lot=/playground/target/debug/deps/libparking_lot-1060770c50dc3d23.rlib --extern parking_lot_core=/playground/target/debug/deps/libparking_lot_core-4a4f0a4d6dacaa05.rlib --extern percent_encoding=/playground/target/debug/deps/libpercent_encoding-e39e47599f9cd167.rlib --extern petgraph=/playground/target/debug/deps/libpetgraph-e09dc607cbdd2ec5.rlib --extern phf=/playground/target/debug/deps/libphf-78ba6596d941de82.rlib --extern phf_codegen=/playground/target/debug/deps/libphf_codegen-856f8a9df324739d.rlib --extern phf_generator=/playground/target/debug/deps/libphf_generator-73fb34813dac16a3.rlib --extern phf_shared=/playground/target/debug/deps/libphf_shared-3f1f2631df81e311.rlib --extern pkg_config=/playground/target/debug/deps/libpkg_config-b67febc273c804b7.rlib --extern png=/playground/target/debug/deps/libpng-6a0b263076070b0d.rlib --extern precomputed_hash=/playground/target/debug/deps/libprecomputed_hash-2fc5041a87a5e684.rlib --extern proc_macro2=/playground/target/debug/deps/libproc_macro2-a90e17129a99911f.rlib --extern publicsuffix=/playground/target/debug/deps/libpublicsuffix-45515c2a566a0ea5.rlib --extern quick_error=/playground/target/debug/deps/libquick_error-548cd2f4cc0085b7.rlib --extern quote=/playground/target/debug/deps/libquote-fefbceb388d5dd08.rlib --extern rand=/playground/target/debug/deps/librand-af9dbf7e5c37e8d7.rlib --extern rand_chacha=/playground/target/debug/deps/librand_chacha-b417f083e5481cef.rlib --extern rand_core=/playground/target/debug/deps/librand_core-88de9a916edc81d0.rlib --extern rand_hc=/playground/target/debug/deps/librand_hc-4ab7a16e575920f0.rlib --extern rand_isaac=/playground/target/debug/deps/librand_isaac-50c31c8fe6f8c382.rlib --extern rand_jitter=/playground/target/debug/deps/librand_jitter-a445681c4ba111f0.rlib --extern rand_os=/playground/target/debug/deps/librand_os-d98c918291af3058.rlib --extern rand_pcg=/playground/target/debug/deps/librand_pcg-31a33403027563f0.rlib --extern rand_xorshift=/playground/target/debug/deps/librand_xorshift-92dd825cb99a815f.rlib --extern rayon=/playground/target/debug/deps/librayon-f8ae39899a48d567.rlib --extern rayon_core=/playground/target/debug/deps/librayon_core-2a71a462df9f8476.rlib --extern rdrand=/playground/target/debug/deps/librdrand-82e434538d1974b9.rlib --extern regex=/playground/target/debug/deps/libregex-d762d3afe1aac4bb.rlib --extern regex_syntax=/playground/target/debug/deps/libregex_syntax-9f37e5bbb18840b4.rlib --extern remove_dir_all=/playground/target/debug/deps/libremove_dir_all-ebb3ea9e986f63b8.rlib --extern reqwest=/playground/target/debug/deps/libreqwest-6243252dee62db44.rlib --extern ring=/playground/target/debug/deps/libring-5323e1405e47e6c5.rlib --extern rustc_demangle=/playground/target/debug/deps/librustc_demangle-61b6ea752e86df87.rlib --extern rustc_serialize=/playground/target/debug/deps/librustc_serialize-ef44315a31a754eb.rlib --extern rustc_version=/playground/target/debug/deps/librustc_version-2831a3c16dd64034.rlib --extern ryu=/playground/target/debug/deps/libryu-34ef57f22afc8e6d.rlib --extern same_file=/playground/target/debug/deps/libsame_file-39674e620c129f24.rlib --extern schannel=/playground/target/debug/deps/libschannel-c36d1f88d3b91b38.rlib --extern scoped_threadpool=/playground/target/debug/deps/libscoped_threadpool-7a1dfbac2d1ca360.rlib --extern scopeguard=/playground/target/debug/deps/libscopeguard-898e2fb42540e22b.rlib --extern select=/playground/target/debug/deps/libselect-dfbbd972ed566556.rlib --extern semver=/playground/target/debug/deps/libsemver-346fc8cab8ab8220.rlib --extern semver_parser=/playground/target/debug/deps/libsemver_parser-d5a8b6df003d327a.rlib --extern serde=/playground/target/debug/deps/libserde-6215d55b80792de3.rlib --extern serde_derive=/playground/target/debug/deps/libserde_derive-fa0d1f4040031dd7.so --extern serde_json=/playground/target/debug/deps/libserde_json-f365a94c03dc77df.rlib --extern serde_urlencoded=/playground/target/debug/deps/libserde_urlencoded-9f1f9d668f09d2a1.rlib --extern siphasher=/playground/target/debug/deps/libsiphasher-571789d049b307af.rlib --extern slab=/playground/target/debug/deps/libslab-351ab9a323b28b76.rlib --extern smallvec=/playground/target/debug/deps/libsmallvec-126e63f09868b017.rlib --extern spin=/playground/target/debug/deps/libspin-a1f77d9189798aa8.rlib --extern stable_deref_trait=/playground/target/debug/deps/libstable_deref_trait-6565ab85b28e455f.rlib --extern string=/playground/target/debug/deps/libstring-fbc8744b23c711cc.rlib --extern string_cache=/playground/target/debug/deps/libstring_cache-316e611f0708d46d.rlib --extern string_cache_codegen=/playground/target/debug/deps/libstring_cache_codegen-9761f0ef68a6fb1c.rlib --extern string_cache_shared=/playground/target/debug/deps/libstring_cache_shared-e314acca4954f9fe.rlib --extern strsim=/playground/target/debug/deps/libstrsim-36e20f9f18156352.rlib --extern syn=/playground/target/debug/deps/libsyn-c5f81cc9fad2c521.rlib --extern synom=/playground/target/debug/deps/libsynom-0a3816437589df40.rlib --extern synstructure=/playground/target/debug/deps/libsynstructure-a30a765c81ba83f9.rlib --extern syslog=/playground/target/debug/deps/libsyslog-3696b3cbbf7f3339.rlib --extern tar=/playground/target/debug/deps/libtar-6e0c38a74aafe786.rlib --extern tempdir=/playground/target/debug/deps/libtempdir-c1732e42d3761068.rlib --extern tempfile=/playground/target/debug/deps/libtempfile-b5e5297fe2435f2f.rlib --extern tendril=/playground/target/debug/deps/libtendril-272bc70fbe9209f4.rlib --extern term=/playground/target/debug/deps/libterm-beb35d7596fd0687.rlib --extern termcolor=/playground/target/debug/deps/libtermcolor-fb87113a57a09dbe.rlib --extern termion=/playground/target/debug/deps/libtermion-73d3a24de831d1e9.rlib --extern textwrap=/playground/target/debug/deps/libtextwrap-655b2728e7e96eb1.rlib --extern thread_id=/playground/target/debug/deps/libthread_id-c2cb963bbd1164f4.rlib --extern thread_local=/playground/target/debug/deps/libthread_local-c927ba4bdaedfdde.rlib --extern threadpool=/playground/target/debug/deps/libthreadpool-11ac5e3a43601f71.rlib --extern tiff=/playground/target/debug/deps/libtiff-5186ecb3964d28b0.rlib --extern time=/playground/target/debug/deps/libtime-c39b9e2098d84f99.rlib --extern tokio=/playground/target/debug/deps/libtokio-d02ba32b0d59440a.rlib --extern tokio_buf=/playground/target/debug/deps/libtokio_buf-7ec1bdd9482b45b3.rlib --extern tokio_current_thread=/playground/target/debug/deps/libtokio_current_thread-5835bfc5da88e40d.rlib --extern tokio_executor=/playground/target/debug/deps/libtokio_executor-cd133843979de66a.rlib --extern tokio_io=/playground/target/debug/deps/libtokio_io-6d257689b3144f06.rlib --extern tokio_reactor=/playground/target/debug/deps/libtokio_reactor-b05beef75d21cff5.rlib --extern tokio_sync=/playground/target/debug/deps/libtokio_sync-a372bc8e5c0d1926.rlib --extern tokio_tcp=/playground/target/debug/deps/libtokio_tcp-4ced89e50afc48a9.rlib --extern tokio_threadpool=/playground/target/debug/deps/libtokio_threadpool-fb3668dac9a222f0.rlib --extern tokio_timer=/playground/target/debug/deps/libtokio_timer-039b95f86e29aaae.rlib --extern tokio_trace_core=/playground/target/debug/deps/libtokio_trace_core-9ccc1138a8db3b81.rlib --extern toml=/playground/target/debug/deps/libtoml-a87043230394bbfb.rlib --extern try_lock=/playground/target/debug/deps/libtry_lock-d76034d315d1464d.rlib --extern try_from=/playground/target/debug/deps/libtry_from-2cb526a55d29072c.rlib --extern ucd_util=/playground/target/debug/deps/libucd_util-7c0d914e8f9bc462.rlib --extern unicase=/playground/target/debug/deps/libunicase-037f5acc8f230532.rlib --extern unicode_bidi=/playground/target/debug/deps/libunicode_bidi-4d3edd68e3a0b0d4.rlib --extern unicode_normalization=/playground/target/debug/deps/libunicode_normalization-8df982957f607234.rlib --extern unicode_width=/playground/target/debug/deps/libunicode_width-4ebbd2f88961184d.rlib --extern unicode_xid=/playground/target/debug/deps/libunicode_xid-63e2599485740622.rlib --extern unreachable=/playground/target/debug/deps/libunreachable-35ff1ef88e79651f.rlib --extern untrusted=/playground/target/debug/deps/libuntrusted-a4d76c6ece073e6e.rlib --extern url=/playground/target/debug/deps/liburl-5b153db141897640.rlib --extern utf8=/playground/target/debug/deps/libutf8-0eac97bd75e8fa5f.rlib --extern utf8_ranges=/playground/target/debug/deps/libutf8_ranges-dcef7d980c012e01.rlib --extern uuid=/playground/target/debug/deps/libuuid-669e14bf53f48970.rlib --extern vcpkg=/playground/target/debug/deps/libvcpkg-41fbda264cad550b.rlib --extern vec_map=/playground/target/debug/deps/libvec_map-fb7106e7fc542a8a.rlib --extern version_check=/playground/target/debug/deps/libversion_check-b5b6a581b21a8f83.rlib --extern void=/playground/target/debug/deps/libvoid-5963f56d4f26650d.rlib --extern walkdir=/playground/target/debug/deps/libwalkdir-065a07348d62dfa7.rlib --extern want=/playground/target/debug/deps/libwant-40952db6170e7e4a.rlib --extern winapi=/playground/target/debug/deps/libwinapi-b08d8288a9a2ff03.rlib --extern build=/playground/target/debug/deps/libbuild-d6256f232aa4628c.rlib --extern winapi_i686_pc_windows_gnu=/playground/target/debug/deps/libwinapi_i686_pc_windows_gnu-ef9cbd4597ca34e4.rlib --extern winapi_util=/playground/target/debug/deps/libwinapi_util-73b19325b60dfe38.rlib --extern winapi_x86_64_pc_windows_gnu=/playground/target/debug/deps/libwinapi_x86_64_pc_windows_gnu-ac5fd45d8680b672.rlib --extern wincolor=/playground/target/debug/deps/libwincolor-280bd09a8462b711.rlib --extern ws2_32=/playground/target/debug/deps/libws2_32-f8020458de3039c5.rlib --extern xattr=/playground/target/debug/deps/libxattr-41d0a2f0a3be4069.rlib -L native=/playground/target/debug/build/backtrace-sys-0bad857f246b8d46/out -L native=/playground/target/debug/build/miniz-sys-cc52677d966ce23b/out -L native=/playground/target/debug/build/ring-3ec1d93b15dca95a/out` (signal: 4, SIGILL: illegal instruction)
