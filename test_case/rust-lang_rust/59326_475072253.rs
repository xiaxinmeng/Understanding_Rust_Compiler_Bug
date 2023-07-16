
stack backtrace:
   0:     0x7faa480b38bf - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h491d55826960b433
                               at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1:     0x7faa480d7317 - std::sys_common::backtrace::print::hc944b112770bad52
                               at src/libstd/sys_common/backtrace.rs:70
                               at src/libstd/sys_common/backtrace.rs:58
   2:     0x7faa480c111d - std::panicking::default_hook::{{closure}}::hd2dc3ad80ef22d72
                               at src/libstd/panicking.rs:200
   3:     0x7faa480c0e93 - std::panicking::default_hook::hf538c2fde927616a
                               at src/libstd/panicking.rs:215
   4:     0x7faa43b8bfbf - rustc::util::common::panic_hook::hc8481fff5a79da47
                               at src/librustc/util/common.rs:39
   5:     0x7faa480c19a9 - std::panicking::rust_panic_with_hook::h832f249017f7afee
                               at src/libstd/panicking.rs:482
   6:     0x7faa42f02cb4 - std::panicking::begin_panic::h0ae094f124031d56
                               at /build/rust/src/libstd/panicking.rs:412
   7:     0x7faa42f1526f - rustc_errors::Handler::emit_db::h49929f998e7c99d0
                               at src/librustc_errors/lib.rs:488
                               at src/librustc_errors/lib.rs:595
                               at src/librustc_errors/lib.rs:712
   8:     0x7faa42f017a7 - rustc_errors::diagnostic_builder::DiagnosticBuilder::emit::h8b5d732f0c8c3789
                               at src/librustc_errors/diagnostic_builder.rs:88
   9:     0x7faa4661d4f4 - <rustc_typeck::check::coercion::CoerceMany<'gcx, 'tcx, 'exprs, E>>::coerce_inner::he1910291388b79d1
                               at src/librustc_typeck/check/coercion.rs:1247
  10:     0x7faa466bb051 - rustc_typeck::check::FnCtxt::check_block_with_expected::h081bee4aeb47b284
                               at src/librustc_typeck/check/coercion.rs:1065
                               at src/librustc_typeck/check/mod.rs:4953
                               at src/librustc_typeck/check/mod.rs:5619
                               at src/librustc_typeck/check/mod.rs:4937
  11:     0x7faa466b014f - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:4482
  12:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  13:     0x7faa466af089 - rustc_typeck::check::FnCtxt::check_return_expr::h5a38f595cba97774
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:3196
                               at src/librustc_typeck/check/mod.rs:3334
  14:     0x7faa4669f4ac - rustc_typeck::check::check_fn::h2e3a570c9e6f5981
                               at src/librustc_typeck/check/mod.rs:1116
  15:     0x7faa46695ee5 - rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_expr_closure::h091a2b34cd3aa212
                               at src/librustc_typeck/check/closure.rs:84
                               at src/librustc_typeck/check/closure.rs:58
  16:     0x7faa466b12d3 - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:4479
  17:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  18:     0x7faa466baf91 - rustc_typeck::check::FnCtxt::check_block_with_expected::h081bee4aeb47b284
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:4944
                               at /build/rust/src/libcore/option.rs:414
                               at src/librustc_typeck/check/mod.rs:4944
                               at src/librustc_typeck/check/mod.rs:5619
                               at src/librustc_typeck/check/mod.rs:4937
  19:     0x7faa466b014f - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:4482
  20:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  21:     0x7faa466af089 - rustc_typeck::check::FnCtxt::check_return_expr::h5a38f595cba97774
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:3196
                               at src/librustc_typeck/check/mod.rs:3334
  22:     0x7faa4669f4ac - rustc_typeck::check::check_fn::h2e3a570c9e6f5981
                               at src/librustc_typeck/check/mod.rs:1116
  23:     0x7faa46695ee5 - rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_expr_closure::h091a2b34cd3aa212
                               at src/librustc_typeck/check/closure.rs:84
                               at src/librustc_typeck/check/closure.rs:58
  24:     0x7faa466b12d3 - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:4479
  25:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  26:     0x7faa466baf91 - rustc_typeck::check::FnCtxt::check_block_with_expected::h081bee4aeb47b284
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:4944
                               at /build/rust/src/libcore/option.rs:414
                               at src/librustc_typeck/check/mod.rs:4944
                               at src/librustc_typeck/check/mod.rs:5619
                               at src/librustc_typeck/check/mod.rs:4937
  27:     0x7faa466b014f - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:4482
  28:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  29:     0x7faa466ad37e - rustc_typeck::check::FnCtxt::check_argument_types::hcb1073213f29b05b
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:3027
  30:     0x7faa466ac220 - rustc_typeck::check::FnCtxt::check_method_argument_types::h0b75b38da5881056
                               at src/librustc_typeck/check/mod.rs:2784
  31:     0x7faa466b3c80 - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:3318
                               at src/librustc_typeck/check/mod.rs:4488
  32:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  33:     0x7faa466b053e - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:3210
                               at src/librustc_typeck/check/mod.rs:3291
                               at src/librustc_typeck/check/mod.rs:4488
  34:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  35:     0x7faa466baf91 - rustc_typeck::check::FnCtxt::check_block_with_expected::h081bee4aeb47b284
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:4944
                               at /build/rust/src/libcore/option.rs:414
                               at src/librustc_typeck/check/mod.rs:4944
                               at src/librustc_typeck/check/mod.rs:5619
                               at src/librustc_typeck/check/mod.rs:4937
  36:     0x7faa466b014f - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:4482
  37:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  38:     0x7faa466af089 - rustc_typeck::check::FnCtxt::check_return_expr::h5a38f595cba97774
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:3196
                               at src/librustc_typeck/check/mod.rs:3334
  39:     0x7faa4669f4ac - rustc_typeck::check::check_fn::h2e3a570c9e6f5981
                               at src/librustc_typeck/check/mod.rs:1116
  40:     0x7faa46695ee5 - rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_expr_closure::h091a2b34cd3aa212
                               at src/librustc_typeck/check/closure.rs:84
                               at src/librustc_typeck/check/closure.rs:58
  41:     0x7faa466b12d3 - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:4479
  42:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  43:     0x7faa466baf91 - rustc_typeck::check::FnCtxt::check_block_with_expected::h081bee4aeb47b284
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:4944
                               at /build/rust/src/libcore/option.rs:414
                               at src/librustc_typeck/check/mod.rs:4944
                               at src/librustc_typeck/check/mod.rs:5619
                               at src/librustc_typeck/check/mod.rs:4937
  44:     0x7faa466b014f - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:4482
  45:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  46:     0x7faa466ad37e - rustc_typeck::check::FnCtxt::check_argument_types::hcb1073213f29b05b
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:3027
  47:     0x7faa466ac220 - rustc_typeck::check::FnCtxt::check_method_argument_types::h0b75b38da5881056
                               at src/librustc_typeck/check/mod.rs:2784
  48:     0x7faa466b3c80 - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:3318
                               at src/librustc_typeck/check/mod.rs:4488
  49:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  50:     0x7faa466baaab - rustc_typeck::check::FnCtxt::check_stmt::h9c7d2ab54b9c4b44
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:3206
                               at src/librustc_typeck/check/mod.rs:4874
  51:     0x7faa466baf5a - rustc_typeck::check::FnCtxt::check_block_with_expected::h081bee4aeb47b284
                               at src/librustc_typeck/check/mod.rs:4939
                               at src/librustc_typeck/check/mod.rs:5619
                               at src/librustc_typeck/check/mod.rs:4937
  52:     0x7faa466b014f - rustc_typeck::check::FnCtxt::check_expr_kind::h3a5fa3b7b2d03743
                               at src/librustc_typeck/check/mod.rs:4482
  53:     0x7faa466afee0 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h4f9b69d5c57806da
                               at src/librustc_typeck/check/mod.rs:4013
  54:     0x7faa466af089 - rustc_typeck::check::FnCtxt::check_return_expr::h5a38f595cba97774
                               at src/librustc_typeck/check/mod.rs:3202
                               at src/librustc_typeck/check/mod.rs:3196
                               at src/librustc_typeck/check/mod.rs:3334
  55:     0x7faa4669f4ac - rustc_typeck::check::check_fn::h2e3a570c9e6f5981
                               at src/librustc_typeck/check/mod.rs:1116
  56:     0x7faa46794962 - rustc::ty::context::GlobalCtxt::enter_local::he0008d8af72ec408
                               at src/librustc_typeck/check/mod.rs:868
                               at src/librustc_typeck/check/mod.rs:615
                               at /build/rust/src/librustc/infer/mod.rs:520
                               at /build/rust/src/librustc/ty/context.rs:1685
                               at /build/rust/src/librustc/ty/context.rs:1924
                               at /build/rust/src/librustc/ty/context.rs:1857
                               at /build/rust/src/librustc/ty/context.rs:1923
                               at /build/rust/src/librustc/ty/context.rs:1684
                               at /build/rust/src/librustc/ty/context.rs:2030
                               at /build/rust/src/librustc/ty/context.rs:2014
                               at /build/rust/src/librustc/ty/context.rs:2004
                               at /build/rust/src/librustc/ty/context.rs:2014
                               at /build/rust/src/librustc/ty/context.rs:2026
                               at /build/rust/src/librustc/ty/context.rs:1676
  57:     0x7faa4669e14f - rustc_typeck::check::typeck_tables_of::h0364e7da5abae100
                               at /build/rust/src/librustc/infer/mod.rs:519
                               at src/librustc_typeck/check/mod.rs:615
                               at src/librustc_typeck/check/mod.rs:852
  58:     0x7faa43fd937c - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute::h3f1d7feafd2776b9
                               at src/librustc/ty/query/plumbing.rs:990
  59:     0x7faa4406b026 - rustc::dep_graph::graph::DepGraph::with_task_impl::ha97b247d2a229a88
                               at src/librustc/dep_graph/graph.rs:254
                               at src/librustc/ty/context.rs:1924
                               at src/librustc/ty/context.rs:1857
                               at src/librustc/ty/context.rs:1923
                               at src/librustc/dep_graph/graph.rs:253
                               at src/librustc/ty/context.rs:2014
                               at src/librustc/ty/context.rs:2004
                               at src/librustc/ty/context.rs:2014
                               at src/librustc/dep_graph/graph.rs:247
  60:     0x7faa43e8645c - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with::hb0ee0e148c611728
                               at src/librustc/dep_graph/graph.rs:181
                               at src/librustc/ty/query/plumbing.rs:587
                               at src/librustc/ty/query/plumbing.rs:220
                               at src/librustc/ty/context.rs:1924
                               at src/librustc/ty/context.rs:1857
                               at src/librustc/ty/context.rs:1923
                               at src/librustc/ty/query/plumbing.rs:219
                               at src/librustc/ty/context.rs:2030
                               at src/librustc/ty/context.rs:2014
                               at src/librustc/ty/context.rs:2004
                               at src/librustc/ty/context.rs:2014
                               at src/librustc/ty/context.rs:2026
                               at src/librustc/ty/query/plumbing.rs:208
                               at src/librustc/ty/query/plumbing.rs:580
                               at src/librustc/ty/query/plumbing.rs:233
                               at src/librustc/ty/query/plumbing.rs:579
                               at src/librustc/ty/query/plumbing.rs:448
  61:     0x7faa43f243de - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query::h73e08f33e6d1aa10
                               at src/librustc/ty/query/plumbing.rs:686
                               at src/librustc/ty/query/plumbing.rs:635
  62:     0x7faa4652a2ac - rustc::session::Session::track_errors::h3c217a66b10ec749
                               at src/librustc_typeck/check/mod.rs:724
                               at /build/rust/src/librustc/ty/mod.rs:2696
                               at /build/rust/src/libcore/iter/iterator.rs:606
                               at /build/rust/src/libcore/slice/mod.rs:2913
                               at /build/rust/src/libcore/iter/iterator.rs:606
                               at /build/rust/src/librustc/ty/mod.rs:2695
                               at src/librustc_typeck/check/mod.rs:723
                               at /build/rust/src/librustc/session/mod.rs:324
  63:     0x7faa4669dc3e - rustc_typeck::check::typeck_item_bodies::hfaa6e0ab992179a2
                               at src/librustc_typeck/check/mod.rs:722
  64:     0x7faa46614ea6 - rustc::ty::query::__query_compute::typeck_item_bodies::h91dba797adf9377a
                               at /build/rust/src/librustc/ty/query/plumbing.rs:998
                               at /build/rust/src/librustc/ty/query/plumbing.rs:957
  65:     0x7faa46714e08 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute::h43d0ee6521110ffb
                               at /build/rust/src/librustc/ty/query/plumbing.rs:990
  66:     0x7faa467b6e25 - rustc::dep_graph::graph::DepGraph::with_task_impl::ha513f9ddad293b48
                               at /build/rust/src/librustc/dep_graph/graph.rs:254
                               at /build/rust/src/librustc/ty/context.rs:1924
                               at /build/rust/src/librustc/ty/context.rs:1857
                               at /build/rust/src/librustc/ty/context.rs:1923
                               at /build/rust/src/librustc/dep_graph/graph.rs:253
                               at /build/rust/src/librustc/ty/context.rs:2014
                               at /build/rust/src/librustc/ty/context.rs:2004
                               at /build/rust/src/librustc/ty/context.rs:2014
                               at /build/rust/src/librustc/dep_graph/graph.rs:247
  67:     0x7faa4676fc57 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with::he823459c511e55c5
                               at /build/rust/src/librustc/dep_graph/graph.rs:181
                               at /build/rust/src/librustc/ty/query/plumbing.rs:587
                               at /build/rust/src/librustc/ty/query/plumbing.rs:220
                               at /build/rust/src/librustc/ty/context.rs:1924
                               at /build/rust/src/librustc/ty/context.rs:1857
                               at /build/rust/src/librustc/ty/context.rs:1923
                               at /build/rust/src/librustc/ty/query/plumbing.rs:219
                               at /build/rust/src/librustc/ty/context.rs:2030
                               at /build/rust/src/librustc/ty/context.rs:2014
                               at /build/rust/src/librustc/ty/context.rs:2004
                               at /build/rust/src/librustc/ty/context.rs:2014
                               at /build/rust/src/librustc/ty/context.rs:2026
                               at /build/rust/src/librustc/ty/query/plumbing.rs:208
                               at /build/rust/src/librustc/ty/query/plumbing.rs:580
                               at /build/rust/src/librustc/ty/query/plumbing.rs:233
                               at /build/rust/src/librustc/ty/query/plumbing.rs:579
                               at /build/rust/src/librustc/ty/query/plumbing.rs:448
  68:     0x7faa46559e84 - rustc::util::common::time::h9951047a13e422d2
                               at /build/rust/src/librustc/ty/query/plumbing.rs:686
                               at /build/rust/src/librustc/ty/query/plumbing.rs:1056
                               at /build/rust/src/librustc/ty/query/plumbing.rs:1048
                               at src/librustc_typeck/check/mod.rs:715
                               at src/librustc_typeck/lib.rs:365
                               at /build/rust/src/librustc/util/common.rs:150
                               at /build/rust/src/librustc/util/common.rs:144
  69:     0x7faa46531a2d - rustc_typeck::check_crate::hff96fb41fa5237f9
                               at src/librustc_typeck/lib.rs:365
  70:     0x7faa484c5235 - <std::thread::local::LocalKey<T>>::with::h29ba31b55ec4d810
                               at src/librustc_driver/driver.rs:1265
                               at /build/rust/src/librustc/ty/context.rs:1958
                               at /build/rust/src/librustc/ty/context.rs:1924
                               at /build/rust/src/librustc/ty/context.rs:1857
                               at /build/rust/src/librustc/ty/context.rs:1923
                               at /build/rust/src/librustc/ty/context.rs:1957
                               at /build/rust/src/librustc/ty/context.rs:1912
                               at /build/rust/src/libstd/thread/local.rs:299
                               at /build/rust/src/libstd/thread/local.rs:245
                               at /build/rust/src/librustc/ty/context.rs:1904
                               at /build/rust/src/libstd/thread/local.rs:299
                               at /build/rust/src/libstd/thread/local.rs:245
  71:     0x7faa4843e9ae - rustc::ty::context::TyCtxt::create_and_enter::h019642ef52479dad
                               at /build/rust/src/librustc/ty/context.rs:1896
                               at /build/rust/src/librustc/ty/context.rs:1935
                               at /build/rust/src/librustc/ty/context.rs:1255
  72:     0x7faa483d077f - rustc_driver::driver::compile_input::hd165033b2a205f4d
                               at src/librustc_driver/driver.rs:1229
                               at src/librustc_driver/driver.rs:275
  73:     0x7faa48468c20 - rustc_driver::run_compiler_with_pool::h9032e7c5f503b5e9
                               at src/librustc_driver/lib.rs:527
  74:     0x7faa48458ef5 - <scoped_tls::ScopedKey<T>>::set::h2b39d1a1d4c4d36d
                               at src/librustc_driver/lib.rs:449
                               at src/librustc_driver/driver.rs:65
                               at /build/rust/vendor/scoped-tls/src/lib.rs:155
  75:     0x7faa48467a6a - rustc_driver::run_compiler::h2eaf4645c8037d02
                               at src/librustc_driver/driver.rs:64
                               at src/librustc_driver/lib.rs:448
  76:     0x7faa484590aa - <scoped_tls::ScopedKey<T>>::set::hc28c1ed216cc584f
                               at src/librustc_driver/lib.rs:1644
                               at src/librustc_driver/lib.rs:171
                               at /build/rust/vendor/scoped-tls/src/lib.rs:155
                               at /build/rust/src/libsyntax/lib.rs:111
                               at /build/rust/vendor/scoped-tls/src/lib.rs:155
  77:     0x7faa484f0302 - std::sys_common::backtrace::__rust_begin_short_backtrace::h8fe9197fc385e149
                               at /build/rust/src/libsyntax/lib.rs:110
                               at src/librustc_driver/lib.rs:170
                               at src/librustc_driver/lib.rs:1560
                               at /build/rust/src/libstd/sys_common/backtrace.rs:135
  78:     0x7faa480db389 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:92
  79:     0x7faa484fa230 - <F as alloc::boxed::FnBox<A>>::call_box::h6d11bf519f2c149a
                               at /build/rust/src/libstd/panicking.rs:276
                               at /build/rust/src/libstd/panic.rs:388
                               at /build/rust/src/libstd/thread/mod.rs:468
                               at /build/rust/src/liballoc/boxed.rs:734
  80:     0x7faa480bbc1d - std::sys_common::thread::start_thread::h7d27c88ec451b131
                               at /build/rust/src/liballoc/boxed.rs:744
                               at src/libstd/sys_common/thread.rs:14
  81:     0x7faa480c82e5 - std::sys::unix::thread::Thread::new::thread_start::h05fffcc062c98212
                               at src/libstd/sys/unix/thread.rs:81
  82:     0x7faa41d0c66d - start_thread
                               at /glibc/2.26/src/glibc-2.26/nptl/pthread_create.c:465
  83:     0x7faa47d92e2e - clone
                               at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  84:                0x0 - <unknown>
query stack during panic:
#0 [typeck_tables_of] processing `thrift::make_thrift`
#1 [typeck_item_bodies] type-checking all item bodies
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug -C linker=<redacted>/ld.sh -C link-arg=@<redacted> -C relocation-model=pic -C force-frame-pointers=yes -C debuginfo=2 -C opt-level=1 -C debug-assertions=on -C codegen-units=8 -C linker-flavor=gcc -C incremental -C rpath --crate-type bin

note: some of the compiler flags provided by cargo are hidden
