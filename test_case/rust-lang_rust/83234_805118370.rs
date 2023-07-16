diff
-1,090,522,105  ???:<rustc_mir::interpret::intern::InternVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_value
+1,040,191,105  ???:rustc_mir::interpret::visitor::ValueVisitor::walk_aggregate
-922,731,040  ???:rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id
-896,650,202  ???:<rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::query::TyCtxtAt> as rustc_target::abi::LayoutOf>::layout_of
-841,474,121  ???:rustc_middle::ty::instance::Instance::resolve_opt_const_arg
-645,164,731  ???:rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::try_read_immediate
-641,732,472  ???:rustc_middle::ty::instance::Instance::subst_mir_and_normalize_erasing_regions
-611,401,342  ???:rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra>::get_bytes_internal
+599,550,859  ???:rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra>::get_bytes_with_uninit_and_ptr
+559,944,830  ???:rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::const_val_to_op
-516,445,484  ???:<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
-482,345,438  ???:rustc_middle::ty::context::TyCtxt::intern_const_alloc
-425,723,340  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/jemalloc.c:malloc
-413,449,538  ???:<rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra> as core::hash::Hash>::hash
-408,944,640  ???:<rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_constant
-403,701,730  ???:<rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty
+398,458,584  ???:<rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_const
-386,928,936  ???:rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::const_to_op
-386,255,464  ???:hashbrown::map::HashMap<K,V,S,A>::insert
-358,615,160  ???:<rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra> as core::cmp::PartialEq>::eq
-335,541,174  ???:rustc_middle::ty::ParamEnv::with_reveal_all_normalized
-303,556,980  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/jemalloc.c:free
-301,991,602  ???:rustc_middle::mir::interpret::allocation::Allocation<Tag>::from_byte_aligned_bytes
-288,353,584  ???:rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve
-277,852,288  ???:<rustc_middle::ty::consts::kind::ConstKind as core::cmp::PartialEq>::ne
-267,901,547  ???:hashbrown::raw::RawTable<T,A>::reserve_rehash
-259,271,610  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/rtree.h:free
-251,660,124  ???:rustc_middle::mir::interpret::allocation::InitMask::grow
-245,371,888  ???:rustc_middle::ty::flags::FlagComputation::for_const
-212,879,306  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/cache_bin.h:malloc
-212,861,621  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/tcache_inlines.h:free
-209,711,487  ???:<rustc_middle::ty::instance::InstanceDef as core::cmp::PartialEq>::eq
-204,505,007  ???:alloc::raw_vec::RawVec<T,A>::reserve
-204,473,478  ???:rustc_middle::ty::context::TyCtxt::allocate_bytes
-173,015,931  ???:rustc_middle::mir::interpret::allocation::InitMask::set_range_inbounds
+166,724,952  ???:rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand
-152,043,520  ???:rustc_middle::ty::consts::valtree::ValTree::unwrap_leaf
+151,468,639  ???:rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
-146,798,099  ???:rustc_middle::ty::fold::TypeFoldable::visit_with
-146,789,664  ???:<rustc_middle::ty::instance::InstanceDef as core::hash::Hash>::hash
