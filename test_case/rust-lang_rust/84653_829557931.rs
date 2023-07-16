
$ /usr/local/google/home/richkadel/rust/install/fuchsia-rust/bin/rustc --color=always --crate-name packet ../../src/lib/network/packet/src/lib.rs --crate-type rlib --emit=dep-info=host_x64-coverage-rust/obj/src/lib/network/packet/libpacket.rlib.d,link -Zdep-info-omit-d-target --cfg=rust_panic=\"unwind\" --cfg=__rust_toolchain=\"1\" -Clinker=/usr/local/google/home/richkadel/fuchsia/prebuilt/third_party/clang/linux-x64/bin/clang++ -Cdefault-linker-libraries -Clink-arg=-static-libstdc++ -Clink-arg=-Wl,-rpath=\$ORIGIN/ -Clink-arg=--sysroot=../../prebuilt/third_party/sysroot/linux -Clink-arg=--target=x86_64-unknown-linux-gnu -Clink-arg=-stdlib=libc++ -Clink-arg=-unwindlib= -Clink-arg=-rtlib=compiler-rt -Clink-arg=-fuse-ld=lld -Clink-arg=-Wl,--build-id --remap-path-prefix /usr/local/google/home/richkadel/fuchsia/=../../ -Copt-level=1 -Cdebuginfo=2 --edition=2018 -Zallow-features= --target x86_64-unknown-linux-gnu --cap-lints=deny -Wrust-2018-idioms -Dwarnings -Cdebug-assertions=yes -Clinker=/usr/local/google/home/richkadel/fuchsia/prebuilt/third_party/clang/linux-x64/bin/clang++ -Cdefault-linker-libraries -Clink-arg=-static-libstdc++ -Clink-arg=-Wl,-rpath=\$ORIGIN/ -Clink-arg=--sysroot=../../prebuilt/third_party/sysroot/linux -Clink-arg=--target=x86_64-unknown-linux-gnu -Clink-arg=-stdlib=libc++ -Clink-arg=-unwindlib= -Clink-arg=-rtlib=compiler-rt -Clink-arg=-fuse-ld=lld -Clink-arg=-Wl,--build-id -Zinstrument-coverage -Dunused_results -Aunused_results -o host_x64-coverage-rust/obj/src/lib/network/packet/libpacket.rlib -Ldependency=host_x64-coverage-rust/obj/garnet/lib/rust/never -Ldependency=host_x64-coverage-rust/obj/src/lib/zerocopy -Ldependency=host_x64-coverage-rust -Ldependency=host_x64-coverage-rust/obj/third_party/rust_crates -ldl -lpthread --extern never=host_x64-coverage-rust/obj/garnet/lib/rust/never/libnever.rlib --extern zerocopy=host_x64-coverage-rust/obj/src/lib/zerocopy/libzerocopy.rlib -Zverbose
error: internal compiler error: compiler/rustc_middle/src/ty/fold.rs:908:21: Not enough bound vars: BoundRegion { var: 0, kind: BrNamed(DefId(0:152 ~ packet[8787]::records::{impl#2}::'a), 'a) } not found in []

thread 'rustc' panicked at 'Box<Any>', /usr/local/google/home/richkadel/rust/library/std/src/panic.rs:59:5
stack backtrace:
   0:     0x7f60d2647440 - std::backtrace_rs::backtrace::libunwind::trace::ha7d16337b841d883
                               at /usr/local/google/home/richkadel/rust/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f60d2647440 - std::backtrace_rs::backtrace::trace_unsynchronized::h4d2aea7bfbfd56ca
                               at /usr/local/google/home/richkadel/rust/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f60d2647440 - std::sys_common::backtrace::_print_fmt::hd0fdedefad5751f4
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f60d2647440 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h70f980369b2ee381
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f60d26c251d - core::fmt::write::h1f76cfa4421eede5
                               at /usr/local/google/home/richkadel/rust/library/core/src/fmt/mod.rs:1094:17
   5:     0x7f60d263b405 - std::io::Write::write_fmt::hecda97db4f848d47
                               at /usr/local/google/home/richkadel/rust/library/std/src/io/mod.rs:1584:15
   6:     0x7f60d264b807 - std::sys_common::backtrace::_print::h9315faaa925010fd
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f60d264b807 - std::sys_common::backtrace::print::h79eeed0d43fae328
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f60d264b807 - std::panicking::default_hook::{{closure}}::hca2be370ec3225a7
                               at /usr/local/google/home/richkadel/rust/library/std/src/panicking.rs:208:50
   9:     0x7f60d264b21b - std::panicking::default_hook::hd07246c4e82d503a
                               at /usr/local/google/home/richkadel/rust/library/std/src/panicking.rs:225:9
  10:     0x7f60d3a377c9 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h3d32ebf2b803a351
                               at /usr/local/google/home/richkadel/rust/library/alloc/src/boxed.rs:1560:9
  11:     0x7f60d3a377c9 - rustc_driver::report_ice::h511fc520263bc3d7
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_driver/src/lib.rs:1195:5
  12:     0x7f60d264c0ba - std::panicking::rust_panic_with_hook::h29481d192417bfde
                               at /usr/local/google/home/richkadel/rust/library/std/src/panicking.rs:595:17
  13:     0x7f60d6770310 - std::panicking::begin_panic::{{closure}}::h9a4f0968f111b4a2
                               at /usr/local/google/home/richkadel/rust/library/std/src/panicking.rs:520:9
  14:     0x7f60d67700f9 - std::sys_common::backtrace::__rust_end_short_backtrace::h44d8ac353a76c407
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys_common/backtrace.rs:141:18
  15:     0x7f60d67702b2 - std::panicking::begin_panic::h6b057b5538e3ec73
                               at /usr/local/google/home/richkadel/rust/library/std/src/panicking.rs:519:12
  16:     0x7f60d6729f00 - std::panic::panic_any::hfe7d929255d2862c
                               at /usr/local/google/home/richkadel/rust/library/std/src/panic.rs:59:5
  17:     0x7f60d6731288 - rustc_errors::HandlerInner::bug::hb1f77d097a1f18cd
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_errors/src/lib.rs:1002:9
  18:     0x7f60d672f0e3 - rustc_errors::Handler::bug::h94fa394c847f5441
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_errors/src/lib.rs:712:9
  19:     0x7f60d636daae - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::h4f6373782aa105fe
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/util/bug.rs:34:34
  20:     0x7f60d636b15b - rustc_middle::ty::context::tls::with_opt::{{closure}}::h1cc20541691940e4
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1798:40
  21:     0x7f60d636b10a - rustc_middle::ty::context::tls::with_context_opt::h1558e1d5bf07588b
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1750:22
  22:     0x7f60d636b10a - rustc_middle::ty::context::tls::with_opt::h6ee045d54e5cd5bf
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1798:9
  23:     0x7f60d636d993 - rustc_middle::util::bug::opt_span_bug_fmt::h030a3ab043659085
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/util/bug.rs:30:5
  24:     0x7f60d636d905 - rustc_middle::util::bug::bug_fmt::h2814b13ab9fc3450
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/util/bug.rs:14:5
  25:     0x7f60d657a959 - <rustc_middle::ty::fold::ValidateBoundVars as rustc_middle::ty::fold::TypeVisitor>::visit_region::hdf285631d8adbe60
  26:     0x7f60d44516cc - rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::sty::RegionKind>::visit_with::h2b2a802667268da9
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/structural_impls.rs:965:9
  27:     0x7f60d44516cc - <rustc_middle::ty::subst::GenericArg as rustc_middle::ty::fold::TypeFoldable>::super_visit_with::h21bee171c94022df
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/subst.rs:166:45
  28:     0x7f60d44516cc - rustc_middle::ty::fold::TypeFoldable::visit_with::h53bbbd73c98642b1
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/fold.rs:56:9
  29:     0x7f60d44516cc - rustc_middle::ty::subst::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>::super_visit_with::{{closure}}::hd624616ea669f518
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/subst.rs:397:38
  30:     0x7f60d44516cc - core::iter::traits::iterator::Iterator::try_for_each::call::{{closure}}::h1bc2c610fb90512c
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/traits/iterator.rs:2048:26
  31:     0x7f60d44516cc - core::iter::adapters::copied::copy_try_fold::{{closure}}::hfa94afeedf766351
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/adapters/copied.rs:30:22
  32:     0x7f60d44516cc - core::iter::traits::iterator::Iterator::try_fold::h5cb68b11178d73e7
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/traits/iterator.rs:2006:21
  33:     0x7f60d44516cc - <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold::hf5ecb6e2b2495612
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/adapters/copied.rs:55:9
  34:     0x7f60d44516cc - core::iter::traits::iterator::Iterator::try_for_each::hb137fc6227363da2
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/traits/iterator.rs:2051:9
  35:     0x7f60d44516cc - rustc_middle::ty::subst::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>::super_visit_with::h42a36fae64d2a9e6
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/subst.rs:397:9
  36:     0x7f60d44516cc - rustc_middle::ty::fold::TypeFoldable::visit_with::h7a7aefa41bde5683
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/fold.rs:56:9
  37:     0x7f60d431d578 - rustc_middle::ty::sty::_DERIVE_rustc_middle_ty_fold_TypeFoldable_tcx_FOR_ExistentialTraitRef::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::ty::sty::ExistentialTraitRef>::super_visit_with::h07d5343cc8430ef6
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/sty.rs:892:22
  38:     0x7f60d431d578 - rustc_middle::ty::fold::TypeFoldable::visit_with::h746a3c0dc5cfb624
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/fold.rs:56:9
  39:     0x7f60d431d578 - rustc_middle::ty::sty::Binder<T>::rebind::hb123503ac0e35458
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/sty.rs:1060:13
  40:     0x7f60d4361f73 - rustc_infer::traits::util::transitive_bounds_that_define_assoc_type::{{closure}}::h6430e39167462804
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_infer/src/traits/util.rs:310:49
  41:     0x7f60d4361f73 - <core::iter::sources::from_fn::FromFn<F> as core::iter::traits::iterator::Iterator>::next::h60e3c3b5c13f07bb
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/sources/from_fn.rs:69:9
  42:     0x7f60d4361f73 - core::iter::traits::iterator::Iterator::try_fold::h03497f9120f96898
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/traits/iterator.rs:2005:29
  43:     0x7f60d4361f73 - core::iter::traits::iterator::Iterator::find::hb456f465bc242544
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/traits/iterator.rs:2359:9
  44:     0x7f60d4361f73 - <core::iter::adapters::filter::Filter<I,P> as core::iter::traits::iterator::Iterator>::next::hf600ef31161b1e58
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/adapters/filter.rs:56:9
  45:     0x7f60d4386aa5 - <dyn rustc_typeck::astconv::AstConv>::one_bound_for_assoc_type::hd360a542fa242546
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:1526:27
  46:     0x7f60d4386aa5 - <dyn rustc_typeck::astconv::AstConv>::find_bound_for_assoc_item::hed341c5471a68545
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:1493:9
  47:     0x7f60d4386aa5 - <dyn rustc_typeck::astconv::AstConv>::associated_path_to_ty::hf05f3d9d5835cf27
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:1697:18
  48:     0x7f60d43a2e76 - <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty_inner::h378528f57fef9fe7
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:2259:17
  49:     0x7f60d44d7421 - <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty::h4556266895aeafdb
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:2194:9
  50:     0x7f60d44d7421 - <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty_inner::{{closure}}::h1124c0f49f3ec8a3
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:2216:50
  51:     0x7f60d44d7421 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once::h4fa96da45b1a4807
                               at /usr/local/google/home/richkadel/rust/library/core/src/ops/function.rs:280:13
  52:     0x7f60d44d7421 - core::option::Option<T>::map::h4ffa931333e496bc
                               at /usr/local/google/home/richkadel/rust/library/core/src/option.rs:487:29
  53:     0x7f60d44d7421 - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next::hb9cd00419a5e76de
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/adapters/map.rs:101:9
  54:     0x7f60d44d7421 - <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend::h3c3b75cfdc3ba595
                               at /usr/local/google/home/richkadel/.cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-1.6.1/src/lib.rs:1663:36
  55:     0x7f60d4416838 - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::h301b41f16b5ecf55
                               at /usr/local/google/home/richkadel/.cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-1.6.1/src/lib.rs:1648:9
  56:     0x7f60d4416838 - core::iter::traits::iterator::Iterator::collect::h7f5f64883f04a57c
                               at /usr/local/google/home/richkadel/rust/library/core/src/iter/traits/iterator.rs:1788:9
  57:     0x7f60d4416838 - <T as rustc_middle::ty::context::InternIteratorElement<T,R>>::intern_with::ha6704b8db0cf6908
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:2732:12
  58:     0x7f60d43a2f67 - <I as rustc_middle::ty::context::InternAs<[T],R>>::intern_with::hd38833e9845ace6c
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:2720:9
  59:     0x7f60d43a2f67 - rustc_middle::ty::context::TyCtxt::mk_tup::h480c3f3ce9a21855
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:2316:9
  60:     0x7f60d43a2f67 - <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty_inner::h378528f57fef9fe7
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:2216:17
  61:     0x7f60d439b4de - <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty::h4556266895aeafdb
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:2194:9
  62:     0x7f60d439b4de - <<dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_path::SubstsForAstPathCtxt as rustc_typeck::astconv::CreateSubstsForGenericArgsCtxt>::provided_kind::hb6ce1ad0f99fd6d1
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:449:29
  63:     0x7f60d439842c - rustc_typeck::astconv::generics::<impl dyn rustc_typeck::astconv::AstConv>::create_substs_for_generic_args::h5ee328ebc9fcceed
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/generics.rs:254:45
  64:     0x7f60d439842c - <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_path::h918341c86ee3b936
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:542:22
  65:     0x7f60d437dbb7 - <dyn rustc_typeck::astconv::AstConv>::ast_path_substs_for_ty::h3b478f79040bc48a
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:275:27
  66:     0x7f60d437f191 - <dyn rustc_typeck::astconv::AstConv>::ast_path_to_ty::h739dc0bedd8f693e
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:1175:22
  67:     0x7f60d438e22d - <dyn rustc_typeck::astconv::AstConv>::res_to_ty::hca3c40dae08ddbbb
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:2099:17
  68:     0x7f60d43a33ac - <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty_inner::h378528f57fef9fe7
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:2237:17
  69:     0x7f60d43915d2 - <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty::h4556266895aeafdb
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:2194:9
  70:     0x7f60d43915d2 - <dyn rustc_typeck::astconv::AstConv>::ty_of_fn::hf4fe81931e6f4c88
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/astconv/mod.rs:2388:17
  71:     0x7f60d42d2452 - rustc_typeck::collect::fn_sig::hce8662bffd3d754c
  72:     0x7f60d4997e0c - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::fn_sig>::compute::h8af2179c2d309048
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:399:17
  73:     0x7f60d4d07795 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h9980a8276a673923
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:270:14
  74:     0x7f60d4a58cd6 - rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}::h9d95118e7a7d7098
  75:     0x7f60d4a58cd6 - stacker::maybe_grow::hc39693e08a47ca91
                               at /usr/local/google/home/richkadel/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  76:     0x7f60d4a58cd6 - rustc_data_structures::stack::ensure_sufficient_stack::h35825313933b088f
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_data_structures/src/stack.rs:16:5
  77:     0x7f60d48d2491 - <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::{{closure}}::h0c94dd2a84d59c7c
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:169:17
  78:     0x7f60d48d2491 - rustc_middle::ty::context::tls::enter_context::{{closure}}::h549ec47d0d841df1
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1733:50
  79:     0x7f60d48d2491 - rustc_middle::ty::context::tls::set_tlv::h1bc7607a91d769f0
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1717:9
  80:     0x7f60d48d2491 - rustc_middle::ty::context::tls::enter_context::h06fea3ac8b2c5bb1
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1733:9
  81:     0x7f60d48d2491 - <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::h2f6e80809d0b311c
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:168:13
  82:     0x7f60d48d2491 - rustc_middle::ty::context::tls::with_related_context::{{closure}}::h59f6110b9dd23148
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1777:13
  83:     0x7f60d48d2491 - rustc_middle::ty::context::tls::with_context::{{closure}}::h6e2f467d00f247b7
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1761:40
  84:     0x7f60d48d2491 - rustc_middle::ty::context::tls::with_context_opt::hbe78895e7ac8d6b3
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1750:22
  85:     0x7f60d48d2491 - rustc_middle::ty::context::tls::with_context::hfb99cb91997817be
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1761:9
  86:     0x7f60d48d2491 - rustc_middle::ty::context::tls::with_related_context::h9afd6c12534fa9dc
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1774:9
  87:     0x7f60d48d2491 - <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::h4a8c10f5dbdb0289
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:157:9
  88:     0x7f60d48d2491 - rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::h29fc3c20a4a07d71
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:630:9
  89:     0x7f60d48d2491 - rustc_query_system::query::plumbing::with_diagnostics::h4c66b848fc8814ee
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:328:18
  90:     0x7f60d48d2491 - rustc_query_system::query::plumbing::force_query_with_job::hc8e9ca13cfae4f33
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:629:51
  91:     0x7f60d4809e60 - rustc_query_system::query::plumbing::try_execute_query::hb896859c91b3dd76
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:444:16
  92:     0x7f60d4809e60 - rustc_query_system::query::plumbing::get_query_impl::h35769e09fb23d32a
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:677:5
  93:     0x7f60d497e0c1 - rustc_query_system::query::plumbing::get_query::h98cf3476b2c515b1
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:788:9
  94:     0x7f60d4c715f8 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::fn_sig::h13d2b845c15e1b82
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:603:17
  95:     0x7f60d42c46d2 - rustc_middle::ty::query::TyCtxtEnsure::fn_sig::h6810107713e4d834
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/query/mod.rs:176:17
  96:     0x7f60d42c46d2 - rustc_typeck::collect::convert_impl_item::hcd651bb4d8747521
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/collect.rs:870:13
  97:     0x7f60d42c46d2 - <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_impl_item::hdd2745ed159ce0b6
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/collect.rs:289:9
  98:     0x7f60d452c263 - <rustc_hir::intravisit::DeepVisitor<V> as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_impl_item::h6d9844a09b349df4
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_hir/src/intravisit.rs:65:9
  99:     0x7f60d452c263 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h49c8fefff290af28
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/hir/map/mod.rs:482:13
 100:     0x7f60d42c12c4 - rustc_typeck::collect::collect_mod_item_types::hd2eaffa2327e68e6
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/collect.rs:65:5
 101:     0x7f60d4d177f8 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hd05668e770f9759b
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:270:14
 102:     0x7f60d4a73566 - rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}::h15ac21c60f41ac5f
 103:     0x7f60d4a73566 - stacker::maybe_grow::hf0cfe263858debf4
                               at /usr/local/google/home/richkadel/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
 104:     0x7f60d4a73566 - rustc_data_structures::stack::ensure_sufficient_stack::hc8a0102826b2b6e6
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_data_structures/src/stack.rs:16:5
 105:     0x7f60d48aebb4 - <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::{{closure}}::ha815d19a0343d396
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:169:17
 106:     0x7f60d48aebb4 - rustc_middle::ty::context::tls::enter_context::{{closure}}::h597bfa4c2be73d5b
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1733:50
 107:     0x7f60d48aebb4 - rustc_middle::ty::context::tls::set_tlv::h2ce35bf390aeeba4
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1717:9
 108:     0x7f60d48aebb4 - rustc_middle::ty::context::tls::enter_context::h7f89f3c18eb64402
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1733:9
 109:     0x7f60d48aebb4 - <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::h6a8e7823acaa3b96
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:168:13
 110:     0x7f60d48aebb4 - rustc_middle::ty::context::tls::with_related_context::{{closure}}::h28bff3ce409d522d
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1777:13
 111:     0x7f60d48aebb4 - rustc_middle::ty::context::tls::with_context::{{closure}}::h9e30347a7efbf34d
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1761:40
 112:     0x7f60d48aebb4 - rustc_middle::ty::context::tls::with_context_opt::h1e7bbd02b39ae2fe
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1750:22
 113:     0x7f60d48aebb4 - rustc_middle::ty::context::tls::with_context::h8e23c58673236f97
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1761:9
 114:     0x7f60d48aebb4 - rustc_middle::ty::context::tls::with_related_context::ha96c785ff1c4abdb
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1774:9
 115:     0x7f60d48aebb4 - <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::hefc33132bab328b3
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:157:9
 116:     0x7f60d48aebb4 - rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::h41e6011214b1bb22
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:630:9
 117:     0x7f60d48aebb4 - rustc_query_system::query::plumbing::with_diagnostics::h049c2f7f5f53c38a
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:328:18
 118:     0x7f60d48aebb4 - rustc_query_system::query::plumbing::force_query_with_job::h35e60d19d9256018
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:629:51
 119:     0x7f60d47f7c46 - rustc_query_system::query::plumbing::try_execute_query::ha817a496779ba8c8
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:444:16
 120:     0x7f60d47f7c46 - rustc_query_system::query::plumbing::get_query_impl::h1bb08d1c185efffd
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:677:5
 121:     0x7f60d496f740 - rustc_query_system::query::plumbing::get_query::h45de16e7de7e96ef
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:788:9
 122:     0x7f60d433d366 - rustc_middle::ty::query::TyCtxtEnsure::collect_mod_item_types::ha72e1ce32b1b1dcd
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/query/mod.rs:176:17
 123:     0x7f60d433d366 - rustc_typeck::check_crate::{{closure}}::{{closure}}::h30f122e39624e495
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/lib.rs:380:17
 124:     0x7f60d433d366 - rustc_data_structures::profiling::VerboseTimingGuard::run::hc7e23588500ed94f
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_data_structures/src/profiling.rs:573:9
 125:     0x7f60d433d366 - rustc_session::utils::<impl rustc_session::session::Session>::time::h105d84d36d33b8d8
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_session/src/utils.rs:16:9
 126:     0x7f60d433d366 - rustc_typeck::check_crate::{{closure}}::hb52ed5f9b822dbb6
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/lib.rs:378:9
 127:     0x7f60d433d366 - rustc_session::session::Session::track_errors::h0dc2398540d63411
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_session/src/session.rs:491:22
 128:     0x7f60d4395d56 - rustc_typeck::check_crate::h17f4f7e251621d95
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_typeck/src/lib.rs:377:5
 129:     0x7f60d3bc9847 - rustc_interface::passes::analysis::h8a94ea56dab9c8d6
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_interface/src/passes.rs:855:5
 130:     0x7f60d4d1b82d - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hdb213566a8ab92cf
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:270:14
 131:     0x7f60d4a58362 - rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}::h929a0f71326d4c22
 132:     0x7f60d4a58362 - stacker::maybe_grow::hdc508a3853c05f24
                               at /usr/local/google/home/richkadel/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
 133:     0x7f60d4a58362 - rustc_data_structures::stack::ensure_sufficient_stack::h33f1b474831fb7bf
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_data_structures/src/stack.rs:16:5
 134:     0x7f60d48d1004 - <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::{{closure}}::h1748912ed4a4aafc
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:169:17
 135:     0x7f60d48d1004 - rustc_middle::ty::context::tls::enter_context::{{closure}}::ha71806e950bdda07
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1733:50
 136:     0x7f60d48d1004 - rustc_middle::ty::context::tls::set_tlv::h854e174759e9ad8b
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1717:9
 137:     0x7f60d48d1004 - rustc_middle::ty::context::tls::enter_context::hc8774be7d7a995d1
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1733:9
 138:     0x7f60d48d1004 - <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::h04199122d6d38f1c
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:168:13
 139:     0x7f60d48d1004 - rustc_middle::ty::context::tls::with_related_context::{{closure}}::h1927ef2f53cbcd12
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1777:13
 140:     0x7f60d48d1004 - rustc_middle::ty::context::tls::with_context::{{closure}}::h9ae394f1cdd1e85e
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1761:40
 141:     0x7f60d48d1004 - rustc_middle::ty::context::tls::with_context_opt::haa6402adc2bb8f64
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1750:22
 142:     0x7f60d48d1004 - rustc_middle::ty::context::tls::with_context::hc31fcc879d0a2c15
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1761:9
 143:     0x7f60d48d1004 - rustc_middle::ty::context::tls::with_related_context::h7603ab493bbdcf00
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1774:9
 144:     0x7f60d48d1004 - <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::hf533444fbf969e17
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_impl/src/plumbing.rs:157:9
 145:     0x7f60d48d1004 - rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::hb7a0b9b5af14c66e
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:630:9
 146:     0x7f60d48d1004 - rustc_query_system::query::plumbing::with_diagnostics::h72cd7f70233a89e8
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:328:18
 147:     0x7f60d48d1004 - rustc_query_system::query::plumbing::force_query_with_job::hc5cf02516798f0e1
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:629:51
 148:     0x7f60d4849ab6 - rustc_query_system::query::plumbing::try_execute_query::h0dda7fb8bb9ffaa1
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:444:16
 149:     0x7f60d4849ab6 - rustc_query_system::query::plumbing::get_query_impl::ha08fe804ce1c516c
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:677:5
 150:     0x7f60d4982483 - rustc_query_system::query::plumbing::get_query::hadf4a8c46bae7926
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_query_system/src/query/plumbing.rs:788:9
 151:     0x7f60d3a7bf63 - rustc_middle::ty::query::TyCtxtAt::analysis::hb3b85630582e8afd
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/query/mod.rs:205:17
 152:     0x7f60d3a7bf63 - rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::analysis::he3f6e7595ae7e3ea
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/query/mod.rs:186:17
 153:     0x7f60d3a7bf63 - rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}::h63c467390971eaf6
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_driver/src/lib.rs:436:59
 154:     0x7f60d3a7bf63 - rustc_interface::passes::QueryContext::enter::{{closure}}::h033e4c738ffa4bcc
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_interface/src/passes.rs:755:42
 155:     0x7f60d3a7bf63 - rustc_middle::ty::context::tls::enter_context::{{closure}}::hb3e293863411f1c2
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1733:50
 156:     0x7f60d3a7bf63 - rustc_middle::ty::context::tls::set_tlv::hf659fe967eb9b4e6
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1717:9
 157:     0x7f60d3a7bf63 - rustc_middle::ty::context::tls::enter_context::hb25a8c914fd7392f
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_middle/src/ty/context.rs:1733:9
 158:     0x7f60d3a7bf63 - rustc_interface::passes::QueryContext::enter::hc11207a09c091761
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_interface/src/passes.rs:755:9
 159:     0x7f60d3a57210 - rustc_driver::run_compiler::{{closure}}::{{closure}}::hff81029240a04778
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_driver/src/lib.rs:436:13
 160:     0x7f60d3a57210 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h8e5854f4ad6671e7
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_interface/src/queries.rs:428:19
 161:     0x7f60d3a39efd - rustc_driver::run_compiler::{{closure}}::hb30ccf7474ac0543
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_driver/src/lib.rs:337:22
 162:     0x7f60d3a39efd - rustc_interface::interface::create_compiler_and_run::{{closure}}::hf4d1c920c3e93838
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_interface/src/interface.rs:208:13
 163:     0x7f60d3a39efd - rustc_span::with_source_map::hec83cfa370c9a229
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_span/src/lib.rs:787:5
 164:     0x7f60d3a54edf - rustc_interface::interface::create_compiler_and_run::haec604fe593a604b
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_interface/src/interface.rs:202:5
 165:     0x7f60d3a54edf - rustc_interface::interface::run_compiler::{{closure}}::hb1fd180a025deade
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_interface/src/interface.rs:224:12
 166:     0x7f60d3a54edf - rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}::hd26db5b50e9b4b2f
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_interface/src/util.rs:155:13
 167:     0x7f60d3a54edf - scoped_tls::ScopedKey<T>::set::h9444b4584de1ca93
                               at /usr/local/google/home/richkadel/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 168:     0x7f60d3a7e234 - rustc_span::with_session_globals::hc7589ead2f7fc07a
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_span/src/lib.rs:104:5
 169:     0x7f60d3a7e234 - rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::h9e442ed5cbac28e5
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_interface/src/util.rs:153:9
 170:     0x7f60d3a7e234 - rustc_interface::util::scoped_thread::{{closure}}::h2960cadce0dc8cb6
                               at /usr/local/google/home/richkadel/rust/compiler/rustc_interface/src/util.rs:128:24
 171:     0x7f60d3a7e234 - std::sys_common::backtrace::__rust_begin_short_backtrace::h72c1c5fa5925ef11
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys_common/backtrace.rs:125:18
 172:     0x7f60d3a7f503 - std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}}::h0b708390368ebfd8
                               at /usr/local/google/home/richkadel/rust/library/std/src/thread/mod.rs:481:17
 173:     0x7f60d3a7f503 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h09e4360883d16c3c
                               at /usr/local/google/home/richkadel/rust/library/std/src/panic.rs:344:9
 174:     0x7f60d3a7f503 - std::panicking::try::do_call::he6f88e59498a280b
                               at /usr/local/google/home/richkadel/rust/library/std/src/panicking.rs:379:40
 175:     0x7f60d3a7f503 - std::panicking::try::h9e14e8ba5a666d0a
                               at /usr/local/google/home/richkadel/rust/library/std/src/panicking.rs:343:19
 176:     0x7f60d3a7f503 - std::panic::catch_unwind::h6a361875e31b7f28
                               at /usr/local/google/home/richkadel/rust/library/std/src/panic.rs:431:14
 177:     0x7f60d3a7f503 - std::thread::Builder::spawn_unchecked::{{closure}}::ha8293a07af268859
                               at /usr/local/google/home/richkadel/rust/library/std/src/thread/mod.rs:480:30
 178:     0x7f60d3a7f503 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h15c963a7de595862
                               at /usr/local/google/home/richkadel/rust/library/core/src/ops/function.rs:227:5
 179:     0x7f60d265ba37 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfb99c185e4170474
                               at /usr/local/google/home/richkadel/rust/library/alloc/src/boxed.rs:1546:9
 180:     0x7f60d265ba37 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h05dcbc64b0320eca
                               at /usr/local/google/home/richkadel/rust/library/alloc/src/boxed.rs:1546:9
 181:     0x7f60d265ba37 - std::sys::unix::thread::Thread::new::thread_start::hb6823714e9e5abdb
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys/unix/thread.rs:71:17
 182:     0x7f60ce89fea7 - start_thread
 183:     0x7f60d245ddef - clone

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (8e4e952a6 2021-04-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z dep-info-omit-d-target -Z allow-features= -Z instrument-coverage -Z verbose -C linker=/usr/local/google/home/richkadel/fuchsia/prebuilt/third_party/clang/linux-x64/bin/clang++ -C default-linker-libraries -C link-arg=-static-libstdc++ -C link-arg=-Wl,-rpath=$ORIGIN/ -C link-arg=--sysroot=../../prebuilt/third_party/sysroot/linux -C link-arg=--target=x86_64-unknown-linux-gnu -C link-arg=-stdlib=libc++ -C link-arg=-unwindlib= -C link-arg=-rtlib=compiler-rt -C link-arg=-fuse-ld=lld -C link-arg=-Wl,--build-id -C opt-level=1 -C debuginfo=2 -C debug-assertions=yes -C linker=/usr/local/google/home/richkadel/fuchsia/prebuilt/third_party/clang/linux-x64/bin/clang++ -C default-linker-libraries -C link-arg=-static-libstdc++ -C link-arg=-Wl,-rpath=$ORIGIN/ -C link-arg=--sysroot=../../prebuilt/third_party/sysroot/linux -C link-arg=--target=x86_64-unknown-linux-gnu -C link-arg=-stdlib=libc++ -C link-arg=-unwindlib= -C link-arg=-rtlib=compiler-rt -C link-arg=-fuse-ld=lld -C link-arg=-Wl,--build-id --crate-type rlib

query stack during panic:
#0 [fn_sig] computing function signature of `records::<impl at ../../src/lib/network/packet/src/records.rs:97:1: 167:2>::parse_raw_with_context` [fn_sig]
#1 [collect_mod_item_types] collecting item types in module `records` [collect_mod_item_types]
#2 [analysis] running analysis passes on this crate [analysis]
end of query stack
error: aborting due to previous error
